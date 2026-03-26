use sqlx::SqlitePool;

use crate::errors::AppError;

// ── Decryption constants (must match license issuer) ──

const LICENSE_VERSION: u8 = 2;
const RAW_LEN_BYTES: usize = 48;
const NONCE_LEN: usize = 12;
const PAYLOAD_LEN: usize = 25;
const TAG_LEN: usize = 11;

/// Embedded AES-256-GCM key (base64-standard, private).
const EMBEDDED_SECRET_B64: &str = "9xTybrVcLKaTujeTK8A9Kr3EWFz1zD8EzTqcFMR3+CI=";

// ── Domain struct ──

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct License {
    pub id: String,
    pub token: String,
    pub project_name: String,
    pub expires_at: String,
    pub status: String,
    pub activated_at: String,
    pub created_at: String,
}

// ── Decoded payload ──

#[derive(Debug)]
pub struct DecodedLicense {
    pub project_name: String,
    pub expires_at: String, // ISO-8601 UTC
}

// ── Decryption ──

fn load_secret() -> Result<[u8; 32], AppError> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    let raw = STANDARD
        .decode(EMBEDDED_SECRET_B64.trim())
        .map_err(|e| AppError::Internal(format!("license secret decode: {e}")))?;
    if raw.len() < 16 {
        return Err(AppError::Internal("license secret too short".into()));
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&raw[..32]);
    Ok(key)
}

fn b64url_decode(token: &str) -> Result<Vec<u8>, AppError> {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    // Accept both padded and unpadded variants.
    let s = token.trim_end_matches('=');
    URL_SAFE_NO_PAD
        .decode(s)
        .map_err(|_| AppError::BadRequest("invalid license token format".into()))
}

pub fn decrypt(token: &str) -> Result<DecodedLicense, AppError> {
    use aes_gcm::{
        aead::{Aead, KeyInit, Payload as AeadPayload},
        Aes256Gcm, Nonce,
    };

    let token = token.trim();
    let raw = b64url_decode(token)?;
    if raw.len() != RAW_LEN_BYTES {
        return Err(AppError::BadRequest(format!(
            "unsupported token length (expected {RAW_LEN_BYTES} bytes decoded, got {})",
            raw.len()
        )));
    }

    let key_bytes = load_secret()?;
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .map_err(|e| AppError::Internal(format!("cipher init: {e}")))?;

    let nonce = Nonce::from_slice(&raw[..NONCE_LEN]);

    // aes-gcm default tag length is 16 bytes; our issuer uses 11 bytes.
    // Use `AeadInPlace` with a full-length tag by zero-padding the tag to 16b.
    // We instead decrypt manually: reconstruct full 16-byte tag slot.
    let ct = &raw[NONCE_LEN..NONCE_LEN + PAYLOAD_LEN];
    let tag_short = &raw[NONCE_LEN + PAYLOAD_LEN..];
    if tag_short.len() != TAG_LEN {
        return Err(AppError::BadRequest("invalid token tag length".into()));
    }
    // Pad tag to 16 bytes (aes-gcm crate requires full 128-bit tag).
    let mut tag16 = [0u8; 16];
    tag16[..TAG_LEN].copy_from_slice(tag_short);

    let mut buf = ct.to_vec();
    buf.extend_from_slice(&tag16);

    let payload = cipher
        .decrypt(nonce, AeadPayload { msg: &buf, aad: b"" })
        .map_err(|_| AppError::BadRequest("license verification failed (invalid or tampered token)".into()))?;

    if payload.len() != PAYLOAD_LEN {
        return Err(AppError::BadRequest("invalid license payload".into()));
    }

    // Parse payload: 1 byte version + 4 bytes u32 big-endian timestamp + 20 bytes UTF-16LE name
    let version = payload[0];
    if version != LICENSE_VERSION {
        return Err(AppError::BadRequest(format!(
            "unsupported license version {version}"
        )));
    }

    let expires_ts = u32::from_be_bytes([payload[1], payload[2], payload[3], payload[4]]);
    let name_bytes = &payload[5..25];

    // Strip trailing null pairs (UTF-16LE padding)
    let mut end = name_bytes.len();
    while end >= 2 && name_bytes[end - 2] == 0 && name_bytes[end - 1] == 0 {
        end -= 2;
    }
    let project_name = if end > 0 {
        let u16_vals: Vec<u16> = name_bytes[..end]
            .chunks(2)
            .map(|c| u16::from_le_bytes([c[0], c.get(1).copied().unwrap_or(0)]))
            .collect();
        String::from_utf16(&u16_vals)
            .map_err(|_| AppError::BadRequest("invalid licensee name encoding".into()))?
    } else {
        String::new()
    };

    use std::time::{Duration, UNIX_EPOCH};
    let expires_at = UNIX_EPOCH + Duration::from_secs(expires_ts as u64);
    let expires_secs = expires_at
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Format as ISO-8601 UTC using manual calculation (no chrono dep needed here)
    let expires_iso = unix_to_iso8601(expires_secs);

    Ok(DecodedLicense {
        project_name,
        expires_at: expires_iso,
    })
}

fn unix_to_iso8601(secs: u64) -> String {
    use chrono::{TimeZone, Utc};
    Utc.timestamp_opt(secs as i64, 0)
        .single()
        .unwrap_or_default()
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string()
}

/// Returns true if the ISO-8601 UTC expires_at is still in the future.
pub fn is_valid(expires_at: &str) -> bool {
    use chrono::{DateTime, Utc};
    expires_at
        .parse::<DateTime<Utc>>()
        .map(|dt| dt > Utc::now())
        .unwrap_or(false)
}

// ── DB queries ──

/// Return the currently active license (latest activated, status='active').
pub async fn find_active(pool: &SqlitePool) -> Result<Option<License>, AppError> {
    Ok(sqlx::query_as::<_, License>(
        "SELECT * FROM licenses WHERE status = 'active' ORDER BY activated_at DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?)
}

/// Mark all current 'active' licenses as 'replaced', then insert new one.
pub async fn activate(
    pool: &SqlitePool,
    token: &str,
    project_name: &str,
    expires_at: &str,
) -> Result<License, AppError> {
    // Replace existing active license.
    sqlx::query("UPDATE licenses SET status = 'replaced' WHERE status = 'active'")
        .execute(pool)
        .await?;

    let id = uuid::Uuid::new_v4().to_string();
    let row = sqlx::query_as::<_, License>(
        r#"
        INSERT INTO licenses (id, token, project_name, expires_at)
        VALUES (?1, ?2, ?3, ?4)
        RETURNING *
        "#,
    )
    .bind(&id)
    .bind(token)
    .bind(project_name)
    .bind(expires_at)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::pagination::PaginationParams;

// ── Structs ──

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub ip: Option<String>,
    pub device: Option<String>,
    pub status: String,
    pub created_at: String,
    pub expires_at: String,
}

/// Result of a token validation JOIN with users table.
#[derive(Debug, sqlx::FromRow)]
pub struct ActiveTokenInfo {
    pub user_id: String,
    pub username: String,
    pub role: String,
    pub user_status: String,
    #[allow(dead_code)]
    pub token: String,
}

// ── DB queries ──

/// Insert a new active token (expires in 24 hours).
pub async fn create(
    pool: &SqlitePool,
    user_id: &str,
    token: &str,
    ip: Option<&str>,
    device: Option<&str>,
) -> Result<(), AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO tokens (id, user_id, token, ip, device, expires_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now', '+24 hours'))",
    )
    .bind(&id)
    .bind(user_id)
    .bind(token)
    .bind(ip)
    .bind(device)
    .execute(pool)
    .await?;
    Ok(())
}

/// Find a token row that is active and not expired, joined with its user.
pub async fn find_active(
    pool: &SqlitePool,
    token: &str,
) -> Result<Option<ActiveTokenInfo>, AppError> {
    let row = sqlx::query_as::<_, ActiveTokenInfo>(
        r#"
        SELECT t.token,
               u.id   AS user_id,
               u.username,
               u.role,
               u.status AS user_status
        FROM tokens t
        JOIN users u ON t.user_id = u.id
        WHERE t.token    = ?1
          AND t.status   = 'active'
          AND t.expires_at > datetime('now')
        "#,
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Mark a token as revoked.
pub async fn revoke(pool: &SqlitePool, token: &str) -> Result<(), AppError> {
    sqlx::query("UPDATE tokens SET status = 'revoked' WHERE token = ?1")
        .bind(token)
        .execute(pool)
        .await?;
    Ok(())
}

/// Extend expiry by 24 hours from now.
pub async fn refresh(pool: &SqlitePool, token: &str) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE tokens SET expires_at = datetime('now', '+24 hours') WHERE token = ?1",
    )
    .bind(token)
    .execute(pool)
    .await?;
    Ok(())
}

// ── Token management (admin) ──

/// Active token row joined with username, for admin listing.
#[derive(Debug, sqlx::FromRow)]
pub struct ActiveTokenRow {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub token: String,
    pub ip: Option<String>,
    pub device: Option<String>,
    pub status: String,
    pub created_at: String,
    pub expires_at: String,
}

/// Safe DTO — token is masked.
#[derive(Debug, serde::Serialize)]
pub struct TokenView {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
    #[serde(rename = "maskedToken")]
    pub masked_token: String,
    pub ip: Option<String>,
    pub device: Option<String>,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

impl From<ActiveTokenRow> for TokenView {
    fn from(r: ActiveTokenRow) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            username: r.username,
            masked_token: mask_token(&r.token),
            ip: r.ip,
            device: r.device,
            status: r.status,
            created_at: r.created_at,
            expires_at: r.expires_at,
        }
    }
}

fn mask_token(token: &str) -> String {
    let chars: Vec<char> = token.chars().collect();
    if chars.len() <= 12 {
        return "****".to_string();
    }
    let prefix: String = chars[..8].iter().collect();
    let suffix: String = chars[chars.len() - 4..].iter().collect();
    format!("{prefix}****{suffix}")
}

pub struct TokenListResult {
    pub items: Vec<ActiveTokenRow>,
    pub total: i64,
}

/// List active (status='active' AND not expired) tokens with pagination, joined with username.
pub async fn list_active(
    pool: &SqlitePool,
    pagination: &PaginationParams,
) -> Result<TokenListResult, AppError> {
    let offset = pagination.offset();
    let page_size = pagination.page_size();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM tokens WHERE status = 'active' AND expires_at > datetime('now')",
    )
    .fetch_one(pool)
    .await?;

    let items = sqlx::query_as::<_, ActiveTokenRow>(
        "SELECT t.id, t.user_id, u.username, t.token, t.ip, t.device, t.status, \
                t.created_at, t.expires_at \
         FROM tokens t JOIN users u ON t.user_id = u.id \
         WHERE t.status = 'active' AND t.expires_at > datetime('now') \
         ORDER BY t.created_at DESC LIMIT ?1 OFFSET ?2",
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(TokenListResult { items, total })
}

/// Revoke a token by its id. Returns an error if the token doesn't exist or is already revoked/expired.
pub async fn revoke_by_id(pool: &SqlitePool, id: &str) -> Result<(), AppError> {
    // Fetch current state first
    let row = sqlx::query_as::<_, Token>(
        "SELECT * FROM tokens WHERE id = ?1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("error.tokens.not_found".into()))?;

    if row.status != "active" {
        return Err(AppError::BadRequest("error.tokens.already_revoked".into()));
    }

    sqlx::query("UPDATE tokens SET status = 'revoked' WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// ── Helper: generate a 32-char alphanumeric token ──

pub fn generate() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect()
}

// ── Helper: extract client IP from request ──
// Trusts X-Forwarded-For / X-Real-IP only when the direct peer is a
// private network address (10/8, 172.16/12, 192.168/16, loopback).

pub fn extract_ip(req: &actix_web::HttpRequest) -> String {
    let peer = req.peer_addr().map(|a| a.ip());
    let from_private = peer.map(is_private_ip).unwrap_or(false);

    if from_private {
        if let Some(xff) = req
            .headers()
            .get("X-Forwarded-For")
            .and_then(|v| v.to_str().ok())
        {
            if let Some(ip) = xff.split(',').next().map(|s| s.trim().to_string()) {
                if !ip.is_empty() {
                    return ip;
                }
            }
        }
        if let Some(xri) = req
            .headers()
            .get("X-Real-IP")
            .and_then(|v| v.to_str().ok())
        {
            let ip = xri.trim().to_string();
            if !ip.is_empty() {
                return ip;
            }
        }
    }

    peer.map(|ip| ip.to_string()).unwrap_or_else(|| "unknown".into())
}

fn is_private_ip(ip: std::net::IpAddr) -> bool {
    match ip {
        std::net::IpAddr::V4(v4) => {
            let o = v4.octets();
            o[0] == 10
                || (o[0] == 172 && o[1] >= 16 && o[1] <= 31)
                || (o[0] == 192 && o[1] == 168)
                || o[0] == 127
        }
        std::net::IpAddr::V6(v6) => v6.is_loopback(),
    }
}

// ── Helper: parse User-Agent → "OS Browser" ──

pub fn parse_device(user_agent: &str) -> String {
    let ua = user_agent.to_lowercase();

    let os = if ua.contains("iphone") {
        "iPhone"
    } else if ua.contains("ipad") {
        "iPad"
    } else if ua.contains("android") {
        "Android"
    } else if ua.contains("macintosh") || ua.contains("mac os x") {
        "macOS"
    } else if ua.contains("windows") {
        "Windows"
    } else if ua.contains("linux") {
        "Linux"
    } else {
        "Unknown"
    };

    let browser = if ua.contains("edg/") || ua.contains("edge/") {
        "Edge"
    } else if ua.contains("firefox") {
        "Firefox"
    } else if ua.contains("chrome") {
        "Chrome"
    } else if ua.contains("safari") {
        "Safari"
    } else if ua.contains("curl") {
        "curl"
    } else {
        "Browser"
    };

    format!("{os} {browser}")
}

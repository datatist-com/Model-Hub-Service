use sqlx::SqlitePool;

use crate::errors::AppError;

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

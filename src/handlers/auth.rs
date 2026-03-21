use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::errors::{ApiResponse, AppError};
use crate::middleware::auth::Claims;
use crate::models::{token as token_model, user};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginData {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub user: UserInfo,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    #[serde(rename = "realName")]
    pub real_name: Option<String>,
    pub role: String,
    pub language: Option<String>,
    #[serde(rename = "uiTheme")]
    pub ui_theme: Option<String>,
}

/// POST /api/v1/auth/login
pub async fn login(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let body = body.into_inner();

    if body.username.is_empty() || body.password.is_empty() {
        return Err(AppError::BadRequest("Username and password required".into()));
    }

    let db_user = user::find_by_username(&pool, &body.username)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".into()))?;

    if db_user.status != "active" {
        return Err(AppError::Forbidden("Account is frozen".into()));
    }

    verify_password(&body.password, &db_user.password_hash)?;

    let ip = token_model::extract_ip(&req);
    let device = req
        .headers()
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(token_model::parse_device)
        .unwrap_or_else(|| "Unknown".into());

    let raw_token = token_model::generate();
    token_model::create(&pool, &db_user.id, &raw_token, Some(&ip), Some(&device)).await?;

    Ok(ApiResponse::ok(LoginData {
        access_token: raw_token,
        user: UserInfo {
            id: db_user.id,
            username: db_user.username,
            real_name: db_user.real_name,
            role: db_user.role,
            language: db_user.language,
            ui_theme: db_user.ui_theme,
        },
    }))
}

/// POST /api/v1/auth/logout — revoke the current token in DB.
pub async fn logout(
    pool: web::Data<SqlitePool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    token_model::revoke(&pool, &claims.token).await?;
    Ok(ApiResponse::ok(serde_json::json!({ "success": true })))
}

/// GET /api/v1/auth/token
/// Validates the token, extends its expiry by 24 h, and returns updated user info.
/// The same token string is returned -- clients should persist it to reflect the new expiry.
pub async fn token_info(
    pool: web::Data<SqlitePool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    token_model::refresh(&pool, &claims.token).await?;

    let db_user = user::find_by_id(&pool, &claims.sub)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(ApiResponse::ok(LoginData {
        access_token: claims.token,
        user: UserInfo {
            id: db_user.id,
            username: db_user.username,
            real_name: db_user.real_name,
            role: db_user.role,
            language: db_user.language,
            ui_theme: db_user.ui_theme,
        },
    }))
}

// ── password helpers ──

pub fn verify_password(password: &str, hash: &str) -> Result<(), AppError> {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    let parsed = PasswordHash::new(hash)
        .map_err(|_| AppError::Internal("Corrupted password hash".into()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| AppError::Unauthorized("Invalid credentials".into()))
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
    use argon2::password_hash::rand_core::OsRng;
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {e}")))
}

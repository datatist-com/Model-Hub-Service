use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::config::AppConfig;
use crate::errors::{ApiResponse, AppError};
use crate::middleware::auth::Claims;
use crate::models::user;

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
    pool: web::Data<SqlitePool>,
    config: web::Data<AppConfig>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let req = body.into_inner();

    if req.username.is_empty() || req.password.is_empty() {
        return Err(AppError::BadRequest("Username and password required".into()));
    }

    let user = user::find_by_username(&pool, &req.username)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".into()))?;

    if user.status != "active" {
        return Err(AppError::Forbidden("Account is frozen".into()));
    }

    verify_password(&req.password, &user.password_hash)?;

    let token = Claims::new(&user.id, &user.username, &user.role, &config.jwt_secret)?;

    Ok(ApiResponse::ok(LoginData {
        access_token: token,
        user: UserInfo {
            id: user.id,
            username: user.username,
            real_name: user.real_name,
            role: user.role,
            language: user.language,
            ui_theme: user.ui_theme,
        },
    }))
}

/// POST /api/v1/auth/logout  (stateless JWT — simply acknowledge)
pub async fn logout(_claims: Claims) -> Result<HttpResponse, AppError> {
    Ok(ApiResponse::ok(serde_json::json!({ "success": true })))
}

/// GET /api/v1/auth/token
/// Validates the current token, refreshes it (new 24h expiry), and returns
/// updated user info. Clients should replace their stored token with the one
/// returned here.
pub async fn token_info(
    pool: web::Data<SqlitePool>,
    config: web::Data<AppConfig>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let user = user::find_by_id(&pool, &claims.sub)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    if user.status != "active" {
        return Err(AppError::Forbidden("Account is frozen".into()));
    }

    // Issue a fresh token so the client's session is automatically extended.
    let new_token = Claims::new(&user.id, &user.username, &user.role, &config.jwt_secret)?;

    Ok(ApiResponse::ok(LoginData {
        access_token: new_token,
        user: UserInfo {
            id: user.id,
            username: user.username,
            real_name: user.real_name,
            role: user.role,
            language: user.language,
            ui_theme: user.ui_theme,
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

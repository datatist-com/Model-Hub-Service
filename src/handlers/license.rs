use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::errors::{ApiResponse, AppError};
use crate::models::license as license_model;

#[derive(Deserialize)]
pub struct LicenseTokenRequest {
    #[serde(rename = "licenseKey")]
    pub license_key: String,
}

#[derive(Serialize)]
pub struct VerifyResult {
    pub valid: bool,
    pub expired: bool,
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

#[derive(Serialize)]
pub struct LicenseInfo {
    pub status: &'static str,           // "active" | "expired" | "missing"
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[serde(rename = "licenseKeyMasked")]
    pub license_key_masked: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    #[serde(rename = "activatedAt")]
    pub activated_at: String,
}

/// POST /api/v1/license/verify
/// Decrypt and validate a license token without persisting it.
/// No auth required.
pub async fn verify(
    body: web::Json<LicenseTokenRequest>,
) -> Result<HttpResponse, AppError> {
    let token = body.license_key.trim().to_string();
    if token.is_empty() {
        return Err(AppError::BadRequest("licenseKey is required".into()));
    }

    let decoded = license_model::decrypt(&token)?;
    let expired = !license_model::is_valid(&decoded.expires_at);

    Ok(ApiResponse::ok(VerifyResult {
        valid: !expired,
        expired,
        project_name: decoded.project_name,
        expires_at: decoded.expires_at,
    }))
}

/// POST /api/v1/license/activate
/// Activate a verified license. Replaces any currently active license.
/// No auth required.
pub async fn activate(
    pool: web::Data<SqlitePool>,
    body: web::Json<LicenseTokenRequest>,
) -> Result<HttpResponse, AppError> {
    let token = body.license_key.trim().to_string();
    if token.is_empty() {
        return Err(AppError::BadRequest("licenseKey is required".into()));
    }

    let decoded = license_model::decrypt(&token)?;

    if !license_model::is_valid(&decoded.expires_at) {
        return Err(AppError::BadRequest(format!(
            "license has already expired ({})",
            decoded.expires_at
        )));
    }

    let row = license_model::activate(
        &pool,
        &token,
        &decoded.project_name,
        &decoded.expires_at,
    )
    .await?;

    Ok(ApiResponse::ok(to_info(&row)))
}

/// GET /api/v1/license
/// Return the currently active license or a "missing" placeholder.
/// No auth required.
pub async fn info(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, AppError> {
    match license_model::find_active(&pool).await? {
        Some(row) => Ok(ApiResponse::ok(to_info(&row))),
        None => Ok(ApiResponse::ok(LicenseInfo {
            status: "missing",
            project_name: String::new(),
            license_key_masked: String::new(),
            expires_at: String::new(),
            activated_at: String::new(),
        })),
    }
}

fn to_info(row: &license_model::License) -> LicenseInfo {
    let status = if license_model::is_valid(&row.expires_at) {
        "active"
    } else {
        "expired"
    };
    LicenseInfo {
        status,
        project_name: row.project_name.clone(),
        license_key_masked: mask_token(&row.token),
        expires_at: row.expires_at.clone(),
        activated_at: row.activated_at.clone(),
    }
}

/// Show first 8 chars + "****" + last 4 chars of the token.
fn mask_token(token: &str) -> String {
    let chars: Vec<char> = token.chars().collect();
    if chars.len() <= 12 {
        return "****".to_string();
    }
    let prefix: String = chars[..8].iter().collect();
    let suffix: String = chars[chars.len() - 4..].iter().collect();
    format!("{prefix}****{suffix}")
}

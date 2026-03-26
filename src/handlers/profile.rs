use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::errors::{ApiResponse, AppError};
use crate::handlers::auth::{hash_password, verify_password};
use crate::middleware::auth::Claims;
use crate::models::log as log_model;
use crate::models::user;

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    #[serde(rename = "currentPassword")]
    pub current_password: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
}

/// PUT /api/v1/profile/password
pub async fn change_password(
    pool: web::Data<SqlitePool>,
    claims: Claims,
    body: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse, AppError> {
    let req = body.into_inner();

    if req.new_password.len() < 6 {
        return Err(AppError::BadRequest(
            "New password must be at least 6 characters".into(),
        ));
    }

    let current_user = user::find_by_id(&pool, &claims.sub)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    verify_password(&req.current_password, &current_user.password_hash)?;

    let new_hash = hash_password(&req.new_password)?;
    user::update_password(&pool, &claims.sub, &new_hash).await?;

    let _ = log_model::insert_operation_log(
        &pool, &claims.sub, &claims.username, "profile", "change_password",
        None, None, None,
    ).await;

    Ok(ApiResponse::ok(serde_json::json!({ "success": true })))
}

use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::errors::{ApiResponse, AppError};
use crate::models::user::{self, hash_password, verify_password};
use crate::middleware::auth::Claims;
use crate::models::log as log_model;
use crate::models::token::extract_ip;

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    #[serde(rename = "currentPassword")]
    pub current_password: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
}

/// PUT /api/v1/profile/password
pub async fn change_password(
    http_req: HttpRequest,
    pool: web::Data<SqlitePool>,
    claims: Claims,
    body: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse, AppError> {
    let req = body.into_inner();

    if req.new_password.len() < 6 {
        return Err(AppError::BadRequest(
            "error.profile.password_too_short".into(),
        ));
    }

    let current_user = user::find_by_id(&pool, &claims.sub)
        .await?
        .ok_or_else(|| AppError::NotFound("error.users.not_found".into()))?;

    verify_password(&req.current_password, &current_user.password_hash)?;

    let new_hash = hash_password(&req.new_password)?;
    user::update_password(&pool, &claims.sub, &new_hash).await?;

    let ip = extract_ip(&http_req);
    log_model::log_operation(
        &pool, &claims.sub, &claims.username, "profile", "change_password",
        None, "operation.profile.change_password", serde_json::json!({}), &ip,
    ).await;

    Ok(ApiResponse::ok(serde_json::json!({ "success": true }), "message.profile.change_password.success"))
}

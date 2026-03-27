use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::SqlitePool;

use crate::errors::{ApiResponse, AppError, PagedData};
use crate::middleware::auth::AdminOnly;
use crate::models::log as log_model;
use crate::models::token::extract_ip;
use crate::models::user::{
    self, CreateUserRequest, UpdateUserRequest, UserView,
    hash_password, validate_role, validate_status,
};

/// GET /api/v1/users
pub async fn list(
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    query: serde_qs::actix::QsQuery<crate::pagination::PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let page = q.page();
    let page_size = q.page_size();

    let result = user::list_users(&pool, &q).await?;

    let items: Vec<UserView> = result.items.into_iter().map(UserView::from).collect();

    Ok(ApiResponse::ok(PagedData {
        items,
        page,
        page_size,
        total: result.total,
    }, "message.users.list.success"))
}

/// POST /api/v1/users
pub async fn create(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let r = body.into_inner();

    if r.username.trim().is_empty() || r.password.is_empty() {
        return Err(AppError::BadRequest("error.auth.credentials_required".into()));
    }
    if r.password.len() < 6 {
        return Err(AppError::BadRequest("error.users.password_too_short".into()));
    }
    validate_role(&r.role).map_err(AppError::BadRequest)?;

    let password_hash = hash_password(&r.password)?;

    let user = user::insert_user(
        &pool,
        &r.username,
        &password_hash,
        r.real_name.as_deref(),
        &r.role,
    )
    .await?;

    let ip = extract_ip(&req);
    log_model::log_operation(
        &pool, &_admin.0.sub, &_admin.0.username, "users", "create_user",
        Some(&user.id), "operation.users.create_user",
        serde_json::json!({"username": &user.username}), &ip,
    ).await;

    let username = user.username.clone();
    Ok(ApiResponse::ok_params(UserView::from(user), "message.users.create.success",
        serde_json::json!({"username": &username})))
}

/// PUT /api/v1/users/{id}
pub async fn update(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    path: web::Path<String>,
    body: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let r = body.into_inner();

    if let Some(ref role) = r.role {
        validate_role(role).map_err(AppError::BadRequest)?;
    }
    if let Some(ref status) = r.status {
        validate_status(status).map_err(AppError::BadRequest)?;
    }

    let user = user::update_user(
        &pool,
        &id,
        r.real_name.as_deref(),
        r.role.as_deref(),
        r.status.as_deref(),
    )
    .await?;

    let ip = extract_ip(&req);
    log_model::log_operation(
        &pool, &_admin.0.sub, &_admin.0.username, "users", "update_user",
        Some(&id), "operation.users.update_user",
        serde_json::json!({"username": &user.username}), &ip,
    ).await;

    let username = user.username.clone();
    Ok(ApiResponse::ok_params(UserView::from(user), "message.users.update.success",
        serde_json::json!({"username": &username})))
}

/// DELETE /api/v1/users/{id}
pub async fn delete(
    req: HttpRequest,
    pool: web::Data<SqlitePool>,
    admin: AdminOnly,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    // Prevent self-deletion
    if id == admin.0.sub {
        return Err(AppError::BadRequest("error.users.cannot_delete_self".into()));
    }

    // Fetch user info before deletion for logging
    let target_user = user::find_by_id(&pool, &id).await?;
    let target_username = target_user.map(|u| u.username).unwrap_or_default();

    user::delete_user(&pool, &id).await?;

    let ip = extract_ip(&req);
    log_model::log_operation(
        &pool, &admin.0.sub, &admin.0.username, "users", "delete_user",
        Some(&id), "operation.users.delete_user",
        serde_json::json!({"username": &target_username}), &ip,
    ).await;

    Ok(ApiResponse::ok(serde_json::json!({ "success": true }), "message.users.delete.success"))
}

use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

use crate::errors::{ApiResponse, AppError, PagedData};
use crate::handlers::auth::hash_password;
use crate::middleware::auth::AdminOnly;
use crate::models::log as log_model;
use crate::models::user::{
    self, CreateUserRequest, ListUsersQuery, UpdateUserRequest, UserView,
    validate_role, validate_status,
};

/// GET /api/v1/users
pub async fn list(
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    query: serde_qs::actix::QsQuery<ListUsersQuery>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let page = q.pagination.page();
    let page_size = q.pagination.page_size();

    let result = user::list_users(
        &pool,
        q.role.as_deref(),
        q.status.as_deref(),
        q.keyword.as_deref(),
        &q.pagination,
    )
    .await?;

    let items: Vec<UserView> = result.items.into_iter().map(UserView::from).collect();

    Ok(ApiResponse::ok(PagedData {
        items,
        page,
        page_size,
        total: result.total,
    }))
}

/// POST /api/v1/users
pub async fn create(
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let req = body.into_inner();

    if req.username.trim().is_empty() || req.password.is_empty() {
        return Err(AppError::BadRequest("Username and password required".into()));
    }
    validate_role(&req.role).map_err(AppError::BadRequest)?;

    let password_hash = hash_password(&req.password)?;

    let user = user::insert_user(
        &pool,
        &req.username,
        &password_hash,
        req.real_name.as_deref(),
        &req.role,
    )
    .await?;

    let _ = log_model::insert_operation_log(
        &pool, &_admin.0.sub, &_admin.0.username, "users", "create_user",
        Some(&user.id), Some(&format!("Created user {}", user.username)), None,
    ).await;

    Ok(ApiResponse::ok(UserView::from(user)))
}

/// PUT /api/v1/users/{id}
pub async fn update(
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    path: web::Path<String>,
    body: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let req = body.into_inner();

    if let Some(ref role) = req.role {
        validate_role(role).map_err(AppError::BadRequest)?;
    }
    if let Some(ref status) = req.status {
        validate_status(status).map_err(AppError::BadRequest)?;
    }

    let user = user::update_user(
        &pool,
        &id,
        req.real_name.as_deref(),
        req.role.as_deref(),
        req.status.as_deref(),
    )
    .await?;

    let _ = log_model::insert_operation_log(
        &pool, &_admin.0.sub, &_admin.0.username, "users", "update_user",
        Some(&id), Some(&format!("Updated user {}", user.username)), None,
    ).await;

    Ok(ApiResponse::ok(UserView::from(user)))
}

/// DELETE /api/v1/users/{id}
pub async fn delete(
    pool: web::Data<SqlitePool>,
    admin: AdminOnly,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    // Prevent self-deletion
    if id == admin.0.sub {
        return Err(AppError::BadRequest("Cannot delete yourself".into()));
    }

    user::delete_user(&pool, &id).await?;

    let _ = log_model::insert_operation_log(
        &pool, &admin.0.sub, &admin.0.username, "users", "delete_user",
        Some(&id), None, None,
    ).await;

    Ok(ApiResponse::ok(serde_json::json!({ "success": true })))
}

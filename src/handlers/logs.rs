use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

use crate::errors::{ApiResponse, AppError, PagedData};
use crate::middleware::auth::{AdminOnly, Claims};
use crate::models::log::{self, LoginLogQuery, OperationLogQuery};

// ─── Login Logs ───

/// GET /api/v1/logs/login/mine
/// Current user's own login history.
pub async fn my_login_logs(
    pool: web::Data<SqlitePool>,
    claims: Claims,
    query: serde_qs::actix::QsQuery<LoginLogQuery>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let page = q.pagination.page();
    let page_size = q.pagination.page_size();

    let result = log::list_login_logs(
        &pool,
        Some(&claims.sub),
        q.result.as_deref(),
        &q.pagination,
    )
    .await?;

    Ok(ApiResponse::ok(PagedData {
        items: result.items,
        page,
        page_size,
        total: result.total,
    }))
}

/// GET /api/v1/logs/login
/// Admin: all users' login history with optional filters.
pub async fn all_login_logs(
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    query: serde_qs::actix::QsQuery<LoginLogQuery>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let page = q.pagination.page();
    let page_size = q.pagination.page_size();

    let result = log::list_login_logs(
        &pool,
        q.user_id.as_deref(),
        q.result.as_deref(),
        &q.pagination,
    )
    .await?;

    Ok(ApiResponse::ok(PagedData {
        items: result.items,
        page,
        page_size,
        total: result.total,
    }))
}

// ─── Operation Logs ───

/// GET /api/v1/logs/operations/mine
/// Current user's own operation history.
pub async fn my_operation_logs(
    pool: web::Data<SqlitePool>,
    claims: Claims,
    query: serde_qs::actix::QsQuery<OperationLogQuery>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let page = q.pagination.page();
    let page_size = q.pagination.page_size();

    let result = log::list_operation_logs(
        &pool,
        Some(&claims.sub),
        q.module.as_deref(),
        q.action.as_deref(),
        &q.pagination,
    )
    .await?;

    Ok(ApiResponse::ok(PagedData {
        items: result.items,
        page,
        page_size,
        total: result.total,
    }))
}

/// GET /api/v1/logs/operations
/// Admin: all users' operation history with optional filters.
pub async fn all_operation_logs(
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    query: serde_qs::actix::QsQuery<OperationLogQuery>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let page = q.pagination.page();
    let page_size = q.pagination.page_size();

    let result = log::list_operation_logs(
        &pool,
        q.user_id.as_deref(),
        q.module.as_deref(),
        q.action.as_deref(),
        &q.pagination,
    )
    .await?;

    Ok(ApiResponse::ok(PagedData {
        items: result.items,
        page,
        page_size,
        total: result.total,
    }))
}

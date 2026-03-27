use actix_web::{web, HttpResponse};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::errors::{ApiResponse, AppError, PagedData};
use crate::middleware::auth::{AdminOnly, Claims};
use crate::models::log;
use crate::pagination::PaginationParams;

/// Build a paged response from a list result.
fn paged_response<T: Serialize>(
    items: Vec<T>,
    total: i64,
    q: &PaginationParams,
    message: &str,
) -> Result<HttpResponse, AppError> {
    Ok(ApiResponse::ok(PagedData {
        items,
        page: q.page(),
        page_size: q.page_size(),
        total,
    }, message))
}

// ─── Login Logs ───

/// GET /api/v1/logs/login/mine
pub async fn my_login_logs(
    pool: web::Data<SqlitePool>,
    claims: Claims,
    query: serde_qs::actix::QsQuery<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let result = log::list_login_logs(&pool, Some(&claims.sub), &q).await?;
    paged_response(result.items, result.total, &q, "message.logs.login_list.success")
}

/// GET /api/v1/logs/login
pub async fn all_login_logs(
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    query: serde_qs::actix::QsQuery<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let result = log::list_login_logs(&pool, None, &q).await?;
    paged_response(result.items, result.total, &q, "message.logs.login_list.success")
}

// ─── Operation Logs ───

/// GET /api/v1/logs/operations/mine
pub async fn my_operation_logs(
    pool: web::Data<SqlitePool>,
    claims: Claims,
    query: serde_qs::actix::QsQuery<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let result = log::list_operation_logs(&pool, Some(&claims.sub), &q).await?;
    paged_response(result.items, result.total, &q, "message.logs.operation_list.success")
}

/// GET /api/v1/logs/operations
pub async fn all_operation_logs(
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    query: serde_qs::actix::QsQuery<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let result = log::list_operation_logs(&pool, None, &q).await?;
    paged_response(result.items, result.total, &q, "message.logs.operation_list.success")
}

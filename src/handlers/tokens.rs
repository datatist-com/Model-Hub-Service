use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

use crate::errors::{ApiResponse, AppError, PagedData};
use crate::middleware::auth::AdminOnly;
use crate::models::token::{self, TokenView};
use crate::pagination::PaginationParams;

/// GET /api/v1/tokens
/// List all active (valid + not expired) tokens. Admin only.
pub async fn list(
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    query: serde_qs::actix::QsQuery<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let q = query.into_inner();
    let result = token::list_active(&pool, &q).await?;

    let items: Vec<TokenView> = result.items.into_iter().map(TokenView::from).collect();

    Ok(ApiResponse::ok(
        PagedData {
            items,
            page: q.page(),
            page_size: q.page_size(),
            total: result.total,
        },
        "message.tokens.list.success",
    ))
}

/// DELETE /api/v1/tokens/{id}
/// Revoke a specific token by id. Admin only.
pub async fn revoke(
    pool: web::Data<SqlitePool>,
    _admin: AdminOnly,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    token::revoke_by_id(&pool, &id).await?;

    Ok(ApiResponse::ok(
        serde_json::json!({ "success": true }),
        "message.tokens.revoke.success",
    ))
}

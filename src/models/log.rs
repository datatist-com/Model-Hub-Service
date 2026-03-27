use serde::Serialize;
use serde_json::Value;
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::pagination::PaginationParams;

/// Build the JSON detail string for operation logs.
pub fn build_detail(i18n_key: &str, params: Value) -> String {
    serde_json::json!({
        "i18n_key": i18n_key,
        "params": params
    }).to_string()
}

/// Insert an operation log entry; logs a warning on failure instead of propagating the error.
pub async fn log_operation(
    pool: &SqlitePool,
    user_id: &str,
    username: &str,
    module: &str,
    action: &str,
    target_id: Option<&str>,
    i18n_key: &str,
    params: Value,
    ip: &str,
) {
    let detail = build_detail(i18n_key, params);
    if let Err(e) = insert_operation_log(
        pool, user_id, username, module, action,
        target_id, Some(&detail), Some(ip),
    ).await {
        tracing::warn!("Failed to write operation log: {e}");
    }
}

// ── Login Log ──

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct LoginLog {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
    pub ip: Option<String>,
    pub device: Option<String>,
    pub result: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

pub async fn insert_login_log(
    pool: &SqlitePool,
    user_id: &str,
    username: &str,
    ip: Option<&str>,
    device: Option<&str>,
    result: &str,
) -> Result<(), AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO login_logs (id, user_id, username, ip, device, result) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )
    .bind(&id)
    .bind(user_id)
    .bind(username)
    .bind(ip)
    .bind(device)
    .bind(result)
    .execute(pool)
    .await?;
    Ok(())
}

pub struct LoginLogListResult {
    pub items: Vec<LoginLog>,
    pub total: i64,
}

pub async fn list_login_logs(
    pool: &SqlitePool,
    user_id: Option<&str>,
    pagination: &PaginationParams,
) -> Result<LoginLogListResult, sqlx::Error> {
    let offset = pagination.offset();
    let page_size = pagination.page_size();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM login_logs WHERE (?1 IS NULL OR user_id = ?1)",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let items = sqlx::query_as::<_, LoginLog>(
        "SELECT id, user_id, username, ip, device, result, created_at \
         FROM login_logs WHERE (?1 IS NULL OR user_id = ?1) \
         ORDER BY created_at DESC LIMIT ?2 OFFSET ?3",
    )
    .bind(user_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(LoginLogListResult {
        items,
        total,
    })
}

// ── Operation Log ──

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct OperationLog {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
    pub module: String,
    pub action: String,
    #[serde(rename = "targetId")]
    pub target_id: Option<String>,
    pub detail: Option<String>,
    pub ip: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

pub async fn insert_operation_log(
    pool: &SqlitePool,
    user_id: &str,
    username: &str,
    module: &str,
    action: &str,
    target_id: Option<&str>,
    detail: Option<&str>,
    ip: Option<&str>,
) -> Result<(), AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO operation_logs (id, user_id, username, module, action, target_id, detail, ip) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    )
    .bind(&id)
    .bind(user_id)
    .bind(username)
    .bind(module)
    .bind(action)
    .bind(target_id)
    .bind(detail)
    .bind(ip)
    .execute(pool)
    .await?;
    Ok(())
}

pub struct OperationLogListResult {
    pub items: Vec<OperationLog>,
    pub total: i64,
}

pub async fn list_operation_logs(
    pool: &SqlitePool,
    user_id: Option<&str>,
    pagination: &PaginationParams,
) -> Result<OperationLogListResult, sqlx::Error> {
    let offset = pagination.offset();
    let page_size = pagination.page_size();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM operation_logs WHERE (?1 IS NULL OR user_id = ?1)",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let items = sqlx::query_as::<_, OperationLog>(
        "SELECT * FROM operation_logs WHERE (?1 IS NULL OR user_id = ?1) \
         ORDER BY created_at DESC LIMIT ?2 OFFSET ?3",
    )
    .bind(user_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(OperationLogListResult {
        items,
        total,
    })
}

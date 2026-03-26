use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::pagination::PaginationParams;

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
    pub detail: Option<String>,
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
    detail: Option<&str>,
) -> Result<(), AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO login_logs (id, user_id, username, ip, device, result, detail) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )
    .bind(&id)
    .bind(user_id)
    .bind(username)
    .bind(ip)
    .bind(device)
    .bind(result)
    .bind(detail)
    .execute(pool)
    .await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct LoginLogQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    pub result: Option<String>,
}

pub struct LoginLogListResult {
    pub items: Vec<LoginLog>,
    pub total: i64,
}

const LOGIN_LOG_SORT_COLS: &[(&str, &str)] = &[
    ("createdAt", "created_at"),
    ("created_at", "created_at"),
    ("username", "username"),
    ("result", "result"),
];

pub async fn list_login_logs(
    pool: &SqlitePool,
    user_id: Option<&str>,
    result_filter: Option<&str>,
    pagination: &PaginationParams,
) -> Result<LoginLogListResult, sqlx::Error> {
    let sort_col = pagination.safe_sort_col(LOGIN_LOG_SORT_COLS, "created_at");
    let order = pagination.safe_order();
    let offset = pagination.offset();
    let page_size = pagination.page_size();

    let total: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM login_logs \
         WHERE (?1 IS NULL OR user_id = ?1) \
           AND (?2 IS NULL OR result = ?2)",
    )
    .bind(user_id)
    .bind(result_filter)
    .fetch_one(pool)
    .await?;

    let query_sql = format!(
        "SELECT * FROM login_logs \
         WHERE (?1 IS NULL OR user_id = ?1) \
           AND (?2 IS NULL OR result = ?2) \
         ORDER BY {sort_col} {order} \
         LIMIT ?3 OFFSET ?4"
    );
    let items = sqlx::query_as::<_, LoginLog>(&query_sql)
        .bind(user_id)
        .bind(result_filter)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok(LoginLogListResult {
        items,
        total: total as i64,
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

#[derive(Debug, Deserialize)]
pub struct OperationLogQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    pub module: Option<String>,
    pub action: Option<String>,
}

pub struct OperationLogListResult {
    pub items: Vec<OperationLog>,
    pub total: i64,
}

const OP_LOG_SORT_COLS: &[(&str, &str)] = &[
    ("createdAt", "created_at"),
    ("created_at", "created_at"),
    ("module", "module"),
    ("action", "action"),
    ("username", "username"),
];

pub async fn list_operation_logs(
    pool: &SqlitePool,
    user_id: Option<&str>,
    module: Option<&str>,
    action: Option<&str>,
    pagination: &PaginationParams,
) -> Result<OperationLogListResult, sqlx::Error> {
    let sort_col = pagination.safe_sort_col(OP_LOG_SORT_COLS, "created_at");
    let order = pagination.safe_order();
    let offset = pagination.offset();
    let page_size = pagination.page_size();

    let total: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM operation_logs \
         WHERE (?1 IS NULL OR user_id = ?1) \
           AND (?2 IS NULL OR module = ?2) \
           AND (?3 IS NULL OR action = ?3)",
    )
    .bind(user_id)
    .bind(module)
    .bind(action)
    .fetch_one(pool)
    .await?;

    let query_sql = format!(
        "SELECT * FROM operation_logs \
         WHERE (?1 IS NULL OR user_id = ?1) \
           AND (?2 IS NULL OR module = ?2) \
           AND (?3 IS NULL OR action = ?3) \
         ORDER BY {sort_col} {order} \
         LIMIT ?4 OFFSET ?5"
    );
    let items = sqlx::query_as::<_, OperationLog>(&query_sql)
        .bind(user_id)
        .bind(module)
        .bind(action)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok(OperationLogListResult {
        items,
        total: total as i64,
    })
}

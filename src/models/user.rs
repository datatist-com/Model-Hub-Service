use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::pagination::PaginationParams;

// ── Domain structs ──

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub real_name: Option<String>,
    pub role: String,
    pub status: String,
    pub language: Option<String>,
    pub ui_theme: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Safe projection without password_hash.
#[derive(Debug, Serialize)]
pub struct UserView {
    pub id: String,
    pub username: String,
    #[serde(rename = "realName")]
    pub real_name: Option<String>,
    pub role: String,
    pub status: String,
    pub language: Option<String>,
    #[serde(rename = "uiTheme")]
    pub ui_theme: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl From<User> for UserView {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            username: u.username,
            real_name: u.real_name,
            role: u.role,
            status: u.status,
            language: u.language,
            ui_theme: u.ui_theme,
            created_at: u.created_at,
        }
    }
}

// ── Request DTOs ──

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    #[serde(rename = "realName")]
    pub real_name: Option<String>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(rename = "realName")]
    pub real_name: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
}

// ── Validation helpers ──

const VALID_ROLES: &[&str] = &["model_developer", "model_operator", "platform_admin"];
const VALID_STATUSES: &[&str] = &["active", "frozen"];

pub fn validate_role(role: &str) -> Result<(), crate::errors::I18nMsg> {
    if VALID_ROLES.contains(&role) {
        Ok(())
    } else {
        Err(crate::errors::I18nMsg::with_params(
            "error.users.invalid_role",
            serde_json::json!({"role": role}),
        ))
    }
}

pub fn validate_status(status: &str) -> Result<(), crate::errors::I18nMsg> {
    if VALID_STATUSES.contains(&status) {
        Ok(())
    } else {
        Err(crate::errors::I18nMsg::with_params(
            "error.users.invalid_status",
            serde_json::json!({"status": status}),
        ))
    }
}

// ── DB queries ──

pub async fn find_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?1")
        .bind(username)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct ListResult {
    pub items: Vec<User>,
    pub total: i64,
}

pub async fn list_users(
    pool: &SqlitePool,
    pagination: &PaginationParams,
) -> Result<ListResult, sqlx::Error> {
    let offset = pagination.offset();
    let page_size = pagination.page_size();

    let total: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    let items = sqlx::query_as::<_, User>(
        "SELECT * FROM users ORDER BY created_at ASC LIMIT ?1 OFFSET ?2",
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(ListResult {
        items,
        total: total as i64,
    })
}

pub async fn insert_user(
    pool: &SqlitePool,
    username: &str,
    password_hash: &str,
    real_name: Option<&str>,
    role: &str,
) -> Result<User, sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, username, password_hash, real_name, role)
        VALUES (?1, ?2, ?3, ?4, ?5)
        RETURNING *
        "#,
    )
    .bind(&id)
    .bind(username)
    .bind(password_hash)
    .bind(real_name)
    .bind(role)
    .fetch_one(pool)
    .await
}

pub async fn update_user(
    pool: &SqlitePool,
    id: &str,
    real_name: Option<&str>,
    role: Option<&str>,
    status: Option<&str>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET real_name  = COALESCE(?2, real_name),
            role       = COALESCE(?3, role),
            status     = COALESCE(?4, status),
            updated_at = datetime('now')
        WHERE id = ?1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(real_name)
    .bind(role)
    .bind(status)
    .fetch_one(pool)
    .await
}

pub async fn delete_user(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    let rows = sqlx::query("DELETE FROM users WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected();
    if rows == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

pub async fn update_password(
    pool: &SqlitePool,
    id: &str,
    new_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET password_hash = ?2, updated_at = datetime('now') WHERE id = ?1")
        .bind(id)
        .bind(new_hash)
        .execute(pool)
        .await?;
    Ok(())
}

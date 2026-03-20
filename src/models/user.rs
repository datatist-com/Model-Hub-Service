use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

// ── Domain structs ──

#[derive(Debug, Clone, sqlx::FromRow)]
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

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<i64>,
    pub role: Option<String>,
    pub status: Option<String>,
    pub keyword: Option<String>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<String>,
}

// ── Validation helpers ──

const VALID_ROLES: &[&str] = &["model_developer", "model_operator", "platform_admin"];
const VALID_STATUSES: &[&str] = &["active", "frozen"];

pub fn validate_role(role: &str) -> Result<(), String> {
    if VALID_ROLES.contains(&role) {
        Ok(())
    } else {
        Err(format!("Invalid role: {role}"))
    }
}

pub fn validate_status(status: &str) -> Result<(), String> {
    if VALID_STATUSES.contains(&status) {
        Ok(())
    } else {
        Err(format!("Invalid status: {status}"))
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
    page: i64,
    page_size: i64,
    role: Option<&str>,
    status: Option<&str>,
    keyword: Option<&str>,
    sort_by: &str,
    sort_order: &str,
) -> Result<ListResult, sqlx::Error> {
    // Whitelist sort columns to prevent injection
    let sort_col = match sort_by {
        "username" => "username",
        "role" => "role",
        "status" => "status",
        "realName" | "real_name" => "real_name",
        _ => "created_at",
    };
    let order = if sort_order.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };

    let offset = (page - 1) * page_size;

    let count_sql = r#"
        SELECT COUNT(*)
        FROM users
        WHERE (?1 IS NULL OR role = ?1)
          AND (?2 IS NULL OR status = ?2)
          AND (?3 IS NULL OR username LIKE '%' || ?3 || '%' OR real_name LIKE '%' || ?3 || '%')
    "#;
    let total: i32 = sqlx::query_scalar(count_sql)
        .bind(role)
        .bind(status)
        .bind(keyword)
        .fetch_one(pool)
        .await?;

    let query_sql = format!(
        r#"
        SELECT *
        FROM users
        WHERE (?1 IS NULL OR role = ?1)
          AND (?2 IS NULL OR status = ?2)
          AND (?3 IS NULL OR username LIKE '%' || ?3 || '%' OR real_name LIKE '%' || ?3 || '%')
        ORDER BY {sort_col} {order}
        LIMIT ?4 OFFSET ?5
        "#
    );
    let items = sqlx::query_as::<_, User>(&query_sql)
        .bind(role)
        .bind(status)
        .bind(keyword)
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

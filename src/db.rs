use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub async fn init_pool(database_url: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create database pool")
}

pub async fn run_migrations(pool: &SqlitePool) {
    let sql = include_str!("../migrations/001_create_users.sql");
    sqlx::raw_sql(sql)
        .execute(pool)
        .await
        .expect("Failed to run migrations");
    tracing::info!("Database migrations applied");
}

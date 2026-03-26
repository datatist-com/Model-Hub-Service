use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub async fn init_pool(database_url: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .expect("Failed to create database pool")
}

pub async fn run_migrations(pool: &SqlitePool) {
    for (name, sql) in [
        ("001_create_users",    include_str!("../migrations/001_create_users.sql")),
        ("002_create_tokens",   include_str!("../migrations/002_create_tokens.sql")),
        ("003_create_licenses", include_str!("../migrations/003_create_licenses.sql")),
        ("004_create_logs",     include_str!("../migrations/004_create_logs.sql")),
        ("005_add_indexes",     include_str!("../migrations/005_add_indexes.sql")),
    ] {
        sqlx::raw_sql(sql)
            .execute(pool)
            .await
            .unwrap_or_else(|e| panic!("Migration {name} failed: {e}"));
    }
    tracing::info!("Database migrations applied");
}

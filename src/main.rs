mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod pagination;
mod routes;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use actix_web::http::header;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let config = config::AppConfig::from_args();
    let pool = db::init_pool(&config.database_url).await;
    db::run_migrations(&pool).await;

    // Seed default admin if no users exist
    seed_admin(&pool).await;

    let bind = format!("{}:{}", config.server_host, config.server_port);
    tracing::info!("Starting server on {bind}");

    let cfg = web::Data::new(config);
    let pool = web::Data::new(pool);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .app_data(cfg.clone())
            .app_data(pool.clone())
            .configure(routes::configure)
    })
    .bind(&bind)?
    .run()
    .await
}

async fn seed_admin(pool: &sqlx::SqlitePool) {
    use models::user;
    if let Ok(None) = user::find_by_username(pool, "admin").await {
        let hash = handlers::auth::hash_password("123456")
            .expect("Failed to hash seed password");
        let _ =
            user::insert_user(pool, "admin", &hash, Some("默认管理员"), "platform_admin").await;
        tracing::info!("Seeded default admin user (username: admin, password: 123456)");
    }
}

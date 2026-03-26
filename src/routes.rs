use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // License (no auth required)
            .service(
                web::scope("/license")
                    .route("", web::get().to(handlers::license::info))
                    .route("/verify", web::post().to(handlers::license::verify))
                    .route("/activate", web::post().to(handlers::license::activate)),
            )
            // Auth (no auth required for login)
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("/logout", web::post().to(handlers::auth::logout))
                    .route("/token", web::get().to(handlers::auth::token_info)),
            )
            // Users management (admin only)
            .service(
                web::scope("/users")
                    .route("", web::get().to(handlers::users::list))
                    .route("", web::post().to(handlers::users::create))
                    .route("/{id}", web::put().to(handlers::users::update))
                    .route("/{id}", web::delete().to(handlers::users::delete)),
            )
            // Profile (authenticated user)
            .service(
                web::scope("/profile")
                    .route("/password", web::put().to(handlers::profile::change_password)),
            )
            // Logs
            .service(
                web::scope("/logs")
                    .route("/login", web::get().to(handlers::logs::all_login_logs))
                    .route("/login/mine", web::get().to(handlers::logs::my_login_logs))
                    .route("/operations", web::get().to(handlers::logs::all_operation_logs))
                    .route("/operations/mine", web::get().to(handlers::logs::my_operation_logs)),
            ),
    );
}

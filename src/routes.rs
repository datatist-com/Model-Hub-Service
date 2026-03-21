use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
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
            ),
    );
}

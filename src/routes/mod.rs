pub mod auth;
pub mod bugs;
pub mod projects;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(bugs::scope())
            .service(projects::scope())
            .service(auth::scope()),
    )
    .service(
        web::scope("")
            .service(bugs::html_scope())
    );
}
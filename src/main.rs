use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use sqlx::sqlite::SqlitePool;

mod models;
mod routes;
mod state;
mod database;

use routes::config as routes_config;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database
    let pool = database::init_db().await.expect("Failed to initialize database");

    // Create application state
    let app_state = web::Data::new(AppState {
        projects: Mutex::new(vec![]),
        db_pool: pool,
    });

    // Load initial projects
    if let Ok(projects) = database::load_projects(&app_state.db_pool).await {
        *app_state.projects.lock().unwrap() = projects;
    }

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
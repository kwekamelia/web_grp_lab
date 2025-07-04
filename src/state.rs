use std::sync::Mutex;
use sqlx::sqlite::SqlitePool;

pub struct AppState {
    pub projects: Mutex<Vec<super::models::Project>>,
    pub db_pool: SqlitePool,
}
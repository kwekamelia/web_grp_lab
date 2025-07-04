use sqlx::{sqlite::SqlitePool, Sqlite};
use crate::models::{Bug, Project};

pub async fn init_db() -> Result<sqlx::Pool<Sqlite>, sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:bugtracker.db").await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

pub async fn load_projects(pool: &SqlitePool) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(pool)
        .await
}
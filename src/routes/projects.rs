use actix_web::{web, HttpResponse, get, post};
use serde_json::json;
use uuid::Uuid;
use crate::{models::{Project, NewProject}, state::AppState};

#[get("")]
async fn list_projects(state: web::Data<AppState>) -> HttpResponse {
    let projects = state.projects.lock().unwrap();
    HttpResponse::Ok().json(projects.clone())
}

#[post("")]
async fn create_project(
    state: web::Data<AppState>,
    new_project: web::Json<NewProject>,
) -> HttpResponse {
    let project_id = Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();

    match sqlx::query!(
        r#"
        INSERT INTO projects (project_id, name, description, created_at)
        VALUES (?, ?, ?, ?)
        "#,
        project_id,
        new_project.name,
        new_project.description,
        created_at
    )
    .execute(&state.db_pool)
    .await {
        Ok(_) => {
            let project = Project {
                project_id,
                name: new_project.name.clone(),
                description: new_project.description.clone(),
                created_at,
            };
            
            // Update in-memory state
            state.projects.lock().unwrap().push(project.clone());
            
            HttpResponse::Created().json(project)
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

pub fn scope() -> actix_web::Scope {
    web::scope("/projects")
        .service(list_projects)
        .service(create_project)
}
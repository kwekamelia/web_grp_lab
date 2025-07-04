use actix_web::{web, HttpResponse, delete, get, patch, post};
use serde_json::json;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::{models::{Bug, NewBug, UpdateBug, AssignBugForm}, state::AppState};

#[post("/new")]
async fn create_bug(
    state: web::Data<AppState>,
    new_bug: web::Json<NewBug>,
) -> HttpResponse {
    let bug_id = Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();

    match sqlx::query!(
        r#"
        INSERT INTO bugs (bug_id, title, description, reported_by, severity, status, created_at, project_id)
        VALUES (?, ?, ?, ?, ?, 'open', ?, ?)
        "#,
        bug_id,
        new_bug.title,
        new_bug.description,
        new_bug.reported_by,
        new_bug.severity,
        created_at,
        new_bug.project_id
    )
    .execute(&state.db_pool)
    .await {
        Ok(_) => {
            let bug = Bug {
                bug_id,
                title: new_bug.title.clone(),
                description: new_bug.description.clone(),
                reported_by: new_bug.reported_by.clone(),
                severity: new_bug.severity.clone(),
                status: "open".to_string(),
                assigned_to: None,
                project_id: new_bug.project_id.clone(),
                created_at,
            };
            HttpResponse::Created().json(bug)
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
        }
    }
}

#[get("")]
async fn list_bugs(
    state: web::Data<AppState>,
    query: web::Query<Vec<(String, String)>>,
) -> HttpResponse {
    let mut base_query = "SELECT * FROM bugs".to_string();
    let mut conditions = vec![];
    let mut params = vec![];

    for (key, value) in query.into_inner() {
        match key.as_str() {
            "status" => {
                conditions.push("status = ?");
                params.push(value);
            }
            "severity" => {
                conditions.push("severity = ?");
                params.push(value);
            }
            "project_id" => {
                conditions.push("project_id = ?");
                params.push(value);
            }
            _ => {}
        }
    }

    if !conditions.is_empty() {
        base_query.push_str(" WHERE ");
        base_query.push_str(&conditions.join(" AND "));
    }

    match sqlx::query_as::<_, Bug>(&base_query)
        .fetch_all(&state.db_pool)
        .await {
            Ok(bugs) => HttpResponse::Ok().json(bugs),
            Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
        }
}

#[get("/{id}")]
async fn get_bug(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let bug_id = path.into_inner();

    match sqlx::query_as::<_, Bug>(
        "SELECT * FROM bugs WHERE bug_id = ?"
    )
    .bind(bug_id)
    .fetch_one(&state.db_pool)
    .await {
        Ok(bug) => HttpResponse::Ok().json(bug),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(json!({"error": "Bug not found"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

#[patch("/{id}")]
async fn update_bug(
    state: web::Data<AppState>,
    path: web::Path<String>,
    update_data: web::Json<UpdateBug>,
) -> HttpResponse {
    let bug_id = path.into_inner();

    // First get the existing bug
    let existing_bug = match sqlx::query_as::<_, Bug>(
        "SELECT * FROM bugs WHERE bug_id = ?"
    )
    .bind(&bug_id)
    .fetch_one(&state.db_pool)
    .await {
        Ok(bug) => bug,
        Err(sqlx::Error::RowNotFound) => return HttpResponse::NotFound().json(json!({"error": "Bug not found"})),
        Err(e) => return HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    };

    // Apply updates
    let title = update_data.title.as_ref().unwrap_or(&existing_bug.title);
    let description = update_data.description.as_ref().unwrap_or(&existing_bug.description);
    let severity = update_data.severity.as_ref().unwrap_or(&existing_bug.severity);
    let status = update_data.status.as_ref().unwrap_or(&existing_bug.status);
    let assigned_to = update_data.assigned_to.as_ref().or(existing_bug.assigned_to.as_ref());
    let project_id = update_data.project_id.as_ref().or(existing_bug.project_id.as_ref());

    match sqlx::query!(
        r#"
        UPDATE bugs 
        SET title = ?, description = ?, severity = ?, status = ?, assigned_to = ?, project_id = ?
        WHERE bug_id = ?
        "#,
        title,
        description,
        severity,
        status,
        assigned_to,
        project_id,
        bug_id
    )
    .execute(&state.db_pool)
    .await {
        Ok(_) => {
            let updated_bug = Bug {
                bug_id,
                title: title.clone(),
                description: description.clone(),
                reported_by: existing_bug.reported_by,
                severity: severity.clone(),
                status: status.clone(),
                assigned_to: assigned_to.cloned(),
                project_id: project_id.cloned(),
                created_at: existing_bug.created_at,
            };
            HttpResponse::Ok().json(updated_bug)
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

#[delete("/{id}")]
async fn delete_bug(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let bug_id = path.into_inner();

    match sqlx::query!(
        "DELETE FROM bugs WHERE bug_id = ?",
        bug_id
    )
    .execute(&state.db_pool)
    .await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().json(json!({"status": "deleted"}))
            } else {
                HttpResponse::NotFound().json(json!({"error": "Bug not found"}))
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

#[post("/assign")]
async fn assign_bug(
    state: web::Data<AppState>,
    form: web::Form<AssignBugForm>,
) -> HttpResponse {
    match sqlx::query!(
        "UPDATE bugs SET assigned_to = ? WHERE bug_id = ?",
        form.developer_id,
        form.bug_id
    )
    .execute(&state.db_pool)
    .await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body(format!(
                    "<html><body><h1>Bug {} assigned to developer {}</h1></body></html>",
                    form.bug_id, form.developer_id
                ))
            } else {
                HttpResponse::NotFound().body("<html><body><h1>Bug not found</h1></body></html>")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!(
            "<html><body><h1>Error: {}</h1></body></html>",
            e
        ))
    }
}

#[get("/assign")]
async fn assign_bug_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../templates/assign_bug.html"))
}

pub fn scope() -> actix_web::Scope {
    web::scope("/bugs")
        .service(create_bug)
        .service(list_bugs)
        .service(get_bug)
        .service(update_bug)
        .service(delete_bug)
}

pub fn html_scope() -> actix_web::Scope {
    web::scope("/bugs")
        .service(assign_bug_form)
        .service(assign_bug)
}
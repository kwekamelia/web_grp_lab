use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Bug {
    pub bug_id: String,
    pub title: String,
    pub description: String,
    pub reported_by: String,
    pub severity: String,
    pub status: String,
    pub assigned_to: Option<String>,
    pub project_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBug {
    pub title: String,
    pub description: String,
    pub reported_by: String,
    pub severity: String,
    pub project_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBug {
    pub title: Option<String>,
    pub description: Option<String>,
    pub severity: Option<String>,
    pub status: Option<String>,
    pub assigned_to: Option<String>,
    pub project_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Project {
    pub project_id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProject {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub status: String,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignBugForm {
    pub bug_id: String,
    pub developer_id: String,
}
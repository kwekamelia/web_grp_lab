use actix_web::{post, web, HttpResponse};
use argon2::{self, Argon2, PasswordHasher};
use argon2::password_hash::{PasswordHash, SaltString};
use crate::models::{LoginRequest, LoginResponse};

const SALT: &str = "bugtrack2025";

#[post("/login")]
async fn login(login_data: web::Json<LoginRequest>) -> HttpResponse {
    let salt = SaltString::from_b64(SALT).unwrap();
    let argon2 = Argon2::default();
    
    // Hash the input password
    let password_hash = match argon2.hash_password(login_data.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return HttpResponse::InternalServerError().json("Failed to hash password")
    };

    // Compare with stored hash (in real app, this would come from DB)
    let stored_hash = if login_data.username == "admin" {
        match argon2.hash_password(b"adminpassword", &salt) {
            Ok(hash) => hash.to_string(),
            Err(_) => return HttpResponse::InternalServerError().json("Failed to hash password")
        }
    } else {
        "".to_string()
    };

    if password_hash == stored_hash {
        HttpResponse::Ok().json(LoginResponse {
            status: "success".to_string(),
            token: Some("fake-session-token".to_string()),
        })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            status: "failure".to_string(),
            token: None,
        })
    }
}

pub fn scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(login)
}
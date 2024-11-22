use crate::{jwt, middleware::Auth, models::*};
use axum::{extract::State, http::StatusCode, response::Response, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::json;
use sqlx::SqlitePool;
use std::sync::Arc;

/// User registration handler
pub async fn register_handler(
    State(pool): State<Arc<SqlitePool>>,
    Json(user): Json<UserRegistration>,
) -> Response<String> {
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to hash password".to_string())
                .unwrap_or_default()
        }
    };

    let result = sqlx::query("INSERT INTO users (email, password) VALUES (?, ?)")
        .bind(&user.email)
        .bind(&hashed_password)
        .execute(&*pool)
        .await;

    match result {
        Ok(_) => Response::builder()
            .status(StatusCode::CREATED)
            .body(
                json!({ "success": true, "message": "User registered successfully" }).to_string(),
            )
            .unwrap_or_default(),
        Err(_) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(json!({ "success": false, "message": "User already exists" }).to_string())
            .unwrap_or_default(),
    }
}

/// User login handler
pub async fn login_handler(
    State(pool): State<Arc<SqlitePool>>,
    Json(user): Json<UserLogin>,
) -> Response<String> {
    let result: Option<(String,)> = sqlx::query_as("SELECT password FROM users WHERE email = ?")
        .bind(&user.email)
        .fetch_optional(&*pool)
        .await
        .ok()
        .flatten();

    if let Some((hashed_password,)) = result {
        if verify(&user.password, &hashed_password).unwrap_or(false) {
            let token = jwt::generate_jwt(&user.email);
            return Response::builder()
                .status(StatusCode::OK)
                .body(json!({ "success": true, "token": token }).to_string())
                .unwrap_or_default();
        }
    }

    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(json!({ "success": false, "message": "Invalid credentials" }).to_string())
        .unwrap_or_default()
}

/// Protected route handler
pub async fn secret_view_handler(Auth(user): Auth) -> Response<String> {
    Response::builder()
        .status(StatusCode::OK)
        .body(json!({ "success": true, "user": user }).to_string())
        .unwrap_or_default()
}

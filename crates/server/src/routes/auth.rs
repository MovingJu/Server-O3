use aide::axum::{ApiRouter, routing::post};
use axum::{Json, extract::State, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};
use log::{error, info};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{prelude::*, repository::RepoFactory};

pub fn get_router(state: Arc<RepoFactory>) -> (Option<Tag>, ApiRouter) {
    (
        Some(Tag {
            name: "auth".to_string(),
            description: Some("Login and signup".to_string()),
            ..Default::default()
        }),
        ApiRouter::new()
            .api_route("/login", post(login))
            .api_route("/signup", post(signup))
            .with_state(state)
            .with_prefix("/auth")
            .with_tag("auth"),
    )
}

#[derive(Deserialize, JsonSchema)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct SignupRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "changeme".to_string())
}

fn exp_30_days() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 30 * 24 * 3600
}

pub async fn login(
    State(state): State<Arc<RepoFactory>>,
    Json(body): Json<LoginRequest>,
) -> (StatusCode, Json<Value>) {
    let user = match state.auth.find_by_username(&body.username).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (StatusCode::UNAUTHORIZED, Json(json!({ "message": "Invalid username or password" })));
        }
        Err(err) => {
            error!("DB error on login: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Server Error" })));
        }
    };

    match bcrypt::verify(&body.password, &user.password_hash) {
        Ok(true) => {}
        Ok(false) => {
            return (StatusCode::UNAUTHORIZED, Json(json!({ "message": "Invalid username or password" })));
        }
        Err(err) => {
            error!("bcrypt error: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Server Error" })));
        }
    }

    let claims = Claims { sub: body.username.clone(), exp: exp_30_days() };
    match encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret().as_bytes())) {
        Ok(token) => {
            info!("User '{}' logged in", body.username);
            (StatusCode::OK, Json(json!({ "token": token })))
        }
        Err(err) => {
            error!("JWT encode error: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Server Error" })))
        }
    }
}

pub async fn signup(
    State(state): State<Arc<RepoFactory>>,
    Json(body): Json<SignupRequest>,
) -> (StatusCode, Json<Value>) {
    if body.username.trim().is_empty() || body.email.trim().is_empty() || body.password.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({ "message": "All fields are required" })));
    }

    let password_hash = match bcrypt::hash(&body.password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(err) => {
            error!("bcrypt error: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Server Error" })));
        }
    };

    match state.auth.create_user(&body.username, &body.email, &password_hash).await {
        Ok(()) => {
            info!("New user '{}' signed up", body.username);
            (StatusCode::OK, Json(json!({})))
        }
        Err(sqlx::Error::Database(e)) if e.message().contains("duplicate") || e.message().contains("unique") => {
            (StatusCode::CONFLICT, Json(json!({ "message": "Username or email already taken" })))
        }
        Err(err) => {
            error!("DB error on signup: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Server Error" })))
        }
    }
}

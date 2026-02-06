use aide::axum::{ApiRouter, routing::get};
use axum::Json;
use log::info;
use schemars::JsonSchema;
use serde::Serialize;

use crate::prelude::*;

/// # get_router
/// Adds route easily in `main.rs` file.
pub fn get_router() -> ApiRouter {
    ApiRouter::new()
        .api_route("/get_users", get(get_users))
        .api_route("/set_users", get(set_users))
        .with_prefix("/users")
}

#[derive(Serialize, JsonSchema)]
pub struct UserResp {
    pub id: usize,
    pub name: String,
    pub email: Option<String>,
}

pub async fn get_users() -> Json<ApiResponse<UserResp>> {
    info!("Request to get users table.");
    Json(ApiResponse {
        code: 200,
        resp: "ok".to_string(),
        data: UserResp {
            id: 1,
            name: "MovingJu".to_string(),
            email: None,
        },
    })
}

pub async fn set_users() -> Json<ApiResponse<Empty>> {
    info!("Request to set users table.");
    Json(ApiResponse {
        code: 0,
        resp: "ok".to_string(),
        data: Empty,
    })
}

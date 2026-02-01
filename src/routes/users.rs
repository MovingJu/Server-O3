use axum::{Json, Router, routing::get};
use log::info;
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

use crate::prelude::*;

#[derive(OpenApi)]
#[openapi(
    paths(get_users, set_users),
    tags((
        name = "users",
        description = "APIs for manipulating users table."
    ))
)]
pub struct UsersApi;
/// # get_router
/// Adds route easily in `main.rs` file.
pub fn get_router() -> Router {
    Router::new()
        .route("/get_users", get(get_users))
        .route("/set_users", get(set_users))
        .with_prefix("/users")
}

#[derive(Serialize, ToSchema)]
pub struct UserResp {
    pub id: usize,
    pub name: String,
    #[schema(nullable)]
    pub email: Option<String>,
}

#[utoipa::path(
    get,
    tag = "users",
    path = "/get_users",
    responses(
        (status = 200, body = ApiResponse<UserResp>, description = "JSON response.")
    )
)]
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

#[utoipa::path(
    get,
    tag = "users",
    path = "/set_users",
    responses(
        (status = 200, body = ApiResponse<Empty>, description = "JSON response")
    )
)]
pub async fn set_users() -> Json<ApiResponse<Empty>> {
    info!("Request to set users table.");
    Json(ApiResponse {
        code: 0,
        resp: "ok".to_string(),
        data: Empty,
    })
}

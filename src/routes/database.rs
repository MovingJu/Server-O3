//! # database
//! module testing my database

use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use log::error;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};

use crate::{
    prelude::*,
    repository::{
        Repo, 
        RepoFactory, 
        users::Users,
        posts::Posts
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(get_user, set_user, get_post),
    tags((
        name = "database",
        description = "APIs for testings database."
    ))
)]
pub struct DatabaseApi;
/// # get_router
/// Adds route easily in `main.rs` file.
pub fn get_router(state: std::sync::Arc<RepoFactory>) -> Router {
    Router::new()
        .route("/get_user", get(get_user))
        .route("/set_user", post(set_user))
        .with_state(state)
        .with_prefix("/db")
}

#[utoipa::path(
    get,
    tag = "database",
    path = "/get_user",
    params(GetUserQuery),
    responses(
        (status = 0, body = ApiResponse<Vec<Users>>, description = "ok"),
        (status = 1, body = ApiResponse<Vec<Users>>, description = "Error occur: {err.what()}")
    )
)]
pub async fn get_user(
    State(state): State<std::sync::Arc<RepoFactory>>,
    Query(query): Query<GetUserQuery>,
) -> Json<ApiResponse<Vec<Users>>> {
    let state = state.user.clone();
    let criteria = Users {
        name: query.name,
        ..Default::default()
    };
    match state.select(&criteria).await {
        Ok(v) => Json(ApiResponse {
            code: 0,
            resp: "ok".to_string(),
            data: v,
        }),
        Err(err) => {
            error!("Error occur: {}", err);
            Json(ApiResponse {
                code: 1,
                resp: format!("Error occur: {}", err),
                ..Default::default()
            })
        }
    }
}
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct GetUserQuery {
    name: String,
}

#[utoipa::path(
    post,
    tag = "database",
    path = "/set_user",
    params(SetUserQuery),
    responses(
        (status = 0, body = ApiResponse<Empty>, description = "ok"),
        (status = 1, body = ApiResponse<Empty>, description = "Error occur: {err.what()}")
    )
)]
pub async fn set_user(
    State(state): State<std::sync::Arc<RepoFactory>>,
    Query(query): Query<SetUserQuery>,
) -> Json<ApiResponse<Empty>> {
    let row = Users {
        name: query.name,
        ..Default::default()
    };
    match state.user.insert(&row).await {
        Ok(..) => Json(ApiResponse {
            code: 0,
            resp: "ok".to_string(),
            ..Default::default()
        }),
        Err(err) => Json(ApiResponse {
            code: 1,
            resp: format!("Error occur: {}", err),
            ..Default::default()
        }),
    }
}
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct SetUserQuery {
    name: String,
}

#[utoipa::path(
    post,
    tag = "database",
    path = "/get_post",
    params(GetPostQuery),
    responses(
        (status = 0, body = ApiResponse<Empty>, description = "ok"),
        (status = 1, body = ApiResponse<Empty>, description = "Error occur: {err.what()}")
    )
)]
pub async fn get_post(
    State(state): State<std::sync::Arc<RepoFactory>>,
    Query(query): Query<SetUserQuery>,
) -> Json<ApiResponse<Empty>> {
    let row = Posts {
        ..Default::default()
    };
    match state.posts.insert(&row).await {
        Ok(..) => Json(ApiResponse {
            code: 0,
            resp: "ok".to_string(),
            ..Default::default()
        }),
        Err(err) => Json(ApiResponse {
            code: 1,
            resp: format!("Error occur: {}", err),
            ..Default::default()
        }),
    }
}
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct GetPostQuery {
    user_id: i64,
    title: Option<String>
}
#[derive(Serialize, ToSchema)]
pub struct GetPostResponse {
    id: i64,
    title: String,
    content: String,
    user_id: i64
}
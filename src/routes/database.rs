//! # database
//! module testing my database

use axum::{Json, Router, extract::{State, Query}, routing::{get, post}};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi, ToSchema};
use log::error;

use crate::{prelude::*, repository::{RepoFactory, users::{Users, UsersRepo}}};

#[derive(OpenApi)]
#[openapi(
    paths(get_user, set_user),
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
        (status = 0, body = ApiResponse<Users>, description = "Calculates Nth get_usernacci number."),
        (status = 1, body = ApiResponse<Users>, description = "Input number exceeds the maximum limit (5,000)")
    )
)]
pub async fn get_user(
    State(state): State<std::sync::Arc<RepoFactory>>,
    Query(query): Query<GetUserQuery>
) -> Json<ApiResponse<Users>> {
    let state = state.user.clone();
    match state.find_by_key(query.id).await {
        Ok(v) => Json(ApiResponse { code: 0, resp: "ok".to_string(), data: v }),
        Err(err) => {
            error!("Error occur: {}", err);
            Json(ApiResponse { code: 1, resp: format!("Error occur: {}", err), ..Default::default()})
        }
    }
}
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct GetUserQuery {
    id: i64,
}

#[utoipa::path(
    post,
    tag = "database",
    path = "/set_user",
    params(SetUserQuery),
    responses(
        (status = 0, body = ApiResponse<Empty>, description = "ok")
    )
)]
pub async fn set_user(
    State(state): State<std::sync::Arc<RepoFactory>>,
    Query(query): Query<SetUserQuery>
) -> Json<ApiResponse<Empty>> {
    let state = state.user.clone();
    match state.insert(query.name).await {
        Ok(..) => Json(ApiResponse { code: 0, resp: "ok".to_string(),..Default::default() }),
        Err(err) => Json(ApiResponse { code: 1, resp: format!("Error occur: {}", err), ..Default::default() })
    }
}
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct SetUserQuery {
    name: String
}
//! # database
//! module testing my database

use axum::{Json, Router, extract::{State, Query}, routing::get};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi, ToSchema};
use log::error;

use crate::{prelude::*, services::sql_pool::AppState};

#[derive(OpenApi)]
#[openapi(
    paths(get_user),
    tags((
        name = "database",
        description = "APIs for testings database."
    ))
)]
pub struct DatabaseApi;
/// # get_router
/// Adds route easily in `main.rs` file.
pub fn get_router(state: std::sync::Arc<AppState>) -> Router {
    Router::new()
        .route("/get_user", get(get_user))
        .with_state(state)
        .with_prefix("/db")
}

#[utoipa::path(
    get,
    tag = "database",
    path = "/get_user",
    params(GetUserQuery),
    responses(
        (status = 0, body = ApiResponse<String>, description = "Calculates Nth get_usernacci number."),
        (status = 1, body = ApiResponse<String>, description = "Input number exceeds the maximum limit (5,000)")
    )
)]
pub async fn get_user(
    State(state): State<std::sync::Arc<AppState>>,
    Query(query): Query<GetUserQuery>
) -> Json<ApiResponse<String>> {
    todo!();
    // match find_user_by_key(&state, query.id).await {
    //     Ok(v) => Json(ApiResponse { code: 0, resp: "ok".to_string(), data: v }),
    //     Err(err) => {
    //         error!("Error occur: {}", err);
    //         Json(ApiResponse { code: 1, resp: format!("Error occur: {}", err), ..Default::default()})
    //     }
    // }
}
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct GetUserQuery {
    id: usize,
}
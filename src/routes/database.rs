//! # database
//! module testing my database

use axum::{Json, routing::get, Router, extract::Query};
use serde::{Deserialize};
use utoipa::{OpenApi, ToSchema, IntoParams};

use crate::prelude::*;

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
pub fn get_router() -> Router {
    Router::new()
        .route("/get_user", get(get_user))
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
pub async fn get_user(Query(query): Query<GetUserQuery>) -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        code: 1,
        resp: "Bad Request".to_string(),
        data: query.n.to_string(),
    })
}
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct GetUserQuery {
    n: usize,
}

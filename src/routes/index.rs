use crate::prelude::*;
use axum::{
    Json, Router,
    response::{Html, Redirect},
    routing::get,
};
use log::info;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(index, test, is_alive),
    tags((
        name = "index",
        description = "APIs that is on index."
    ))
)]
pub struct IndexApi;

/// # get_router
/// Adds route easily in `main.rs` file.
pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/test", get(test))
        .route("/is_alive", get(is_alive))
}

#[utoipa::path(
    get,
    tag = "index",
    path = "/",
    responses(
        (status = 302, description = "Redirect to /docs")
    )
)]
/// index
/// Redirects to /docs, which is swagger api document.
pub async fn index() -> Redirect {
    info!("redirect user to docs");
    Redirect::to("/docs")
}

#[utoipa::path(
    get,
    tag = "index",
    path = "/test",
    responses(
        (status = 200, body = String, description = "HTML response")
    )
)]
pub async fn test() -> Html<&'static str> {
    info!("Handling request.");
    Html("<p>백엔드에서 받은 응답!</p>")
}

#[utoipa::path(
    get,
    tag = "index",
    path = "/is_alive",
    responses(
        (status = 200, body = ApiResponse<String>, description = "JSON response")
    )
)]
pub async fn is_alive() -> Json<ApiResponse<String>> {
    info!("Server live test.");
    Json(ApiResponse {
        code: 200,
        resp: "ok".to_string(),
        data: "Server is alive!".to_string(),
    })
}

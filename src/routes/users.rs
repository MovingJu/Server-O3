use utoipa::OpenApi;
use axum::{
    Router,
    routing::get,
    response::Html
};
use log::info;

use crate::routes::RouterExt;

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

#[utoipa::path(
    get,
    tag = "users",
    path = "/get_users",
    responses(
        (status = 200, body = String, description = "HTML response.")
    )
)]
pub async fn get_users() -> Html<&'static str> {
    info!("Request to get users table.");
    Html("<p>user : movingju</p>")
}

#[utoipa::path(
    get,
    tag = "users",
    path = "/set_users",
    responses(
        (status = 200, body = String, description = "HTML response")
    )
)]
pub async fn set_users() -> Html<&'static str> {
    info!("Request to set users table.");
    Html("<p>setting complete.</p>")
}

use utoipa::OpenApi;
use axum::{
    Router,
    response::Redirect,
    routing::get,
    response::Html
};
use log::info;

// routes/users.rs

#[derive(OpenApi)]
#[openapi(
    paths(index, test),
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

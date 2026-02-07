use crate::prelude::*;
use aide::axum::{ApiRouter, routing::get};
use axum::{
    Json,
    response::{Html, Redirect},
};
use log::info;

/// # get_router
/// Adds route easily in `main.rs` file.
pub fn get_router() -> (Option<Tag>, ApiRouter) {
    (
        Some(Tag {
            name: "Index".to_string(),
            description: Some("Default features".to_string()),
            ..Default::default()
        }),
        ApiRouter::new()
            .api_route("/", get(index))
            .api_route("/test", get(test))
            .api_route("/is_alive", get(is_alive))
            .with_tag("Index"),
    )
}

/// index
/// Redirects to /docs, which is swagger api document.
pub async fn index() -> Redirect {
    info!("redirect user to docs");
    Redirect::to("/docs")
}

pub async fn test() -> Html<&'static str> {
    info!("Handling request.");
    Html("<p>백엔드에서 받은 응답!</p>")
}

pub async fn is_alive() -> Json<ApiResponse<String>> {
    info!("Server live test.");
    Json(ApiResponse {
        code: 200,
        resp: "ok".to_string(),
        data: "Server is alive!".to_string(),
    })
}

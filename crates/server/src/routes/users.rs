use aide::axum::{ApiRouter, routing::get};
use axum::{Json, extract::Query};
use log::info;
use schemars::JsonSchema;

use crate::prelude::*;

/// # get_router
/// Adds route easily in `main.rs` file.
pub fn get_router() -> (ApiRouter, Option<Tag>) {
    (
        ApiRouter::new()
            .api_route("/get_users", get(get_users))
            .api_route("/set_users", get(set_users))
            .with_prefix("/users")
            .with_tag("test"),
        Some(Tag {
            name: "test".to_string(),
            description: Some("testing routes".to_string()),
            ..Default::default()
        }),
    )
}

#[derive(Serialize, JsonSchema)]
pub struct UserResp {
    pub id: usize,
    pub name: String,
    pub email: Option<String>,
}

pub async fn get_users(Query(query): Query<GetUserQuery>) -> Json<ApiResponse<UserResp>> {
    info!("Request to get users table.");
    Json(ApiResponse {
        code: 200,
        resp: "ok".to_string(),
        data: UserResp {
            id: 1,
            name: query.name,
            email: None,
        },
    })
}
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetUserQuery {
    name: String,
}

pub async fn set_users() -> Json<ApiResponse<Empty>> {
    info!("Request to set users table.");
    Json(ApiResponse {
        code: 0,
        resp: "ok".to_string(),
        data: Empty,
    })
}

#![allow(dead_code)]

//! # database
//! module testing my database

use aide::axum::{ApiRouter, routing::get};
use axum::{
    Json,
    extract::{Query, State},
};
use log::error;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    prelude::*,
    repository::{Repo, RepoFactory, posts::Posts, users::Users},
};

pub fn get_router(state: Arc<RepoFactory>) -> (Option<Tag>, ApiRouter) {
    (
        Some(Tag {
            name: "database".to_string(),
            description: Some("APIs for manipulating database".to_string()),
            ..Default::default()
        }),
        ApiRouter::new()
            .api_route("/get_user", get(get_user))
            .api_route("/set_user", get(set_user))
            .with_state(state)
            .with_prefix("/db")
            .with_tag("database"),
    )
}

pub async fn get_user(
    State(state): State<Arc<RepoFactory>>,
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
#[derive(Deserialize, JsonSchema)]
pub struct GetUserQuery {
    name: String,
}

pub async fn set_user(
    State(state): State<Arc<RepoFactory>>,
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
#[derive(Deserialize, JsonSchema)]
pub struct SetUserQuery {
    name: String,
}

pub async fn get_post(
    State(state): State<std::sync::Arc<RepoFactory>>,
    Query(_query): Query<SetUserQuery>,
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
#[derive(Deserialize, JsonSchema)]
pub struct GetPostQuery {
    user_id: i64,
    title: Option<String>,
}
#[derive(Serialize, JsonSchema)]
pub struct GetPostResponse {
    id: i64,
    title: String,
    content: String,
    user_id: i64,
}

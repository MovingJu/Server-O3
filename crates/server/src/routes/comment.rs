use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use log::{error, info};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{prelude::*, repository::RepoFactory};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

fn extract_username(headers: &HeaderMap) -> Option<String> {
    let value = headers.get("authorization")?.to_str().ok()?;
    let token = value.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "changeme".to_string());
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .ok()?;
    Some(data.claims.sub)
}

pub fn get_router(state: Arc<RepoFactory>) -> (Option<Tag>, ApiRouter) {
    (
        Some(Tag {
            name: "comments".to_string(),
            description: Some("Post comments".to_string()),
            ..Default::default()
        }),
        ApiRouter::new()
            .api_route("/comments", get_with(get_comments, |op| op).post_with(post_comment, |op| op))
            .with_state(state)
            .with_tag("comments"),
    )
}

#[derive(Deserialize, JsonSchema)]
pub struct GetCommentsQuery {
    post_id: String,
}

pub async fn get_comments(
    State(state): State<Arc<RepoFactory>>,
    Query(q): Query<GetCommentsQuery>,
) -> (StatusCode, Json<Value>) {
    match state.comment.get_by_post(&q.post_id).await {
        Ok(comments) => (StatusCode::OK, Json(json!(comments))),
        Err(err) => {
            error!("DB error fetching comments: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Server Error" })))
        }
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct PostCommentBody {
    post_id: String,
    content: String,
}

pub async fn post_comment(
    State(state): State<Arc<RepoFactory>>,
    headers: HeaderMap,
    Json(body): Json<PostCommentBody>,
) -> (StatusCode, Json<Value>) {
    let username = match extract_username(&headers) {
        Some(u) => u,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({ "message": "Login required" }))),
    };

    let content = body.content.trim().to_string();
    if content.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({ "message": "Content is empty" })));
    }

    match state.comment.insert(&body.post_id, &username, &content).await {
        Ok(()) => {
            info!("comment posted by '{}' on '{}'", username, body.post_id);
            (StatusCode::OK, Json(json!({})))
        }
        Err(err) => {
            error!("DB error posting comment: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Server Error" })))
        }
    }
}

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
        None,
        ApiRouter::new()
            .api_route("/reactions", get_with(get_reactions, |op| op).post_with(toggle_reaction, |op| op))
            .with_state(state),
    )
}

#[derive(Deserialize, JsonSchema)]
pub struct GetReactionsQuery {
    post_id: String,
}

pub async fn get_reactions(
    State(state): State<Arc<RepoFactory>>,
    headers: HeaderMap,
    Query(q): Query<GetReactionsQuery>,
) -> (StatusCode, Json<Value>) {
    let counts = match state.reaction.get_counts(&q.post_id).await {
        Ok(c) => c,
        Err(err) => {
            error!("DB error fetching reactions: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Server Error" })));
        }
    };

    let mine: Vec<String> = if let Some(username) = extract_username(&headers) {
        state.reaction.get_user_reactions(&q.post_id, &username).await.unwrap_or_default()
    } else {
        vec![]
    };

    (StatusCode::OK, Json(json!({ "counts": counts, "mine": mine })))
}

#[derive(Deserialize, JsonSchema)]
pub struct ToggleReactionBody {
    post_id: String,
    reaction: String,
}

const ALLOWED_REACTIONS: &[&str] = &["like", "heart", "tada", "thinking", "sad"];

pub async fn toggle_reaction(
    State(state): State<Arc<RepoFactory>>,
    headers: HeaderMap,
    Json(body): Json<ToggleReactionBody>,
) -> (StatusCode, Json<Value>) {
    let username = match extract_username(&headers) {
        Some(u) => u,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({ "message": "Login required" }))),
    };

    if !ALLOWED_REACTIONS.contains(&body.reaction.as_str()) {
        return (StatusCode::BAD_REQUEST, Json(json!({ "message": "Invalid reaction" })));
    }

    match state.reaction.toggle(&body.post_id, &username, &body.reaction).await {
        Ok(added) => {
            info!("reaction '{}' {} by '{}' on '{}'", body.reaction, if added { "added" } else { "removed" }, username, body.post_id);
            (StatusCode::OK, Json(json!({ "added": added })))
        }
        Err(err) => {
            error!("DB error toggling reaction: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": "Server Error" })))
        }
    }
}

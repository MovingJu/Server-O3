use aide::axum::{ApiRouter, routing::get_with};
use axum::{Json, extract::Query};
use log::{error, info};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    prelude::*,
    services::{fibo, hanoi},
};

/// # get_router
/// Adds route easily in `main.rs` file.
pub fn get_router() -> (ApiRouter, Option<Tag>) {
    (
        ApiRouter::new()
            .api_route("/fibo", get_with(fibo, |op| op.tag("calc")))
            .api_route("/hanoi", get_with(hanoi, |op| op.tag("calc")))
            .with_prefix("/calc"),
        Some(Tag {
            name: "calc".to_string(),
            description: Some("API for custom calculations".to_string()),
            ..Default::default()
        }),
    )
}

/// # API for calculating n'th Fibonacci number
pub async fn fibo(Query(query): Query<FiboQuery>) -> Json<ApiResponse<String>> {
    info!("user requests fibonacci {}'th number", query.n);
    if query.n > 5_000 {
        Json(ApiResponse {
            code: 1,
            resp: "Bad Request".to_string(),
            data: "The number is too big! (>5,000)".to_string(),
        })
    } else {
        Json(ApiResponse {
            code: 0,
            resp: "ok".to_string(),
            data: fibo::calc_fibo_rec(query.n).to_string(),
        })
    }
}
#[derive(Deserialize, JsonSchema)]
pub struct FiboQuery {
    n: usize,
}

/// # API for calculating n'th Hanoi's tower
pub async fn hanoi(Query(query): Query<HanoiQuery>) -> Json<ApiResponse<HanoiResponse>> {
    let mut res_default = ApiResponse::<HanoiResponse>::default();
    info!("user requests hanoi {}'th squence", query.n);
    if query.n < 15 {
        res_default = match hanoi::calc_hanoi_rec(query.n).await {
            Ok(res) => res_default
                .code(0)
                .resp("ok".to_string())
                .data(HanoiResponse {
                    num_replacement: res.len().to_string(),
                    orders: Some(res),
                }),
            Err(err) => {
                error!("Thread join error occur!: {}", err);
                res_default
                    .code(-1)
                    .resp("Thread join Error occur!".to_string())
            }
        };
    } else if query.n < 10_000_000 {
        let num_replacement = hanoi::calc_hanoi_num(query.n).await;
        res_default = match num_replacement {
            Ok(v) => res_default
                .code(1)
                .resp("Input number exceeds order calculation limit (14)".to_string())
                .data(HanoiResponse {
                    num_replacement: v.to_string(),
                    ..Default::default()
                }),
            Err(err) => {
                error!("Thread join error occur!: {}", err);
                res_default
                    .code(-1)
                    .resp("Thread join Error occur!".to_string())
            }
        };
    } else {
        res_default = res_default
            .code(2)
            .resp("Input number exceeds limit (10,000,000)".to_string());
    }
    Json(res_default)
}
#[derive(Deserialize, JsonSchema)]
pub struct HanoiQuery {
    n: usize,
}
#[derive(Serialize, JsonSchema, Default)]
pub struct HanoiResponse {
    num_replacement: String,
    orders: Option<Vec<(u8, u8)>>,
}

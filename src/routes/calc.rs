use axum::{Json, Router, extract::Query, routing::get};
use log::{error, info};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};

use crate::{
    prelude::*,
    services::{fibo, hanoi},
};

#[derive(OpenApi)]
#[openapi(
    paths(fibo, hanoi),
    tags((
        name = "calc",
        description = "APIs for custom calculation."
    ))
)]
pub struct CalcApi;
/// # get_router
/// Adds route easily in `main.rs` file.
pub fn get_router() -> Router {
    Router::new()
        .route("/fibo", get(fibo))
        .route("/hanoi", get(hanoi))
        .with_prefix("/calc")
}

/// # API for calculating n'th Fibonacci number
#[utoipa::path(
    get,
    tag = "calc",
    path = "/fibo",
    params(FiboQuery),
    responses(
        (status = 0, body = ApiResponse<String>, description = "Calculates Nth fibonacci number."),
        (status = 1, body = ApiResponse<String>, description = "Input number exceeds the maximum limit (5,000)")
    )
)]
pub async fn fibo(Query(query): Query<FiboQuery>) -> Json<ApiResponse<String>> {
    if query.n > 5_000 {
        Json(ApiResponse {
            code: 1,
            resp: "Bad Request".to_string(),
            data: "The number is too big! (>5,000)".to_string(),
        })
    } else {
        info!("user requests fibonacci {}'th number", query.n);
        Json(ApiResponse {
            code: 0,
            resp: "ok".to_string(),
            data: fibo::calc_fibo_rec(query.n).to_string(),
        })
    }
}
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct FiboQuery {
    n: usize,
}

/// # API for calculating n'th Hanoi's tower
#[utoipa::path(
    get,
    tag = "calc",
    path = "/hanoi",
    params(HanoiQuery),
    responses(
        (status = 0, body = ApiResponse<HanoiResponse>, description = "ok"),
        (status = 1, body = ApiResponse<HanoiResponse>, description = "Input number exceeds order calculation limit (14)"),
        (status = 2, body = ApiResponse<HanoiResponse>, description = "Input number exceeds limit (10,000,000)"),
        (status = -1, body = ApiResponse<HanoiResponse>, description = "Thread join Error occur!")
    )
)]
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
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct HanoiQuery {
    n: usize,
}
#[derive(Serialize, ToSchema, Default)]
pub struct HanoiResponse {
    num_replacement: String,
    orders: Option<Vec<(u8, u8)>>,
}

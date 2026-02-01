use axum::{Json, Router, extract::Query, routing::get};
use log::{error, info};
use num::{BigUint, pow::pow};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};

use crate::prelude::*;

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
            data: calc_fibo::calc_fibo_rec(query.n).to_string(),
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
        (status = 2, body = ApiResponse<HanoiResponse>, description = "Thread join Error occur!")
    )
)]
pub async fn hanoi(Query(query): Query<HanoiQuery>) -> Json<ApiResponse<HanoiResponse>> {
    if query.num_cell < 15 {
        match calc_hanoi::calc_hanoi_rec(query.num_cell).await {
            Ok(res) => Json(ApiResponse {
                code: 0,
                resp: "ok".to_string(),
                data: HanoiResponse {
                    num_replacement: res.len().to_string(),
                    orders: Some(res),
                },
            }),
            Err(err) => {
                error!("Thread join error occur!: {}", err);
                Json(ApiResponse {
                    code: 2,
                    resp: "Thread join Error occur!".to_string(),
                    data: HanoiResponse::default(),
                })
            }
        }
    } else {
        let num_replacement =
            (pow(BigUint::from(2usize), query.num_cell) - BigUint::from(1usize)).to_string();
        Json(ApiResponse {
            code: 1,
            resp: "Input number exceeds order calculation limit (14)".to_string(),
            data: HanoiResponse {
                num_replacement, ..Default::default()
            },
        })
    }
}
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct HanoiQuery {
    num_cell: usize,
}
#[derive(Serialize, ToSchema, Default)]
pub struct HanoiResponse {
    num_replacement: String,
    orders: Option<Vec<(u8, u8)>>,
}

mod calc_fibo {
    use lazy_static::lazy_static;
    use num::BigUint;
    use std::collections::HashMap;
    use std::sync::RwLock;

    lazy_static! {
        static ref FIBO_CACHE: RwLock<HashMap<usize, BigUint>> = RwLock::new(HashMap::new());
    }
    pub fn calc_fibo_rec(n: usize) -> BigUint {
        {
            if let Some(res) = FIBO_CACHE
                .read()
                .ok()
                .and_then(|cache| cache.get(&n).cloned())
            {
                return res;
            }
        }
        if n == 0 || n == 1 {
            BigUint::from(n)
        } else {
            let res = calc_fibo_rec(n - 1) + calc_fibo_rec(n - 2);
            {
                if let Ok(mut cache) = FIBO_CACHE.write() {
                    cache.insert(n, res.clone());
                }
            }
            res
        }
    }
}

mod calc_hanoi {
    use tokio::task;

    pub async fn calc_hanoi_rec(num_cell: usize) -> Result<Vec<(u8, u8)>, tokio::task::JoinError> {
        task::spawn_blocking(move || {
            let mut orders: Vec<(u8, u8)> = Vec::new();
            calc_hanoi_inner_(num_cell, 1, 3, 2, &mut orders);
            Ok(orders)
        })
        .await?
    }
    fn calc_hanoi_inner_(num_cell: usize, from: u8, to: u8, via: u8, res_vec: &mut Vec<(u8, u8)>) {
        if num_cell == 1 {
            res_vec.push((from, to));
        } else {
            calc_hanoi_inner_(num_cell - 1, from, via, to, res_vec);
            res_vec.push((from, to));
            calc_hanoi_inner_(num_cell - 1, via, to, from, res_vec);
        }
    }
}

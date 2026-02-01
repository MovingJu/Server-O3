use axum::{Json, Router, extract::Query, routing::get};
use log::info;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};

use crate::prelude::*;

#[derive(OpenApi)]
#[openapi(
    paths(fibo),
    tags((
        name = "calc",
        description = "APIs for custom calculation."
    ))
)]
pub struct CalcApi;
/// # get_router
/// Adds route easily in `main.rs` file.
pub fn get_router() -> Router {
    Router::new().route("/fibo", get(fibo)).with_prefix("/calc")
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct UBig(pub BigUint);
impl Serialize for UBig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}
// impl ToSchema for UBig {
//     fn name() -> std::borrow::Cow<'static, str> {
//         std::borrow::Cow::Borrowed("UBig")
//     }

//     fn schemas(
//         schemas: &mut Vec<(
//             String,
//             RefOr<Schema>,
//         )>,
//     ) {
//         // OpenAPI에서 string으로 표시
//         schemas.push((
//             Self::name().into_owned(),
//             RefOr::Inline(Schema::new(utoipa::openapi::schema::SchemaType::String)),
//         ));
//     }
// }

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct FiboQuery {
    n: usize,
}

#[utoipa::path(
    get,
    tag = "calc",
    path = "/fibo",
    params(FiboQuery),
    responses(
        (status = 0, body = ApiResponse<String>, description = "Calculates Nth fibonacci number."),
        (status = 1, body = ApiResponse<String>, description = "Number user puts is too big!")
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

mod calc_fibo {
    use lazy_static::lazy_static;
    use num_bigint::BigUint;
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

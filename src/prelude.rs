use axum::Router;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
pub trait RouterExt {
    fn with_prefix(self, prefix: &str) -> Self;
}

impl RouterExt for Router {
    fn with_prefix(self, prefix: &str) -> Self {
        Router::new().nest(prefix, self)
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T>
where
    T: ToSchema,
{
    pub code: u16,
    pub resp: String,
    pub data: T,
}

#[derive(Serialize, ToSchema)]
/// # Empty
/// Describes `null` state for compiler to understand.
/// ## How to use
/// ```
/// let response = ApiResponse<Empty> {
///     code: 0,
///     resp: "ok".to_string(),
///     data: Empty
/// };
/// ```
pub struct Empty;

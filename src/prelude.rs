use axum::Router;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// # RouterExt
/// implemets with_prefix to easily add prefix with string slice.
pub trait RouterExt {
    fn with_prefix(self, prefix: &str) -> Self;
}

impl RouterExt for Router {
    fn with_prefix(self, prefix: &str) -> Self {
        Router::new().nest(prefix, self)
    }
}

#[derive(Serialize, Deserialize, ToSchema, Default, Clone)]
pub struct ApiResponse<T>
where
    T: ToSchema,
{
    pub code: isize,
    pub resp: String,
    pub data: T,
}
impl<T: ToSchema> ApiResponse<T> {
    pub fn code(mut self, code: isize) -> Self {
        self.code = code;
        self
    }
    pub fn resp(mut self, resp: String) -> Self {
        self.resp = resp;
        self
    }
    pub fn data(mut self, data: T) -> Self {
        self.data = data;
        self
    }
}

#[derive(Serialize, Deserialize, ToSchema, Default, Clone)]
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

pub use aide::axum::ApiRouter;
pub use schemars::JsonSchema;
pub use serde::{Deserialize, Serialize};
pub use std::sync::Arc;

/// # RouterExt
/// implemets with_prefix to easily add prefix with string slice.
pub trait RouterExt {
    fn with_prefix(self, prefix: &str) -> Self;
}
impl RouterExt for ApiRouter {
    fn with_prefix(self, prefix: &str) -> Self {
        ApiRouter::new().nest(prefix, self)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Default, Clone)]
pub struct ApiResponse<T>
where
    T: JsonSchema,
{
    pub code: isize,
    pub resp: String,
    pub data: T,
}
impl<T: JsonSchema> ApiResponse<T> {
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

#[derive(Serialize, Deserialize, JsonSchema, Default, Clone)]
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

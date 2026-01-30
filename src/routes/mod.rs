pub mod index;
pub mod users;


use axum::Router;
pub trait RouterExt {
    fn with_prefix(self, prefix: &str) -> Self;
}
impl RouterExt for Router {
    fn with_prefix(self, prefix: &str) -> Self {
        Router::new().nest(prefix, self)
    }
}
pub mod calc;
pub mod database;
pub mod index;
pub mod users;

pub mod apis {
    use axum::Router;
    use utoipa::OpenApi;
    
    use crate::services::sql_pool::AppState;
    use super::*;

    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "movingju.com API",
            version = "1.1.0",
            description = "My Public APIs"
        ),
        nest(
            (path = "/users", api = users::UsersApi),
            (path = "/calc", api = calc::CalcApi),
            (path = "/db", api = database::DatabaseApi)
        )
    )]
    pub struct ApiDoc;

    pub fn get_router(state: std::sync::Arc<AppState>) -> Router {
        Router::new()
            .merge(users::get_router())
            .merge(calc::get_router())
            .merge(database::get_router(state))
    }
}

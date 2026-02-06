pub mod calc;
pub mod database;
pub mod index;
pub mod users;

pub mod apis {
    use std::sync::Arc;
    use aide::swagger::Swagger;
    use aide::{
        axum::{
            ApiRouter,
            IntoApiResponse,
            routing::{get, get_with},
        },
        openapi::OpenApi,
        redoc::Redoc,
        scalar::Scalar,
    };
    use axum::{Extension, Json, response::IntoResponse};

    use crate::repository::RepoFactory;
    use super::*;

    pub fn get_router(state: std::sync::Arc<RepoFactory>) -> ApiRouter {
        ApiRouter::new()
            .merge(users::get_router())
            .merge(calc::get_router())
            .merge(database::get_router(state))
    }

    pub fn docs_routes(state: Arc<RepoFactory>) -> ApiRouter {
        // We infer the return types for these routes
        // as an example.
        //
        // As a result, the `serve_redoc` route will
        // have the `text/html` content-type correctly set
        // with a 200 status.
        aide::generate::infer_responses(true);
        const DOC_TITLE: &str = "api.movingju.com";

        let router: ApiRouter = ApiRouter::new()
            .route(
                "/",
                get_with(
                    Scalar::new("/docs/openapi.json")
                        .with_title(DOC_TITLE)
                        .axum_handler(),
                    |op| op.description("This documentation page."),
                ),
            )
            .route(
                "/redoc",
                get_with(
                    Redoc::new("/docs/openapi.json")
                        .with_title(DOC_TITLE)
                        .axum_handler(),
                    |op| op.description("This documentation page."),
                ),
            )
            .route(
                "/swagger",
                get_with(
                    Swagger::new("/docs/openapi.json")
                        .with_title(DOC_TITLE)
                        .axum_handler(),
                    |op| op.description("This documentation page."),
                ),
            )
            .route("/openapi.json", get(serve_docs))
            .with_state(state);

        // Afterwards we disable response inference because
        // it might be incorrect for other routes.
        aide::generate::infer_responses(false);

        router
    }

    async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
        Json(api).into_response()
    }
}

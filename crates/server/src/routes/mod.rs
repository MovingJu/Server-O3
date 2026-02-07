pub mod calc;
pub mod database;
pub mod index;
pub mod users;


pub mod apis {
    use aide::{axum::ApiRouter, openapi::OpenApi};
    use std::sync::Arc;

    pub fn route_settings(state: Arc<RepoFactory>) -> (ApiRouter, OpenApi) {
        [
            // Add routes here
            index::get_router(),
            calc::get_router(),
            database::get_router(state),
            users::get_router(),
        ]
        .into_iter()
        .fold(
            (ApiRouter::new(), OpenApi::default()),
            |(app, mut api), route| {
                match route.0 {
                    Some(v) => api.tags.push(v),
                    None => (),
                };
                (app.merge(route.1), api)
            },
        )
    }

    use aide::swagger::Swagger;
    use aide::{
        axum::{
            IntoApiResponse,
            routing::{get, get_with},
        },
        redoc::Redoc,
        scalar::Scalar,
    };
    use axum::{Extension, Json, response::IntoResponse};

    use super::*;
    use crate::repository::RepoFactory;

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

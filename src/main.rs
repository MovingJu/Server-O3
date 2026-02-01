use anyhow::Result;
use axum::Router;
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod prelude;
mod routes;
mod services;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "movingju.com API",
        version = "1.0.0",
        description = "My Public APIs"
    ),
    nest(
        (path = "/users", api = routes::users::UsersApi),
        (path = "/calc", api = routes::calc::CalcApi)
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging first
    init_logger();

    // Merge all OpenAPI documents
    let mut openapi = ApiDoc::openapi();
    openapi.merge(routes::index::IndexApi::openapi());

    // Build application with all routes
    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", openapi))
        .merge(routes::index::get_router())
        .merge(routes::users::get_router())
        .merge(routes::calc::get_router());

    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:8080").await?;

    info!("Server listening on http://0.0.0.0:8080");
    axum::serve(listener, app)
        .with_graceful_shutdown(wait_for_signal())
        .await?;
    info!("Server closed gracefully");

    Ok(())
}

/// Initialize logger based on build mode
fn init_logger() {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter(None, log::LevelFilter::Info)
            .init();
    }
}

/// Signal handler for graceful shutdown
/// Essential for container environments
async fn wait_for_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sigint = signal(SignalKind::interrupt()).unwrap();

        tokio::select! {
            _ = sigterm.recv() => info!("Received SIGTERM"),
            _ = sigint.recv() => info!("Received SIGINT"),
        }
    }

    #[cfg(windows)]
    {
        use tokio::signal;
        let _ = signal::ctrl_c().await;
        info!("Received Ctrl+C");
    }
}

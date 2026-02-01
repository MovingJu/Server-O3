use anyhow::Result;
use axum::Router;
use log::{error, info, debug};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod prelude;
mod repository;
mod services;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize log level
    init_logger();

    // Merge all OpenAPI documents
    let mut openapi = routes::index::IndexApi::openapi();
    openapi.merge(routes::apis::ApiDoc::openapi());

    // Load database
    debug!("Loading env paths");
    dotenv::dotenv().ok();
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(v) => v,
        Err(err) => {
            error!("Fail to load DATABASE_URL from .env : {}", err);
            panic!()
        }
    };
    debug!("Complete to load DATABASE_URL");
    let pool = PgPool::connect(&database_url).await?;
    let state = std::sync::Arc::new(services::sql_pool::AppState { pool });
    debug!("Succesfully connect to Database");

    // Build application with all routes
    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", openapi))
        .merge(routes::index::get_router())
        .merge(routes::apis::get_router(state));
    run_server(app).await?;

    Ok(())
}

async fn run_server(app: Router) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:8080").await?;
    info!("Server listening on http://0.0.0.0:8080");
    match axum::serve(listener, app)
        .with_graceful_shutdown(wait_for_signal())
        .await
    {
        Ok(..) => (),
        Err(err) => error!("Error occur while shutting down : {}", err),
    };
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
            _ = sigint.recv() => {println!(); info!("Received SIGINT")},
        }
    }

    #[cfg(windows)]
    {
        use tokio::signal;
        let _ = signal::ctrl_c().await;
        println!("-----------------");
        info!("Received Ctrl+C");
    }
}

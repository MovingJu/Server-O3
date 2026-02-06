use aide::{
    axum::{IntoApiResponse, routing::get},
    openapi::{OpenApi, Tag},
    transform::TransformOpenApi,
};
use anyhow::Result;
use axum::{Extension, Json};
use log::{debug, error, info};
use sqlx::PgPool;

use prelude::*;

mod prelude;
mod repository;
mod routes;
mod services;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize log level
    init_logger();

    // Load database
    debug!("Loading env variables");
    dotenv::dotenv().ok();
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(v) => v,
        Err(err) => {
            error!("Fail to load DATABASE_URL from .env : {}", err);
            panic!()
        }
    };
    debug!("Complete to load variable DATABASE_URL");
    let pool = PgPool::connect(&database_url).await?;
    let state = Arc::new(repository::RepoFactory::new(pool));
    debug!("Succesfully connect to Database");

    let api = OpenApi::default();
    // Build application with all routes
    let app = ApiRouter::new()
        .merge(routes::index::get_router())
        .merge(routes::apis::get_router(state.clone()))
        .nest_api_service("/docs", routes::apis::docs_routes(state.clone()))
        .route("/full_api.json", get(serve_api));
    run_server(app, api).await?;

    Ok(())
}

async fn run_server(app: ApiRouter, mut api: OpenApi) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:8080").await?;
    info!("Server listening on http://0.0.0.0:8080");
    match axum::serve(
        listener,
        app.finish_api_with(&mut api, api_docs)
            .layer(Extension(std::sync::Arc::new(api)))
            .into_make_service(),
    )
    .with_graceful_shutdown(wait_for_signal())
    .await
    {
        Ok(..) => (),
        Err(err) => error!("Error occur while shutting down : {}", err),
    };
    info!("Server closed gracefully");
    Ok(())
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("api.movingju.com")
        .summary("My public APIs")
        .tag(Tag {
            name: "calc".to_string(),
            description: Some("Custom calculation modules".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "database".to_string(),
            description: Some("Database manipulation modules".to_string()),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
    // .description(include_str!("README.md"))
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
        println!();
        info!("Received Ctrl+C");
    }
}

// Note that this clones the document on each request.
// To be more efficient, we could wrap it into an Arc,
// or even store it as a serialized string.
async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

use axum::Router;
use tower::ServiceBuilder;
use tower_http::{
    trace::TraceLayer,
    catch_panic::CatchPanicLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::db;
use crate::routes;

pub fn init_tracing(log_level: &str) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub async fn create_app(database_url: &str) -> Result<Router, Box<dyn std::error::Error>> {
    let db = db::create_connection(database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    let router = routes::create_router(db);

    Ok(router.layer(
        ServiceBuilder::new()
            .layer(CatchPanicLayer::new())
            .layer(TraceLayer::new_for_http())
            .into_inner(),
    ))
}

pub async fn run_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    init_tracing(&config.log_level);

    let app = create_app(&config.database_url).await?;
    let address = config.address();

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .map_err(|e| format!("Failed to bind to address {}: {}", address, e))?;

    tracing::info!("Server running on http://{}", address);
    tracing::info!("Health check available at http://{}/health", address);

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Server failed to start: {}", e))?;

    Ok(())
}

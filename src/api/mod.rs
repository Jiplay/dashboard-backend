pub mod handlers;
pub mod routes;

use crate::config::DashboardConfig;
use axum::Router;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub dashboard: Arc<crate::Dashboard>,
}

/// Create and configure the API router
pub fn create_router(config: &DashboardConfig) -> Router {
    let dashboard = Arc::new(crate::Dashboard::new());
    let state = AppState { dashboard };

    let cors = if config.server.enable_cors {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        CorsLayer::permissive()
    };

    Router::new()
        .nest("/api/v1", routes::create_routes())
        .layer(cors)
        .with_state(state)
}

/// Start the API server
pub async fn start_server(config: DashboardConfig) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting dashboard server on {}", addr);

    let app = create_router(&config);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

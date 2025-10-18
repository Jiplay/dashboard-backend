// Module declarations
// These tell Rust to include these files as modules
mod handlers;
mod models;
mod routes;
mod state;

use routes::create_router;
use state::AppState;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Main entry point
///
/// #[tokio::main]: Macro that sets up the async runtime
/// async: This function can use .await for async operations
#[tokio::main]
async fn main() {
    // Initialize tracing (logging)
    // This sets up structured logging for debugging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_api_example=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create application state
    let state = AppState::new();

    // Create router with all routes and middleware
    let app = create_router(state)
        // Add tracing layer for request logging
        .layer(TraceLayer::new_for_http());

    // Define server address
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("🚀 Server running on http://{}", addr);

    // Start the server
    // serve(): Runs the server until shutdown
    axum::serve(listener, app).await.unwrap();
}

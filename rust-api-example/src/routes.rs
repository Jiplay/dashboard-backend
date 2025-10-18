use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers::{create_user, delete_user, get_user, get_users, health_check, update_user};
use crate::state::AppState;

/// Creates the application router with all routes
///
/// Router: Axum's routing table
/// with_state: Attaches the application state to all routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        // User CRUD endpoints
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        // Attach shared state to all routes
        .with_state(state)
}

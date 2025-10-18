use axum::Router;
use crate::routes;

pub fn init_router() -> Router {
    Router::new()
        .merge(routes::services::init_services())
        // Add more route modules here as needed
}

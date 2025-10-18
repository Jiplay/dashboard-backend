use axum::Router;
use axum::routing::{get, post};
use crate::controllers::services;

pub fn init_services() -> Router {
    Router::new()
        .route("/services", get(services::get_services))
        .route("/services", post(services::post_services))
}

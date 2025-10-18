use axum::{Json, http::StatusCode};
use crate::models::services::{CreateService, Service, ServiceResponse};

pub async fn get_services() -> &'static str {
    "get services"
}

pub async fn post_services(Json(create_data): Json<CreateService>)
    -> Result<Json<ServiceResponse>, StatusCode> {
    let service = Service::new(create_data);
    println!("{:?}", service);
    Ok(Json(service.to_response()))
}

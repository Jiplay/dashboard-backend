use crate::api::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "dashboard-backend"
    }))
}

/// Get all metrics
pub async fn get_all_metrics(State(state): State<AppState>) -> impl IntoResponse {
    match state.dashboard.collect_all().await {
        Ok(metrics) => (StatusCode::OK, Json(metrics)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// Get CPU metrics
#[cfg(feature = "cpu")]
pub async fn get_cpu_metrics(State(state): State<AppState>) -> impl IntoResponse {
    match state.dashboard.collect_cpu().await {
        Ok(metrics) => (StatusCode::OK, Json(metrics)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// Get memory metrics
#[cfg(feature = "memory")]
pub async fn get_memory_metrics(State(state): State<AppState>) -> impl IntoResponse {
    match state.dashboard.collect_memory().await {
        Ok(metrics) => (StatusCode::OK, Json(metrics)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// Get disk metrics
#[cfg(feature = "disk")]
pub async fn get_disk_metrics(State(state): State<AppState>) -> impl IntoResponse {
    match state.dashboard.collect_disk().await {
        Ok(metrics) => (StatusCode::OK, Json(metrics)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// Get network metrics
#[cfg(feature = "network")]
pub async fn get_network_metrics(State(state): State<AppState>) -> impl IntoResponse {
    match state.dashboard.collect_network().await {
        Ok(metrics) => (StatusCode::OK, Json(metrics)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// Get system metrics
#[cfg(feature = "system")]
pub async fn get_system_metrics(State(state): State<AppState>) -> impl IntoResponse {
    match state.dashboard.collect_system().await {
        Ok(metrics) => (StatusCode::OK, Json(metrics)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

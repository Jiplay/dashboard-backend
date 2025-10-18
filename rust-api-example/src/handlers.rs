use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::models::{ApiResponse, CreateUserRequest, UpdateUserRequest, User};
use crate::state::AppState;

/// Handler: GET /health
///
/// Simple health check endpoint
pub async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("API is running!".to_string()))
}

/// Handler: GET /users
///
/// Returns all users
///
/// State: Axum automatically injects the shared application state
/// Json: Automatically serializes the response to JSON
pub async fn get_users(State(state): State<AppState>) -> Json<ApiResponse<Vec<User>>> {
    // .read() acquires a read lock (allows multiple concurrent readers)
    let users = state.users.read().await;

    // Convert HashMap values to a Vec
    let users_list: Vec<User> = users.values().cloned().collect();

    Json(ApiResponse::success(users_list))
}

/// Handler: GET /users/:id
///
/// Returns a single user by ID
///
/// Path: Extracts the ID from the URL path
/// Result: Returns either success (200) or not found (404)
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    let users = state.users.read().await;

    match users.get(&id) {
        Some(user) => Ok(Json(ApiResponse::success(user.clone()))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Handler: POST /users
///
/// Creates a new user
///
/// Json<T>: Automatically deserializes request body from JSON
/// StatusCode::CREATED: Returns 201 status for successful creation
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<ApiResponse<User>>) {
    let new_user = User {
        id: Uuid::new_v4(), // Generate a new UUID
        name: payload.name,
        email: payload.email,
        age: payload.age,
    };

    // .write() acquires a write lock (exclusive access)
    let mut users = state.users.write().await;
    users.insert(new_user.id, new_user.clone());

    (StatusCode::CREATED, Json(ApiResponse::success(new_user)))
}

/// Handler: PUT /users/:id
///
/// Updates an existing user
///
/// Supports partial updates - only provided fields are updated
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    let mut users = state.users.write().await;

    match users.get_mut(&id) {
        Some(user) => {
            // Update only the fields that are provided
            if let Some(name) = payload.name {
                user.name = name;
            }
            if let Some(email) = payload.email {
                user.email = email;
            }
            if let Some(age) = payload.age {
                user.age = age;
            }

            Ok(Json(ApiResponse::success(user.clone())))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Handler: DELETE /users/:id
///
/// Deletes a user by ID
///
/// Returns 204 No Content on success
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut users = state.users.write().await;

    match users.remove(&id) {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND,
    }
}

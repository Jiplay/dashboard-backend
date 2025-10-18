use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a User in the system
///
/// Derives:
/// - Serialize/Deserialize: Converts to/from JSON automatically
/// - Clone: Allows copying the struct
/// - Debug: Enables printing with {:?}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub age: u32,
}

/// Request body for creating a new user
///
/// Note: No 'id' field - server generates it
#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub age: u32,
}

/// Request body for updating a user
///
/// All fields are optional - allows partial updates
#[derive(Deserialize, Debug)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<u32>,
}

/// Generic API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

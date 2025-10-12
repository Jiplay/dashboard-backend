use thiserror::Error;

/// Main error type for the dashboard backend
#[derive(Error, Debug)]
pub enum DashboardError {
    #[error("Failed to collect metric: {0}")]
    CollectionError(String),

    #[error("System information unavailable: {0}")]
    SystemInfoError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[cfg(feature = "api-server")]
    #[error("API error: {0}")]
    ApiError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias for dashboard operations
pub type DashboardResult<T> = Result<T, DashboardError>;

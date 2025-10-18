use crate::models::User;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Application state shared across all handlers
///
/// Arc (Atomic Reference Count): Enables shared ownership across threads
/// RwLock: Allows multiple readers OR one writer (thread-safe)
/// HashMap: In-memory storage (in production, use a real database)
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

use crate::api::handlers;
use axum::{routing::get, Router};

use super::AppState;

/// Create all API routes
pub fn create_routes() -> Router<AppState> {
    let mut router = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/all", get(handlers::get_all_metrics));

    #[cfg(feature = "cpu")]
    {
        router = router.route("/cpu", get(handlers::get_cpu_metrics));
    }

    #[cfg(feature = "memory")]
    {
        router = router.route("/memory", get(handlers::get_memory_metrics));
    }

    #[cfg(feature = "disk")]
    {
        router = router.route("/disk", get(handlers::get_disk_metrics));
    }

    #[cfg(feature = "network")]
    {
        router = router.route("/network", get(handlers::get_network_metrics));
    }

    #[cfg(feature = "system")]
    {
        router = router.route("/system", get(handlers::get_system_metrics));
    }

    router
}

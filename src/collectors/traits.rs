use crate::models::error::DashboardResult;
use async_trait::async_trait;
use serde::Serialize;

/// Trait that all metric collectors must implement
#[async_trait]
pub trait MetricCollector: Send + Sync {
    /// The output type for this collector
    type Output: Serialize + Send;

    /// Collect the metrics
    async fn collect(&self) -> DashboardResult<Self::Output>;

    /// Get the name of this collector
    fn name(&self) -> &'static str;

    /// Check if this collector is enabled
    fn is_enabled(&self) -> bool {
        true
    }
}

/// Trait for collectors that can be refreshed periodically
#[async_trait]
pub trait RefreshableCollector: MetricCollector {
    /// Refresh the internal state of the collector
    async fn refresh(&mut self) -> DashboardResult<()>;
}

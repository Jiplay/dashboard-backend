use crate::collectors::traits::MetricCollector;
use crate::models::error::DashboardResult;
use crate::models::memory::{MemoryMetrics, SwapMetrics};
use async_trait::async_trait;
use sysinfo::System;

/// Memory metrics collector
pub struct MemoryCollector;

impl MemoryCollector {
    /// Create a new memory collector
    pub fn new() -> Self {
        Self
    }
}

impl Default for MemoryCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MetricCollector for MemoryCollector {
    type Output = MemoryMetrics;

    async fn collect(&self) -> DashboardResult<Self::Output> {
        let mut system = System::new_all();
        system.refresh_memory();

        let total_bytes = system.total_memory();
        let available_bytes = system.available_memory();
        let used_bytes = system.used_memory();
        let free_bytes = system.free_memory();

        let usage_percent = if total_bytes > 0 {
            (used_bytes as f32 / total_bytes as f32) * 100.0
        } else {
            0.0
        };

        // Swap information
        let swap_total_bytes = system.total_swap();
        let swap_used_bytes = system.used_swap();
        let swap_free_bytes = system.free_swap();

        let swap_usage_percent = if swap_total_bytes > 0 {
            (swap_used_bytes as f32 / swap_total_bytes as f32) * 100.0
        } else {
            0.0
        };

        let swap = SwapMetrics {
            total_bytes: swap_total_bytes,
            used_bytes: swap_used_bytes,
            free_bytes: swap_free_bytes,
            usage_percent: swap_usage_percent,
        };

        Ok(MemoryMetrics {
            total_bytes,
            available_bytes,
            used_bytes,
            free_bytes,
            usage_percent,
            swap,
        })
    }

    fn name(&self) -> &'static str {
        "memory"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_collector() {
        let collector = MemoryCollector::new();
        let result = collector.collect().await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert!(metrics.total_bytes > 0);
        assert!(metrics.usage_percent >= 0.0);
        assert!(metrics.usage_percent <= 100.0);
    }
}

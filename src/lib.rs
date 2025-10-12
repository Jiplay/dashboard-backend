//! # Dashboard Backend
//!
//! A modular Rust library for collecting system metrics.
//!
//! ## Features
//!
//! - **cpu**: CPU metrics (usage, cores, frequency)
//! - **memory**: Memory and swap metrics
//! - **disk**: Disk usage and filesystem information
//! - **network**: Network interface statistics
//! - **system**: System information (OS, uptime, hostname)
//! - **api-server**: Optional HTTP API server
//!
//! ## Usage
//!
//! ### As a Library
//!
//! ```rust,no_run
//! use dashboard_backend::Dashboard;
//!
//! #[tokio::main]
//! async fn main() {
//!     let dashboard = Dashboard::new();
//!     let metrics = dashboard.collect_all().await.unwrap();
//!     println!("{}", serde_json::to_string_pretty(&metrics).unwrap());
//! }
//! ```
//!
//! ### Individual Collectors
//!
//! ```rust,no_run
//! use dashboard_backend::collectors::{CpuCollector, MetricCollector};
//!
//! #[tokio::main]
//! async fn main() {
//!     let collector = CpuCollector::new();
//!     let cpu_metrics = collector.collect().await.unwrap();
//!     println!("CPU Usage: {}%", cpu_metrics.usage_percent);
//! }
//! ```

pub mod collectors;
pub mod models;

#[cfg(feature = "config-support")]
pub mod config;

#[cfg(feature = "api-server")]
pub mod api;

// Re-export commonly used types
pub use models::{AllMetrics, DashboardError, DashboardResult};

use collectors::MetricCollector;

/// Main dashboard interface for collecting all metrics
pub struct Dashboard {
    #[cfg(feature = "cpu")]
    cpu_collector: collectors::CpuCollector,

    #[cfg(feature = "memory")]
    memory_collector: collectors::MemoryCollector,

    #[cfg(feature = "disk")]
    disk_collector: collectors::DiskCollector,

    #[cfg(feature = "network")]
    network_collector: collectors::NetworkCollector,

    #[cfg(feature = "system")]
    system_collector: collectors::SystemCollector,
}

impl Dashboard {
    /// Create a new Dashboard instance
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "cpu")]
            cpu_collector: collectors::CpuCollector::new(),

            #[cfg(feature = "memory")]
            memory_collector: collectors::MemoryCollector::new(),

            #[cfg(feature = "disk")]
            disk_collector: collectors::DiskCollector::new(),

            #[cfg(feature = "network")]
            network_collector: collectors::NetworkCollector::new(),

            #[cfg(feature = "system")]
            system_collector: collectors::SystemCollector::new(),
        }
    }

    /// Collect all enabled metrics
    pub async fn collect_all(&self) -> DashboardResult<AllMetrics> {
        Ok(AllMetrics {
            #[cfg(feature = "system")]
            system: Some(self.system_collector.collect().await?),

            #[cfg(feature = "cpu")]
            cpu: Some(self.cpu_collector.collect().await?),

            #[cfg(feature = "memory")]
            memory: Some(self.memory_collector.collect().await?),

            #[cfg(feature = "disk")]
            disk: Some(self.disk_collector.collect().await?),

            #[cfg(feature = "network")]
            network: Some(self.network_collector.collect().await?),
        })
    }

    /// Collect CPU metrics
    #[cfg(feature = "cpu")]
    pub async fn collect_cpu(&self) -> DashboardResult<models::CpuMetrics> {
        self.cpu_collector.collect().await
    }

    /// Collect memory metrics
    #[cfg(feature = "memory")]
    pub async fn collect_memory(&self) -> DashboardResult<models::MemoryMetrics> {
        self.memory_collector.collect().await
    }

    /// Collect disk metrics
    #[cfg(feature = "disk")]
    pub async fn collect_disk(&self) -> DashboardResult<models::DiskMetrics> {
        self.disk_collector.collect().await
    }

    /// Collect network metrics
    #[cfg(feature = "network")]
    pub async fn collect_network(&self) -> DashboardResult<models::NetworkMetrics> {
        self.network_collector.collect().await
    }

    /// Collect system metrics
    #[cfg(feature = "system")]
    pub async fn collect_system(&self) -> DashboardResult<models::SystemMetrics> {
        self.system_collector.collect().await
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dashboard_collect_all() {
        let dashboard = Dashboard::new();
        let result = dashboard.collect_all().await;
        assert!(result.is_ok());
    }
}

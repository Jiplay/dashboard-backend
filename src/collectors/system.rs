use crate::collectors::traits::MetricCollector;
use crate::models::error::DashboardResult;
use crate::models::system::SystemMetrics;
use async_trait::async_trait;
use chrono::Utc;
use sysinfo::System;

/// System information collector
pub struct SystemCollector;

impl SystemCollector {
    /// Create a new system collector
    pub fn new() -> Self {
        Self
    }
}

impl Default for SystemCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MetricCollector for SystemCollector {
    type Output = SystemMetrics;

    async fn collect(&self) -> DashboardResult<Self::Output> {
        let mut system = System::new_all();
        system.refresh_all();

        // Get OS information
        let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

        // Get uptime
        let uptime_seconds = System::uptime();

        // Calculate boot time
        let now = Utc::now();
        let boot_time = now - chrono::Duration::seconds(uptime_seconds as i64);

        // Get process count
        let process_count = system.processes().len();

        // Get architecture
        let architecture = System::cpu_arch().unwrap_or_else(|| "Unknown".to_string());

        Ok(SystemMetrics {
            os_name,
            os_version,
            kernel_version,
            hostname,
            uptime_seconds,
            boot_time,
            process_count,
            architecture,
        })
    }

    fn name(&self) -> &'static str {
        "system"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_system_collector() {
        let collector = SystemCollector::new();
        let result = collector.collect().await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert!(!metrics.hostname.is_empty());
        assert!(metrics.uptime_seconds > 0);
    }
}

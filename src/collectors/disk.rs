use crate::collectors::traits::MetricCollector;
use crate::models::disk::{DiskInfo, DiskMetrics};
use crate::models::error::DashboardResult;
use async_trait::async_trait;
use sysinfo::Disks;

/// Disk metrics collector
pub struct DiskCollector;

impl DiskCollector {
    /// Create a new disk collector
    pub fn new() -> Self {
        Self
    }
}

impl Default for DiskCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MetricCollector for DiskCollector {
    type Output = DiskMetrics;

    async fn collect(&self) -> DashboardResult<Self::Output> {
        let disks = Disks::new_with_refreshed_list();

        let mut disk_infos = Vec::new();
        let mut total_bytes = 0u64;
        let mut total_used_bytes = 0u64;

        for disk in &disks {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);

            let usage_percent = if total > 0 {
                (used as f32 / total as f32) * 100.0
            } else {
                0.0
            };

            total_bytes += total;
            total_used_bytes += used;

            disk_infos.push(DiskInfo {
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                device_name: disk.name().to_string_lossy().to_string(),
                file_system: disk.file_system().to_string_lossy().to_string(),
                total_bytes: total,
                available_bytes: available,
                used_bytes: used,
                usage_percent,
                is_removable: disk.is_removable(),
            });
        }

        let overall_usage_percent = if total_bytes > 0 {
            (total_used_bytes as f32 / total_bytes as f32) * 100.0
        } else {
            0.0
        };

        Ok(DiskMetrics {
            disks: disk_infos,
            total_bytes,
            total_used_bytes,
            overall_usage_percent,
        })
    }

    fn name(&self) -> &'static str {
        "disk"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_disk_collector() {
        let collector = DiskCollector::new();
        let result = collector.collect().await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        // At least one disk should be available on any system
        assert!(!metrics.disks.is_empty());
    }
}

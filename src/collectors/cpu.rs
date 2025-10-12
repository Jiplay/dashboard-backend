use crate::collectors::traits::MetricCollector;
use crate::models::cpu::{CoreUsage, CpuMetrics};
use crate::models::error::{DashboardError, DashboardResult};
use async_trait::async_trait;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

/// CPU metrics collector
pub struct CpuCollector;

impl CpuCollector {
    /// Create a new CPU collector
    pub fn new() -> Self {
        Self
    }
}

impl Default for CpuCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MetricCollector for CpuCollector {
    type Output = CpuMetrics;

    async fn collect(&self) -> DashboardResult<Self::Output> {
        let mut system = System::new_with_specifics(
            RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
        );

        // Refresh CPU information
        system.refresh_cpu_all();

        // Need to wait a bit for CPU usage to be accurate
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        system.refresh_cpu_all();

        let cpus = system.cpus();

        if cpus.is_empty() {
            return Err(DashboardError::CollectionError(
                "No CPU information available".to_string(),
            ));
        }

        // Get per-core usage
        let per_core_usage: Vec<CoreUsage> = cpus
            .iter()
            .enumerate()
            .map(|(idx, cpu)| CoreUsage {
                core_id: idx,
                usage_percent: cpu.cpu_usage(),
                frequency_mhz: Some(cpu.frequency()),
            })
            .collect();

        // Calculate overall usage as average of all cores
        let usage_percent = if !per_core_usage.is_empty() {
            per_core_usage.iter().map(|c| c.usage_percent).sum::<f32>() / per_core_usage.len() as f32
        } else {
            0.0
        };

        // Get physical and logical core counts
        let physical_cores = system.physical_core_count().unwrap_or(1);
        let logical_cores = cpus.len();

        // Get CPU brand and vendor
        let brand = cpus
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let vendor_id = cpus
            .first()
            .map(|cpu| cpu.vendor_id().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        // Get frequency (use first CPU's frequency as representative)
        let frequency_mhz = cpus.first().map(|cpu| cpu.frequency());

        Ok(CpuMetrics {
            usage_percent,
            physical_cores,
            logical_cores,
            frequency_mhz,
            per_core_usage,
            brand,
            vendor_id,
        })
    }

    fn name(&self) -> &'static str {
        "cpu"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cpu_collector() {
        let collector = CpuCollector::new();
        let result = collector.collect().await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert!(metrics.logical_cores > 0);
        assert!(metrics.physical_cores > 0);
        assert!(metrics.usage_percent >= 0.0);
        assert!(metrics.usage_percent <= 100.0);
    }
}

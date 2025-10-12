use serde::{Deserialize, Serialize};

/// CPU metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    /// Overall CPU usage percentage (0-100)
    pub usage_percent: f32,

    /// Number of physical cores
    pub physical_cores: usize,

    /// Number of logical cores (including hyperthreading)
    pub logical_cores: usize,

    /// CPU frequency in MHz (if available)
    pub frequency_mhz: Option<u64>,

    /// Per-core usage percentages
    pub per_core_usage: Vec<CoreUsage>,

    /// CPU brand/model name
    pub brand: String,

    /// Vendor ID
    pub vendor_id: String,
}

/// Individual core usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUsage {
    /// Core index
    pub core_id: usize,

    /// Usage percentage (0-100)
    pub usage_percent: f32,

    /// Frequency in MHz (if available)
    pub frequency_mhz: Option<u64>,
}

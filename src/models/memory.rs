use serde::{Deserialize, Serialize};

/// Memory metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// Total RAM in bytes
    pub total_bytes: u64,

    /// Available RAM in bytes
    pub available_bytes: u64,

    /// Used RAM in bytes
    pub used_bytes: u64,

    /// Free RAM in bytes
    pub free_bytes: u64,

    /// Memory usage percentage (0-100)
    pub usage_percent: f32,

    /// Swap memory information
    pub swap: SwapMetrics,
}

/// Swap memory metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapMetrics {
    /// Total swap space in bytes
    pub total_bytes: u64,

    /// Used swap space in bytes
    pub used_bytes: u64,

    /// Free swap space in bytes
    pub free_bytes: u64,

    /// Swap usage percentage (0-100)
    pub usage_percent: f32,
}

impl MemoryMetrics {
    /// Convert bytes to human-readable format
    pub fn total_gb(&self) -> f64 {
        self.total_bytes as f64 / 1_073_741_824.0
    }

    pub fn used_gb(&self) -> f64 {
        self.used_bytes as f64 / 1_073_741_824.0
    }

    pub fn available_gb(&self) -> f64 {
        self.available_bytes as f64 / 1_073_741_824.0
    }
}

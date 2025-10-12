use serde::{Deserialize, Serialize};

/// Disk metrics for all mounted filesystems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    /// List of all disks
    pub disks: Vec<DiskInfo>,

    /// Total storage across all disks in bytes
    pub total_bytes: u64,

    /// Total used storage across all disks in bytes
    pub total_used_bytes: u64,

    /// Overall usage percentage
    pub overall_usage_percent: f32,
}

/// Individual disk information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    /// Mount point
    pub mount_point: String,

    /// Device name
    pub device_name: String,

    /// Filesystem type (e.g., "ext4", "NTFS", "APFS")
    pub file_system: String,

    /// Total space in bytes
    pub total_bytes: u64,

    /// Available space in bytes
    pub available_bytes: u64,

    /// Used space in bytes
    pub used_bytes: u64,

    /// Usage percentage (0-100)
    pub usage_percent: f32,

    /// Whether the disk is removable
    pub is_removable: bool,
}

impl DiskInfo {
    /// Convert bytes to GB
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

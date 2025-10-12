use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// System information metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Operating system name
    pub os_name: String,

    /// OS version
    pub os_version: String,

    /// Kernel version
    pub kernel_version: String,

    /// Hostname
    pub hostname: String,

    /// System uptime in seconds
    pub uptime_seconds: u64,

    /// Boot time
    pub boot_time: DateTime<Utc>,

    /// Number of processes
    pub process_count: usize,

    /// System architecture (e.g., "x86_64", "aarch64")
    pub architecture: String,
}

impl SystemMetrics {
    /// Get uptime as human-readable string
    pub fn uptime_display(&self) -> String {
        let days = self.uptime_seconds / 86400;
        let hours = (self.uptime_seconds % 86400) / 3600;
        let minutes = (self.uptime_seconds % 3600) / 60;

        if days > 0 {
            format!("{}d {}h {}m", days, hours, minutes)
        } else if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }
}

/// Process information (optional feature)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,

    /// Process name
    pub name: String,

    /// CPU usage percentage
    pub cpu_usage: f32,

    /// Memory usage in bytes
    pub memory_bytes: u64,

    /// Parent process ID
    pub parent_pid: Option<u32>,

    /// Process status
    pub status: String,

    /// Command line
    pub cmd: Vec<String>,
}

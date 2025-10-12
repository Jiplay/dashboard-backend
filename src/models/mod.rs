pub mod error;

#[cfg(feature = "cpu")]
pub mod cpu;

#[cfg(feature = "memory")]
pub mod memory;

#[cfg(feature = "disk")]
pub mod disk;

#[cfg(feature = "network")]
pub mod network;

#[cfg(feature = "system")]
pub mod system;

// Re-export commonly used types
pub use error::{DashboardError, DashboardResult};

#[cfg(feature = "cpu")]
pub use cpu::CpuMetrics;

#[cfg(feature = "memory")]
pub use memory::MemoryMetrics;

#[cfg(feature = "disk")]
pub use disk::DiskMetrics;

#[cfg(feature = "network")]
pub use network::NetworkMetrics;

#[cfg(feature = "system")]
pub use system::SystemMetrics;

use serde::{Deserialize, Serialize};

/// Combined metrics from all enabled collectors
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AllMetrics {
    #[cfg(feature = "system")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemMetrics>,

    #[cfg(feature = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<CpuMetrics>,

    #[cfg(feature = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryMetrics>,

    #[cfg(feature = "disk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<DiskMetrics>,

    #[cfg(feature = "network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<NetworkMetrics>,
}

pub mod traits;

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

// Re-export collectors
#[cfg(feature = "cpu")]
pub use cpu::CpuCollector;

#[cfg(feature = "memory")]
pub use memory::MemoryCollector;

#[cfg(feature = "disk")]
pub use disk::DiskCollector;

#[cfg(feature = "network")]
pub use network::NetworkCollector;

#[cfg(feature = "system")]
pub use system::SystemCollector;

pub use traits::MetricCollector;

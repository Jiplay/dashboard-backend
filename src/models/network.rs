use serde::{Deserialize, Serialize};

/// Network metrics for all interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// List of all network interfaces
    pub interfaces: Vec<NetworkInterface>,

    /// Total bytes received across all interfaces
    pub total_bytes_received: u64,

    /// Total bytes transmitted across all interfaces
    pub total_bytes_transmitted: u64,
}

/// Individual network interface information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// Interface name (e.g., "eth0", "wlan0")
    pub name: String,

    /// MAC address
    pub mac_address: Option<String>,

    /// IP addresses assigned to this interface
    pub ip_addresses: Vec<String>,

    /// Bytes received
    pub bytes_received: u64,

    /// Bytes transmitted
    pub bytes_transmitted: u64,

    /// Packets received
    pub packets_received: u64,

    /// Packets transmitted
    pub packets_transmitted: u64,

    /// Errors on receive
    pub errors_received: u64,

    /// Errors on transmit
    pub errors_transmitted: u64,

    /// Whether the interface is up
    pub is_up: bool,

    /// Whether the interface is a loopback interface
    pub is_loopback: bool,
}

impl NetworkInterface {
    /// Convert bytes to MB
    pub fn received_mb(&self) -> f64 {
        self.bytes_received as f64 / 1_048_576.0
    }

    pub fn transmitted_mb(&self) -> f64 {
        self.bytes_transmitted as f64 / 1_048_576.0
    }
}

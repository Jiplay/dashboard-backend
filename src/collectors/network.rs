use crate::collectors::traits::MetricCollector;
use crate::models::error::DashboardResult;
use crate::models::network::{NetworkInterface, NetworkMetrics};
use async_trait::async_trait;
use sysinfo::Networks;

/// Network metrics collector
pub struct NetworkCollector;

impl NetworkCollector {
    /// Create a new network collector
    pub fn new() -> Self {
        Self
    }
}

impl Default for NetworkCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MetricCollector for NetworkCollector {
    type Output = NetworkMetrics;

    async fn collect(&self) -> DashboardResult<Self::Output> {
        let networks = Networks::new_with_refreshed_list();

        let mut interfaces = Vec::new();
        let mut total_bytes_received = 0u64;
        let mut total_bytes_transmitted = 0u64;

        for (interface_name, data) in &networks {
            let bytes_received = data.total_received();
            let bytes_transmitted = data.total_transmitted();
            let packets_received = data.total_packets_received();
            let packets_transmitted = data.total_packets_transmitted();
            let errors_received = data.total_errors_on_received();
            let errors_transmitted = data.total_errors_on_transmitted();

            total_bytes_received += bytes_received;
            total_bytes_transmitted += bytes_transmitted;

            // Get MAC address
            let mac_address = {
                let mac = data.mac_address();
                if mac.is_unspecified() {
                    None
                } else {
                    Some(format!(
                        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                        mac.0[0], mac.0[1], mac.0[2], mac.0[3], mac.0[4], mac.0[5]
                    ))
                }
            };

            // Determine if interface is loopback
            let is_loopback = interface_name.to_lowercase().contains("lo")
                || interface_name.to_lowercase().contains("loopback");

            interfaces.push(NetworkInterface {
                name: interface_name.to_string(),
                mac_address,
                ip_addresses: Vec::new(), // sysinfo doesn't provide IP addresses directly
                bytes_received,
                bytes_transmitted,
                packets_received,
                packets_transmitted,
                errors_received,
                errors_transmitted,
                is_up: bytes_received > 0 || bytes_transmitted > 0, // Heuristic
                is_loopback,
            });
        }

        Ok(NetworkMetrics {
            interfaces,
            total_bytes_received,
            total_bytes_transmitted,
        })
    }

    fn name(&self) -> &'static str {
        "network"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_collector() {
        let collector = NetworkCollector::new();
        let result = collector.collect().await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        // Most systems should have at least a loopback interface
        assert!(!metrics.interfaces.is_empty());
    }
}

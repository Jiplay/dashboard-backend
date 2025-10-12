//! Example: Using individual collectors
//!
//! Run with: cargo run --example custom_collector

use dashboard_backend::collectors::{MetricCollector, CpuCollector, MemoryCollector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Dashboard Backend - Custom Collector Example\n");

    // Create individual collectors
    let cpu_collector = CpuCollector::new();
    let memory_collector = MemoryCollector::new();

    // Collect metrics individually
    println!("Collecting {} metrics...", cpu_collector.name());
    let cpu_metrics = cpu_collector.collect().await?;
    println!("CPU Usage: {:.2}%\n", cpu_metrics.usage_percent);

    println!("Collecting {} metrics...", memory_collector.name());
    let memory_metrics = memory_collector.collect().await?;
    println!("Memory Usage: {:.2}%\n", memory_metrics.usage_percent);

    // You can also implement your own collectors by implementing the MetricCollector trait!
    println!("Collectors can be easily extended by implementing the MetricCollector trait.");

    Ok(())
}

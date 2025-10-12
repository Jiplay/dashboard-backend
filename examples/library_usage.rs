//! Example: Using the dashboard backend as a library
//!
//! Run with: cargo run --example library_usage

use dashboard_backend::Dashboard;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Dashboard Backend - Library Usage Example\n");

    // Create a new dashboard instance
    let dashboard = Dashboard::new();

    // Collect all metrics at once
    println!("=== Collecting All Metrics ===");
    let all_metrics = dashboard.collect_all().await?;
    println!("{}\n", serde_json::to_string_pretty(&all_metrics)?);

    // Or collect individual metrics
    #[cfg(feature = "cpu")]
    {
        println!("=== CPU Metrics ===");
        let cpu_metrics = dashboard.collect_cpu().await?;
        println!("CPU Usage: {:.2}%", cpu_metrics.usage_percent);
        println!("Cores: {} physical, {} logical", cpu_metrics.physical_cores, cpu_metrics.logical_cores);
        println!("Brand: {}\n", cpu_metrics.brand);
    }

    #[cfg(feature = "memory")]
    {
        println!("=== Memory Metrics ===");
        let memory_metrics = dashboard.collect_memory().await?;
        println!("Total: {:.2} GB", memory_metrics.total_gb());
        println!("Used: {:.2} GB ({:.2}%)", memory_metrics.used_gb(), memory_metrics.usage_percent);
        println!("Available: {:.2} GB\n", memory_metrics.available_gb());
    }

    #[cfg(feature = "disk")]
    {
        println!("=== Disk Metrics ===");
        let disk_metrics = dashboard.collect_disk().await?;
        println!("Total disks: {}", disk_metrics.disks.len());
        for disk in &disk_metrics.disks {
            println!("  {} ({}):", disk.mount_point, disk.file_system);
            println!("    Total: {:.2} GB", disk.total_gb());
            println!("    Used: {:.2} GB ({:.2}%)", disk.used_gb(), disk.usage_percent);
        }
        println!();
    }

    #[cfg(feature = "system")]
    {
        println!("=== System Information ===");
        let system_metrics = dashboard.collect_system().await?;
        println!("Hostname: {}", system_metrics.hostname);
        println!("OS: {} {}", system_metrics.os_name, system_metrics.os_version);
        println!("Kernel: {}", system_metrics.kernel_version);
        println!("Architecture: {}", system_metrics.architecture);
        println!("Uptime: {}", system_metrics.uptime_display());
        println!("Processes: {}\n", system_metrics.process_count);
    }

    Ok(())
}

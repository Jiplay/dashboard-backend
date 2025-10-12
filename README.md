# Dashboard Backend

A modular, production-ready Rust library for collecting system metrics. Designed for easy integration into existing APIs or as a standalone service.

## Features

- **Modular Design**: Enable only the metrics you need via feature flags
- **Dual-Mode Support**: Use as a library or standalone HTTP server
- **Cross-Platform**: Works on Linux, macOS, and Windows
- **Async/Await**: Built with Tokio for high performance
- **Type-Safe**: Strongly typed metrics with comprehensive error handling
- **Zero Configuration**: Works out of the box with sensible defaults

### Available Metrics

- **CPU**: Usage percentage, core count, frequency, per-core statistics
- **Memory**: RAM and swap usage, available/used bytes
- **Disk**: Filesystem usage, mount points, disk information
- **Network**: Interface statistics, traffic counters
- **System**: OS info, uptime, hostname, process count

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dashboard-backend = { path = ".", features = ["full"] }
```

### Feature Flags

Enable only what you need:

```toml
[dependencies]
dashboard-backend = { path = ".", features = ["cpu", "memory", "disk"] }
```

Available features:
- `cpu` - CPU metrics
- `memory` - Memory metrics
- `disk` - Disk metrics
- `network` - Network metrics
- `system` - System information
- `config-support` - Configuration file support
- `api-server` - HTTP API server
- `full` - All features enabled
- `default` - `cpu`, `memory`, `disk`, `system`

## Usage

### As a Library

The simplest way to use the library:

```rust
use dashboard_backend::Dashboard;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dashboard = Dashboard::new();

    // Collect all metrics
    let metrics = dashboard.collect_all().await?;
    println!("{}", serde_json::to_string_pretty(&metrics)?);

    Ok(())
}
```

### Individual Collectors

For more control, use individual collectors:

```rust
use dashboard_backend::collectors::{CpuCollector, MemoryCollector, MetricCollector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cpu = CpuCollector::new();
    let memory = MemoryCollector::new();

    let cpu_metrics = cpu.collect().await?;
    let memory_metrics = memory.collect().await?;

    println!("CPU Usage: {:.2}%", cpu_metrics.usage_percent);
    println!("Memory Usage: {:.2}%", memory_metrics.usage_percent);

    Ok(())
}
```

### As a Standalone HTTP Server

Run with the `api-server` feature:

```bash
cargo run --features full --bin dashboard-server
```

The server will start on `http://127.0.0.1:3000` by default.

#### API Endpoints

- `GET /api/v1/health` - Health check
- `GET /api/v1/all` - All metrics
- `GET /api/v1/cpu` - CPU metrics
- `GET /api/v1/memory` - Memory metrics
- `GET /api/v1/disk` - Disk metrics
- `GET /api/v1/network` - Network metrics
- `GET /api/v1/system` - System information

Example:

```bash
curl http://localhost:3000/api/v1/cpu | jq
```

### Integration into Your Existing API

#### Example with Axum

```rust
use axum::{routing::get, Router, Json};
use dashboard_backend::Dashboard;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let dashboard = Arc::new(Dashboard::new());

    let app = Router::new()
        .route("/metrics", get({
            let dashboard = dashboard.clone();
            move || async move {
                let metrics = dashboard.collect_all().await.unwrap();
                Json(metrics)
            }
        }));

    // Add your existing routes here
    // app = app.merge(your_existing_routes());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

#### Example with Actix-Web

```rust
use actix_web::{web, App, HttpServer, Result};
use dashboard_backend::Dashboard;

async fn get_metrics(dashboard: web::Data<Dashboard>) -> Result<impl actix_web::Responder> {
    let metrics = dashboard.collect_all().await.unwrap();
    Ok(web::Json(metrics))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let dashboard = web::Data::new(Dashboard::new());

    HttpServer::new(move || {
        App::new()
            .app_data(dashboard.clone())
            .route("/metrics", web::get().to(get_metrics))
            // Add your existing routes here
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
```

## Configuration

### Configuration File

Create a `config/default.toml`:

```toml
[collectors]
collection_interval_secs = 5
enable_cpu = true
enable_memory = true
enable_disk = true
enable_network = true
enable_system = true

[server]
host = "127.0.0.1"
port = 3000
enable_cors = true
cors_origins = ["*"]
```

### Loading Configuration

```rust
use dashboard_backend::config::DashboardConfig;

let config = DashboardConfig::from_file("config/default.toml")?;
```

## Examples

Run the examples to see different usage patterns:

```bash
# Library usage example
cargo run --example library_usage

# Custom collector example
cargo run --example custom_collector

# Standalone server
cargo run --features full --bin dashboard-server
```

## Extending with Custom Collectors

Implement the `MetricCollector` trait to create custom collectors:

```rust
use dashboard_backend::collectors::MetricCollector;
use dashboard_backend::models::error::DashboardResult;
use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize)]
pub struct CustomMetrics {
    pub value: f64,
}

pub struct CustomCollector;

#[async_trait]
impl MetricCollector for CustomCollector {
    type Output = CustomMetrics;

    async fn collect(&self) -> DashboardResult<Self::Output> {
        Ok(CustomMetrics { value: 42.0 })
    }

    fn name(&self) -> &'static str {
        "custom"
    }
}
```

## Architecture

```
dashboard-backend/
├── src/
│   ├── lib.rs                  # Main library entry point
│   ├── main.rs                 # Standalone server binary
│   ├── collectors/             # Metric collection modules
│   │   ├── cpu.rs
│   │   ├── memory.rs
│   │   ├── disk.rs
│   │   ├── network.rs
│   │   ├── system.rs
│   │   └── traits.rs           # Collector trait definitions
│   ├── models/                 # Data models
│   │   ├── cpu.rs
│   │   ├── memory.rs
│   │   ├── disk.rs
│   │   ├── network.rs
│   │   ├── system.rs
│   │   └── error.rs
│   ├── api/                    # HTTP API (optional)
│   │   ├── handlers.rs
│   │   └── routes.rs
│   └── config/                 # Configuration
│       └── mod.rs
└── examples/                   # Usage examples
```

## Performance

- **Lightweight**: Minimal overhead on system resources
- **Async**: Non-blocking metric collection
- **Efficient**: Uses `sysinfo` crate for optimal system access
- **Caching**: Built-in support for metric caching (configurable)

## Dependencies

Core dependencies:
- `sysinfo` - Cross-platform system information
- `tokio` - Async runtime
- `serde` - Serialization
- `async-trait` - Async trait support

Optional dependencies:
- `axum` - HTTP framework (api-server feature)
- `config` - Configuration management (config-support feature)

## Platform Support

- Linux (all distributions)
- macOS
- Windows

## License

MIT

## Contributing

Contributions are welcome! The modular design makes it easy to add new collectors or improve existing ones.

### Adding a New Collector

1. Create a new file in `src/collectors/`
2. Implement the `MetricCollector` trait
3. Add corresponding model in `src/models/`
4. Add feature flag in `Cargo.toml`
5. Export in `src/collectors/mod.rs`

## Roadmap

- [ ] WebSocket support for real-time metrics
- [ ] Prometheus exporter
- [ ] Metrics aggregation and history
- [ ] Docker container metrics
- [ ] GPU metrics
- [ ] Custom alerting system

## Support

For issues and questions, please open an issue on GitHub.

# Quick Start Guide

## Prerequisites

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Building the Project

### Build the library (default features)

```bash
cargo build
```

### Build with all features

```bash
cargo build --all-features
```

### Build in release mode (optimized)

```bash
cargo build --release --all-features
```

## Running Examples

### Library usage example

```bash
cargo run --example library_usage
```

### Custom collector example

```bash
cargo run --example custom_collector
```

## Running the Standalone Server

```bash
cargo run --features full --bin dashboard-server
```

Then test the API:

```bash
# Health check
curl http://localhost:3000/api/v1/health

# Get all metrics
curl http://localhost:3000/api/v1/all | jq

# Get CPU metrics
curl http://localhost:3000/api/v1/cpu | jq

# Get memory metrics
curl http://localhost:3000/api/v1/memory | jq

# Get disk metrics
curl http://localhost:3000/api/v1/disk | jq

# Get system info
curl http://localhost:3000/api/v1/system | jq
```

## Running Tests

```bash
cargo test --all-features
```

## Integration into Your API

### Step 1: Add as dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
dashboard-backend = { path = "../path/to/dashboard-backend" }
```

### Step 2: Use in your code

```rust
use dashboard_backend::Dashboard;
use std::sync::Arc;

// In your main function or service initialization
let dashboard = Arc::new(Dashboard::new());

// In your API handler
let metrics = dashboard.collect_all().await?;
```

### Step 3: Expose via your API framework

See the README.md for examples with Axum, Actix-Web, and other frameworks.

## Feature Flags

Choose only the features you need:

```toml
# Minimal build (CPU and memory only)
dashboard-backend = { path = ".", features = ["cpu", "memory"] }

# Full build (all features)
dashboard-backend = { path = ".", features = ["full"] }

# Library only (no HTTP server)
dashboard-backend = { path = ".", default-features = false, features = ["cpu", "memory", "disk", "system"] }
```

## Troubleshooting

### Build errors

Make sure you have the latest Rust toolchain:

```bash
rustup update
```

### Permission errors on Linux

Some metrics may require elevated permissions. Run with `sudo` if needed:

```bash
sudo cargo run --features full --bin dashboard-server
```

### Port already in use

Change the port in `config/default.toml`:

```toml
[server]
port = 8080
```

## Next Steps

- Read the full [README.md](README.md) for detailed documentation
- Check the [examples/](examples/) directory for more usage patterns
- Implement custom collectors by extending the `MetricCollector` trait
- Configure the system using `config/default.toml`

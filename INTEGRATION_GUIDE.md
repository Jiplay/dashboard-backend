# Integration Guide

This guide shows how to integrate the dashboard-backend library into your existing API.

## Integration Patterns

### Pattern 1: Direct Library Integration (Recommended)

Best for: When you want full control and minimal overhead.

**Advantages:**
- No network calls (direct function calls)
- Lower latency
- Shared memory space
- Type safety

**Example with your existing Axum API:**

```rust
// In your main.rs or lib.rs
use axum::{extract::State, routing::get, Router, Json};
use dashboard_backend::Dashboard;
use std::sync::Arc;

// Your app state
#[derive(Clone)]
struct AppState {
    // Your existing state
    db: Arc<Database>,
    // Add the dashboard
    dashboard: Arc<Dashboard>,
}

async fn metrics_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let metrics = state.dashboard.collect_all().await.unwrap();
    Json(serde_json::to_value(metrics).unwrap())
}

async fn cpu_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let cpu = state.dashboard.collect_cpu().await.unwrap();
    Json(serde_json::to_value(cpu).unwrap())
}

#[tokio::main]
async fn main() {
    let state = AppState {
        db: Arc::new(Database::new()),
        dashboard: Arc::new(Dashboard::new()),
    };

    let app = Router::new()
        // Your existing routes
        .route("/api/users", get(your_users_handler))
        .route("/api/posts", get(your_posts_handler))
        // Add system metrics routes
        .route("/api/metrics/all", get(metrics_handler))
        .route("/api/metrics/cpu", get(cpu_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Pattern 2: Standalone Microservice

Best for: When you want isolation or when your API is in a different language.

**Advantages:**
- Language-agnostic
- Independent deployment
- Easy to scale separately
- Can be containerized

**Setup:**

1. Run the dashboard server:
```bash
cargo run --features full --bin dashboard-server
```

2. Call from your API:
```javascript
// In your Node.js/Express API
app.get('/api/metrics', async (req, res) => {
    const response = await fetch('http://localhost:3000/api/v1/all');
    const metrics = await response.json();
    res.json(metrics);
});
```

```python
# In your Python/Flask API
@app.route('/api/metrics')
def metrics():
    response = requests.get('http://localhost:3000/api/v1/all')
    return jsonify(response.json())
```

### Pattern 3: Embedded in Background Task

Best for: Periodic metric collection and storage.

```rust
use dashboard_backend::Dashboard;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    let dashboard = Dashboard::new();

    // Collect metrics every 5 seconds
    let mut interval = interval(Duration::from_secs(5));

    tokio::spawn(async move {
        loop {
            interval.tick().await;

            match dashboard.collect_all().await {
                Ok(metrics) => {
                    // Store in database, send to monitoring service, etc.
                    println!("Collected metrics: {:?}", metrics);
                    // database.store(metrics).await;
                }
                Err(e) => eprintln!("Error collecting metrics: {}", e),
            }
        }
    });

    // Your main application continues...
}
```

## Real-World Integration Examples

### Example 1: Adding to an Existing Axum API

```rust
// src/main.rs
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dashboard_backend::Dashboard;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Your existing models
#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

// Combined app state
#[derive(Clone)]
struct AppState {
    dashboard: Arc<Dashboard>,
    // Your existing state here
}

// Your existing handlers
async fn list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    // Your existing logic
    Json(vec![])
}

// New system metrics handler
async fn get_metrics(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    state
        .dashboard
        .collect_all()
        .await
        .map(|m| Json(serde_json::to_value(m).unwrap()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[tokio::main]
async fn main() {
    let state = AppState {
        dashboard: Arc::new(Dashboard::new()),
    };

    let app = Router::new()
        // Your existing API routes
        .route("/api/users", get(list_users))
        // New metrics routes
        .route("/api/system/metrics", get(get_metrics))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Example 2: Adding to Actix-Web

```rust
use actix_web::{web, App, HttpServer, Result};
use dashboard_backend::Dashboard;
use std::sync::Arc;

async fn health_check() -> Result<String> {
    Ok("OK".to_string())
}

async fn get_metrics(dashboard: web::Data<Dashboard>) -> Result<impl actix_web::Responder> {
    let metrics = dashboard.collect_all().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(e)
    })?;
    Ok(web::Json(metrics))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let dashboard = web::Data::new(Dashboard::new());

    HttpServer::new(move || {
        App::new()
            .app_data(dashboard.clone())
            // Your existing routes
            .route("/health", web::get().to(health_check))
            // Metrics routes
            .route("/api/metrics", web::get().to(get_metrics))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Example 3: Adding to Rocket

```rust
use rocket::{State, serde::json::Json};
use dashboard_backend::Dashboard;
use std::sync::Arc;

#[get("/metrics")]
async fn get_metrics(dashboard: &State<Arc<Dashboard>>) -> Json<dashboard_backend::AllMetrics> {
    let metrics = dashboard.collect_all().await.unwrap();
    Json(metrics)
}

#[launch]
fn rocket() -> _ {
    let dashboard = Arc::new(Dashboard::new());

    rocket::build()
        .manage(dashboard)
        .mount("/api", routes![get_metrics])
}
```

### Example 4: Metrics Caching

For high-traffic APIs, implement caching:

```rust
use dashboard_backend::{Dashboard, AllMetrics};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

struct CachedDashboard {
    dashboard: Dashboard,
    cached_metrics: Arc<RwLock<Option<AllMetrics>>>,
}

impl CachedDashboard {
    fn new() -> Self {
        let dashboard = Dashboard::new();
        let cached_metrics = Arc::new(RwLock::new(None));

        // Background task to refresh cache
        let cache_clone = cached_metrics.clone();
        let dashboard_clone = Dashboard::new();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                if let Ok(metrics) = dashboard_clone.collect_all().await {
                    *cache_clone.write().await = Some(metrics);
                }
            }
        });

        Self {
            dashboard,
            cached_metrics,
        }
    }

    async fn get_metrics(&self) -> Option<AllMetrics> {
        self.cached_metrics.read().await.clone()
    }
}

// Use in your API
async fn metrics_handler(State(cache): State<Arc<CachedDashboard>>) -> Json<AllMetrics> {
    let metrics = cache.get_metrics().await.unwrap_or_default();
    Json(metrics)
}
```

## Configuration in Your API

### Using Environment Variables

```rust
use dashboard_backend::config::DashboardConfig;
use std::env;

fn load_config() -> DashboardConfig {
    let config_path = env::var("DASHBOARD_CONFIG")
        .unwrap_or_else(|_| "config/default.toml".to_string());

    DashboardConfig::from_file(&config_path)
        .unwrap_or_default()
}
```

### Custom Configuration

```rust
use dashboard_backend::config::{DashboardConfig, CollectorConfig};

let config = DashboardConfig {
    collectors: CollectorConfig {
        enable_cpu: true,
        enable_memory: true,
        enable_disk: false,  // Disable disk metrics
        enable_network: false,  // Disable network metrics
        enable_system: true,
        collection_interval_secs: 10,
    },
    ..Default::default()
};
```

## Docker Integration

### Dockerfile

```dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release --features full

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/dashboard-server /usr/local/bin/
COPY config/default.toml /etc/dashboard/config.toml

EXPOSE 3000

CMD ["dashboard-server"]
```

### Docker Compose

```yaml
version: '3.8'

services:
  api:
    build: .
    ports:
      - "3000:3000"
    environment:
      - RUST_LOG=info
    volumes:
      - ./config:/etc/dashboard
```

## Best Practices

1. **Use caching** for high-traffic APIs
2. **Error handling**: Always handle potential errors from metric collection
3. **Rate limiting**: Consider rate limiting metrics endpoints
4. **Security**: Add authentication for metrics endpoints in production
5. **Monitoring**: Monitor the metrics collector itself for performance
6. **Feature flags**: Only enable the collectors you need

## Performance Considerations

- Metric collection is async and non-blocking
- CPU collector includes a 200ms sleep for accurate measurements
- Consider caching results for high-frequency requests
- Network metrics may require elevated permissions on some systems

## Troubleshooting

### High Memory Usage

Disable unused collectors:

```toml
[dependencies]
dashboard-backend = { path = ".", default-features = false, features = ["cpu", "memory"] }
```

### Slow Response Times

Implement caching (see Example 4 above)

### Permission Errors

Some metrics require elevated privileges. Either:
- Run with appropriate permissions
- Disable collectors that require elevated access

## Next Steps

- See [README.md](README.md) for detailed API documentation
- Check [examples/](examples/) for more usage patterns
- Review [QUICKSTART.md](QUICKSTART.md) for building and running

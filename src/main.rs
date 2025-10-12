use dashboard_backend::config::DashboardConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from file or use default
    let config = match DashboardConfig::from_file("config/default.toml") {
        Ok(config) => {
            println!("Loaded configuration from config/default.toml");
            config
        }
        Err(_) => {
            println!("Using default configuration");
            DashboardConfig::default()
        }
    };

    // Start the API server
    dashboard_backend::api::start_server(config).await?;

    Ok(())
}

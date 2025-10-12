use serde::{Deserialize, Serialize};

/// Configuration for the dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Server configuration (if API server is enabled)
    #[cfg(feature = "api-server")]
    pub server: ServerConfig,

    /// Collector configuration
    pub collectors: CollectorConfig,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            #[cfg(feature = "api-server")]
            server: ServerConfig::default(),
            collectors: CollectorConfig::default(),
        }
    }
}

/// Server configuration
#[cfg(feature = "api-server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,

    /// Port to listen on
    pub port: u16,

    /// Enable CORS
    pub enable_cors: bool,

    /// CORS allowed origins
    pub cors_origins: Vec<String>,
}

#[cfg(feature = "api-server")]
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            enable_cors: true,
            cors_origins: vec!["*".to_string()],
        }
    }
}

/// Collector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectorConfig {
    /// Enable CPU collector
    #[cfg(feature = "cpu")]
    pub enable_cpu: bool,

    /// Enable memory collector
    #[cfg(feature = "memory")]
    pub enable_memory: bool,

    /// Enable disk collector
    #[cfg(feature = "disk")]
    pub enable_disk: bool,

    /// Enable network collector
    #[cfg(feature = "network")]
    pub enable_network: bool,

    /// Enable system collector
    #[cfg(feature = "system")]
    pub enable_system: bool,

    /// Collection interval in seconds (for periodic collection)
    pub collection_interval_secs: u64,
}

impl Default for CollectorConfig {
    fn default() -> Self {
        Self {
            #[cfg(feature = "cpu")]
            enable_cpu: true,

            #[cfg(feature = "memory")]
            enable_memory: true,

            #[cfg(feature = "disk")]
            enable_disk: true,

            #[cfg(feature = "network")]
            enable_network: true,

            #[cfg(feature = "system")]
            enable_system: true,

            collection_interval_secs: 5,
        }
    }
}

impl DashboardConfig {
    /// Load configuration from a TOML file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config: DashboardConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}

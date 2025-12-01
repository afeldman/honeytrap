use honeytrap_core::{Config, HoneyTrap};
use honeytrap_policy::PolicyEngine;
use std::path::Path;
use tracing_subscriber::EnvFilter;

/// Load configuration from a TOML file or use defaults
fn load_config(path: &Path) -> anyhow::Result<Config> {
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        match toml::from_str(&content) {
            Ok(config) => {
                tracing::info!("Configuration loaded from: {}", path.display());
                Ok(config)
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to parse config file {}: {}, using defaults",
                    path.display(),
                    e
                );
                Ok(Config::default())
            }
        }
    } else {
        tracing::warn!(
            "Config file not found at {}, using defaults",
            path.display()
        );
        Ok(Config::default())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();

    tracing::info!("Starting HoneyTrap server...");

    // Load config from config.toml
    let config_path = Path::new("config.toml");
    let config = load_config(config_path)?;

    // Initialize PolicyEngine
    // Note: The policy engine is initialized but not yet integrated into the HoneyTrap router.
    // Future work: Connect PolicyEngine to Router for policy-based traffic decisions.
    let _policy_engine = PolicyEngine::new(honeytrap_policy::model::ActionType::Deception);
    // TODO: Load policies from config.policies.files and integrate with Router
    tracing::info!("Policy engine initialized");

    // Initialize HoneyTrap system (includes AI-Client initialization)
    let honeytrap = HoneyTrap::new(config)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to initialize HoneyTrap: {}", e))?;
    tracing::info!("HoneyTrap system initialized");

    // Start listener
    tracing::info!("🚀 Listener starting...");

    // Handle graceful shutdown
    tokio::select! {
        result = honeytrap.run() => {
            if let Err(e) = result {
                tracing::error!("HoneyTrap run error: {}", e);
                return Err(anyhow::anyhow!("HoneyTrap run error: {}", e));
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Received shutdown signal, stopping...");
        }
    }

    tracing::info!("HoneyTrap server stopped");
    Ok(())
}

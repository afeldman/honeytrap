//! HoneyTrap Server Main Entry Point
//!
//! This module contains the main entry point for the HoneyTrap server application.
//! It handles configuration loading, system initialization, and the main event loop.
//!
//! # Overview
//!
//! The server performs the following steps on startup:
//! 1. Initializes structured JSON logging
//! 2. Loads configuration from `config.toml` (or uses defaults)
//! 3. Initializes the PolicyEngine for traffic decision-making
//! 4. Initializes the HoneyTrap system (AI engine, honeypots, QUIC transport)
//! 5. Starts the listener and processes incoming connections
//! 6. Handles graceful shutdown on SIGINT (Ctrl+C)
//!
//! # Examples
//!
//! Run the server with default configuration:
//! ```bash
//! cargo run --bin honeytrap-server
//! ```
//!
//! Run with custom log level:
//! ```bash
//! RUST_LOG=debug cargo run --bin honeytrap-server
//! ```

use honeytrap_core::{Config, HoneyTrap};
use honeytrap_policy::PolicyEngine;
use std::path::Path;
use tracing_subscriber::EnvFilter;

/// Loads the HoneyTrap configuration from a TOML file.
///
/// This function attempts to read and parse a configuration file. If the file
/// doesn't exist or cannot be parsed, it falls back to using default configuration
/// values and logs a warning.
///
/// # Arguments
///
/// * `path` - Path to the configuration file (typically `config.toml`)
///
/// # Returns
///
/// * `Ok(Config)` - Successfully loaded or default configuration
/// * `Err(anyhow::Error)` - Fatal I/O error reading the file
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// # async fn example() -> anyhow::Result<()> {
/// let config = load_config(Path::new("config.toml"))?;
/// println!("Loaded config for bind address: {}", config.network.bind_addr);
/// # Ok(())
/// # }
/// ```
///
/// # Behavior
///
/// - If the file exists and is valid TOML matching the Config schema: loads it
/// - If the file exists but has parsing errors: logs warning, uses defaults
/// - If the file doesn't exist: logs warning, uses defaults
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

/// HoneyTrap server main entry point.
///
/// Initializes and runs the HoneyTrap server with the following steps:
///
/// 1. **Logging Setup**: Configures JSON-formatted structured logging with
///    environment-based filtering (controlled via `RUST_LOG` env var)
///
/// 2. **Configuration Loading**: Reads `config.toml` or falls back to defaults
///
/// 3. **Policy Engine Init**: Creates a PolicyEngine for traffic routing decisions
///    (currently initialized but not yet integrated into the router)
///
/// 4. **HoneyTrap System Init**: Initializes the complete system including:
///    - AI anomaly detection engine
///    - Honeypot deployments (SSH, HTTP, MySQL, etc.)
///    - QUIC transport layer for secure connections
///    - Traffic router
///
/// 5. **Listener Start**: Begins accepting incoming connections and processing
///    them through the AI engine and policy rules
///
/// 6. **Graceful Shutdown**: Waits for either a connection error or SIGINT signal
///    (Ctrl+C) before shutting down cleanly
///
/// # Returns
///
/// * `Ok(())` - Server shut down gracefully
/// * `Err(anyhow::Error)` - Fatal error during initialization or runtime
///
/// # Examples
///
/// Run the server (from project root):
/// ```bash
/// cargo run --bin honeytrap-server
/// ```
///
/// Run with debug logging:
/// ```bash
/// RUST_LOG=debug cargo run --bin honeytrap-server
/// ```
///
/// Run with trace logging for specific modules:
/// ```bash
/// RUST_LOG=honeytrap_core=trace,honeytrap_policy=debug cargo run --bin honeytrap-server
/// ```
///
/// # Environment Variables
///
/// - `RUST_LOG`: Controls logging level (default: "info")
///   - Levels: trace, debug, info, warn, error
///   - Can be module-specific: `RUST_LOG=honeytrap_core=debug`
///
/// # Errors
///
/// Returns errors in the following cases:
/// - Failed to read configuration file (I/O error)
/// - Failed to initialize HoneyTrap system (network binding, crypto setup, etc.)
/// - Runtime error in the main event loop
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

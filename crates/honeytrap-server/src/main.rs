use anyhow::{Context, Result};
use honeytrap_core::{Config, HoneyTrap};
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::path::PathBuf;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

/// Server-Konfiguration
#[derive(Debug)]
struct ServerConfig {
    config_path: PathBuf,
    enable_json_logs: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            config_path: PathBuf::from("honeytrap.toml"),
            enable_json_logs: false,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse Umgebungsvariablen
    let server_config = parse_env();

    // Logging initialisieren
    init_logging(&server_config);

    info!("🍯 Starting HoneyTrap Server v{}", env!("CARGO_PKG_VERSION"));

    // Config laden
    let config = load_config(&server_config.config_path).await?;
    info!("✅ Configuration loaded from {:?}", server_config.config_path);

    // HoneyTrap System initialisieren
    let honeytrap = HoneyTrap::new(config).await.map_err(|e| {
        anyhow::anyhow!("Failed to initialize HoneyTrap: {}", e)
    })?;

    info!("✅ HoneyTrap system initialized");

    // Signal Handler Setup
    let signals = Signals::new([SIGTERM, SIGINT, SIGQUIT])
        .context("Failed to register signal handlers")?;
    let handle = signals.handle();

    // Server starten
    let server_handle = tokio::spawn(async move {
        if let Err(e) = honeytrap.run().await {
            error!("Server error: {:#}", e);
            std::process::exit(1);
        }
    });

    // Auf Shutdown-Signal warten
    tokio::select! {
        _ = wait_for_shutdown_signal(signals) => {
            info!("🛑 Shutdown signal received, stopping server...");
        }
        _ = server_handle => {
            warn!("Server task completed unexpectedly");
        }
    }

    // Cleanup
    handle.close();
    info!("👋 HoneyTrap Server stopped");

    Ok(())
}

/// Umgebungsvariablen parsen
fn parse_env() -> ServerConfig {
    ServerConfig {
        config_path: std::env::var("HONEYTRAP_CONFIG")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("honeytrap.toml")),
        enable_json_logs: std::env::var("HONEYTRAP_JSON_LOGS")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false),
    }
}

/// Logging initialisieren
fn init_logging(config: &ServerConfig) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,honeytrap=debug"));

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    if config.enable_json_logs {
        // JSON logging mit manueller Initialisierung
        tracing::subscriber::set_global_default(subscriber.finish())
            .expect("Failed to set tracing subscriber");
    } else {
        subscriber.init();
    }
}

/// Config-Datei laden
async fn load_config(path: &PathBuf) -> Result<Config> {
    let content = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Failed to read config file: {:?}", path))?;

    let config: Config = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {:?}", path))?;

    Ok(config)
}

/// Auf Shutdown-Signal warten
async fn wait_for_shutdown_signal(mut signals: Signals) {
    use futures::StreamExt;

    while let Some(signal) = signals.next().await {
        match signal {
            SIGTERM | SIGINT | SIGQUIT => {
                info!("Received signal: {}", signal);
                break;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_server_config() {
        let config = ServerConfig::default();
        assert_eq!(config.config_path, PathBuf::from("honeytrap.toml"));
        assert!(!config.enable_json_logs);
    }

    #[test]
    fn test_parse_env() {
        std::env::set_var("HONEYTRAP_CONFIG", "/etc/honeytrap/config.toml");
        std::env::set_var("HONEYTRAP_JSON_LOGS", "true");

        let config = parse_env();
        assert_eq!(
            config.config_path,
            PathBuf::from("/etc/honeytrap/config.toml")
        );
        assert!(config.enable_json_logs);

        // Cleanup
        std::env::remove_var("HONEYTRAP_CONFIG");
        std::env::remove_var("HONEYTRAP_JSON_LOGS");
    }
}

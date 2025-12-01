use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();

    // TODO: Config einlesen (config.toml)
    // TODO: PolicyEngine initialisieren
    // TODO: AI-Client initialisieren
    // TODO: Listener starten

    tracing::info!("Starting HoneyTrap server...");
    Ok(())
}

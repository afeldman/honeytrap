use super::{Connection, Honeypot, HoneypotType, Session};
use async_trait::async_trait;

/// MySQL Honeypot (Low Interaction)
pub struct MysqlHoneypot {
    port: u16,
}

impl MysqlHoneypot {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}

#[async_trait]
impl Honeypot for MysqlHoneypot {
    async fn handle(
        &self,
        _connection: Connection,
        session: Session,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("🗄️ MySQL Honeypot: Handling connection {}", session.id);

        // MySQL Greeting senden
        // TODO: Echte MySQL-Protokoll-Implementierung
        tracing::debug!("📤 Sending MySQL greeting: 5.7.38-0ubuntu0.18.04.1");

        // Login-Attempt loggen
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        tracing::debug!("🔐 MySQL login attempt logged");

        tracing::info!("✅ MySQL Honeypot: Session {} completed", session.id);

        Ok(())
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn service_type(&self) -> HoneypotType {
        HoneypotType::Mysql
    }
}

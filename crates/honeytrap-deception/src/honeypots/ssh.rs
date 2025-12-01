use super::{Connection, Honeypot, HoneypotType, Session};
use async_trait::async_trait;

/// SSH Honeypot (Medium Interaction)
pub struct SshHoneypot {
    port: u16,
}

impl SshHoneypot {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}

#[async_trait]
impl Honeypot for SshHoneypot {
    async fn handle(
        &self,
        _connection: Connection,
        session: Session,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("🔐 SSH Honeypot: Handling connection {}", session.id);

        // SSH Banner senden
        // TODO: Echte SSH-Implementierung
        tracing::debug!("📤 Sending SSH banner: SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5");

        // Authentifizierung emulieren
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        tracing::debug!("🔑 Authentication attempt logged");

        // Fake Shell
        tracing::debug!("💻 Starting fake shell session");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        tracing::info!("✅ SSH Honeypot: Session {} completed", session.id);

        Ok(())
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn service_type(&self) -> HoneypotType {
        HoneypotType::Ssh
    }
}

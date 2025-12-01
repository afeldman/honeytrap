use super::{Honeypot, HoneypotType, Connection, Session};
use async_trait::async_trait;

/// HTTP Honeypot (High Interaction)
pub struct HttpHoneypot {
    port: u16,
}

impl HttpHoneypot {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}

#[async_trait]
impl Honeypot for HttpHoneypot {
    async fn handle(
        &self,
        _connection: Connection,
        session: Session,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("🌐 HTTP Honeypot: Handling connection {}", session.id);
        
        // HTTP Response senden
        // TODO: Echte HTTP-Implementierung
        tracing::debug!("📤 Sending HTTP 200 OK");
        
        // Fake Web Application
        tracing::debug!("🖥️ Serving fake login page");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        tracing::info!("✅ HTTP Honeypot: Session {} completed", session.id);
        
        Ok(())
    }
    
    fn port(&self) -> u16 {
        self.port
    }
    
    fn service_type(&self) -> HoneypotType {
        HoneypotType::Http
    }
}

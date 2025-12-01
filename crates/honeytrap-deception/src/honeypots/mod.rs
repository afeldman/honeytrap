pub mod http;
pub mod mysql;
pub mod ssh;

pub use http::HttpHoneypot;
pub use mysql::MysqlHoneypot;
pub use ssh::SshHoneypot;

use async_trait::async_trait;
use std::fmt;

/// Honeypot-Trait
#[async_trait]
pub trait Honeypot: Send + Sync {
    /// Connection verarbeiten
    async fn handle(
        &self,
        connection: Connection,
        session: Session,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Port
    fn port(&self) -> u16;

    /// Service-Type
    fn service_type(&self) -> HoneypotType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoneypotType {
    Ssh,
    Http,
    Mysql,
}

impl fmt::Display for HoneypotType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HoneypotType::Ssh => write!(f, "SSH"),
            HoneypotType::Http => write!(f, "HTTP"),
            HoneypotType::Mysql => write!(f, "MySQL"),
        }
    }
}

// Placeholder Connection Type
#[derive(Debug)]
pub struct Connection {
    pub peer_addr: std::net::SocketAddr,
}

// Session structure matching honeytrap-core
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub peer_addr: std::net::SocketAddr,
    pub started_at: std::time::Instant,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub is_suspicious: bool,
    pub anomaly_score: f64,
}

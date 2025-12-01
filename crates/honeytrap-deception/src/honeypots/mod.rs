pub mod ssh;
pub mod http;
pub mod mysql;

pub use ssh::SshHoneypot;
pub use http::HttpHoneypot;
pub use mysql::MysqlHoneypot;

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

// Placeholder Types
#[derive(Debug)]
pub struct Connection {
    pub peer_addr: std::net::SocketAddr,
}

#[derive(Debug)]
pub struct Session {
    pub id: String,
    pub peer_addr: std::net::SocketAddr,
}

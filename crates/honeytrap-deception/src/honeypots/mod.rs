pub mod http;
pub mod mysql;
pub mod ssh;

pub use http::HttpHoneypot;
pub use mysql::MysqlHoneypot;
pub use ssh::SshHoneypot;

use async_trait::async_trait;
use std::fmt;
use std::sync::Arc;

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

/// Connection mit Quinn QUIC-Support
#[derive(Debug, Clone)]
pub struct Connection {
    pub peer_addr: std::net::SocketAddr,
    /// Optional Quinn QUIC Connection
    /// Wird nur gesetzt wenn QUIC verwendet wird
    #[cfg(feature = "quic")]
    pub quinn_connection: Option<Arc<quinn::Connection>>,
    
    #[cfg(not(feature = "quic"))]
    pub quinn_connection: Option<Arc<()>>, // Placeholder wenn QUIC disabled
}

impl Connection {
    /// Neue Connection ohne QUIC
    pub fn new(peer_addr: std::net::SocketAddr) -> Self {
        Self {
            peer_addr,
            quinn_connection: None,
        }
    }

    /// Neue Connection mit Quinn QUIC
    #[cfg(feature = "quic")]
    pub fn with_quic(peer_addr: std::net::SocketAddr, quinn: Arc<quinn::Connection>) -> Self {
        Self {
            peer_addr,
            quinn_connection: Some(quinn),
        }
    }

    /// QUIC Bi-Stream öffnen
    #[cfg(feature = "quic")]
    pub async fn open_bi(&self) -> Result<(quinn::SendStream, quinn::RecvStream), Box<dyn std::error::Error>> {
        if let Some(ref conn) = self.quinn_connection {
            Ok(conn.open_bi().await?)
        } else {
            Err("No QUIC connection available".into())
        }
    }

    /// QUIC Uni-Stream öffnen
    #[cfg(feature = "quic")]
    pub async fn open_uni(&self) -> Result<quinn::SendStream, Box<dyn std::error::Error>> {
        if let Some(ref conn) = self.quinn_connection {
            Ok(conn.open_uni().await?)
        } else {
            Err("No QUIC connection available".into())
        }
    }

    /// QUIC Bi-Stream akzeptieren
    #[cfg(feature = "quic")]
    pub async fn accept_bi(&self) -> Result<(quinn::SendStream, quinn::RecvStream), Box<dyn std::error::Error>> {
        if let Some(ref conn) = self.quinn_connection {
            Ok(conn.accept_bi().await?)
        } else {
            Err("No QUIC connection available".into())
        }
    }

    /// QUIC Uni-Stream akzeptieren
    #[cfg(feature = "quic")]
    pub async fn accept_uni(&self) -> Result<quinn::RecvStream, Box<dyn std::error::Error>> {
        if let Some(ref conn) = self.quinn_connection {
            Ok(conn.accept_uni().await?)
        } else {
            Err("No QUIC connection available".into())
        }
    }

    /// Connection schließen
    pub async fn close(&self) {
        #[cfg(feature = "quic")]
        if let Some(ref conn) = self.quinn_connection {
            conn.close(0u32.into(), b"connection closed");
        }
    }
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

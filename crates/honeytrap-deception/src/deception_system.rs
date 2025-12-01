use crate::honeypots::{Honeypot, HoneypotType, HttpHoneypot, MysqlHoneypot, SshHoneypot};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::RwLock;

/// Deception System - Verwaltet alle Honeypots
pub struct DeceptionSystem {
    honeypots: RwLock<HashMap<u16, Box<dyn Honeypot>>>,
    active_sessions: AtomicUsize,
    blocked_ips: RwLock<std::collections::HashSet<std::net::IpAddr>>,
}

impl Default for DeceptionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl DeceptionSystem {
    /// Neues Deception System
    pub fn new() -> Self {
        Self {
            honeypots: RwLock::new(HashMap::new()),
            active_sessions: AtomicUsize::new(0),
            blocked_ips: RwLock::new(std::collections::HashSet::new()),
        }
    }

    /// Honeypot deployen
    pub async fn deploy_honeypot(
        &self,
        config: HoneypotConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let honeypot: Box<dyn Honeypot> = match config.honeypot_type {
            HoneypotType::Ssh => Box::new(SshHoneypot::new(config.port)),
            HoneypotType::Http => Box::new(HttpHoneypot::new(config.port)),
            HoneypotType::Mysql => Box::new(MysqlHoneypot::new(config.port)),
        };

        tracing::info!(
            "🍯 Deploying {} honeypot on port {}",
            config.honeypot_type,
            config.port
        );

        let mut honeypots = self.honeypots.write().await;
        honeypots.insert(config.port, honeypot);

        Ok(())
    }

    /// Connection verarbeiten
    pub async fn handle_connection(
        &self,
        connection: crate::Connection,
        session: crate::Session,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.active_sessions.fetch_add(1, Ordering::SeqCst);

        tracing::info!(
            "🍯 Honeypot handling connection from {} (session: {})",
            session.peer_addr,
            session.id
        );

        // TODO: Honeypot auswählen basierend auf Ziel-Port
        // Für jetzt: SSH als Default
        let honeypots = self.honeypots.read().await;
        if let Some(honeypot) = honeypots.get(&22) {
            honeypot.handle(connection, session).await?;
        }

        self.active_sessions.fetch_sub(1, Ordering::SeqCst);

        Ok(())
    }

    /// IP blockieren
    pub async fn block_ip(&self, ip: std::net::IpAddr) {
        let mut blocked = self.blocked_ips.write().await;
        blocked.insert(ip);
        tracing::warn!("🚫 Blocked IP: {}", ip);
    }

    /// Report generieren
    pub async fn generate_report(&self) -> DeceptionReport {
        let honeypots = self.honeypots.read().await;
        let blocked = self.blocked_ips.read().await;

        DeceptionReport {
            active_sessions: self.active_sessions.load(Ordering::SeqCst),
            blocked_count: blocked.len(),
            honeypot_count: honeypots.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HoneypotConfig {
    pub port: u16,
    pub honeypot_type: HoneypotType,
    pub interaction_level: InteractionLevel,
}

#[derive(Debug, Clone)]
pub enum InteractionLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub struct DeceptionReport {
    pub active_sessions: usize,
    pub blocked_count: usize,
    pub honeypot_count: usize,
}

impl DeceptionReport {
    pub fn active_sessions(&self) -> usize {
        self.active_sessions
    }

    pub fn blocked_count(&self) -> usize {
        self.blocked_count
    }
}

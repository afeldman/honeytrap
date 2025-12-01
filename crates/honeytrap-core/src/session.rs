use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

/// Session-Tracking für einzelne Verbindungen
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub peer_addr: SocketAddr,
    pub started_at: Instant,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub is_suspicious: bool,
    pub anomaly_score: f64,
}

impl Session {
    /// Neue Session erstellen
    pub fn new(peer_addr: SocketAddr) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            peer_addr,
            started_at: Instant::now(),
            bytes_sent: 0,
            bytes_received: 0,
            is_suspicious: false,
            anomaly_score: 0.0,
        }
    }
    
    /// Session-Dauer berechnen
    pub fn duration(&self) -> Duration {
        self.started_at.elapsed()
    }
    
    /// Bytes hinzufügen
    pub fn add_bytes_sent(&mut self, bytes: u64) {
        self.bytes_sent += bytes;
    }
    
    pub fn add_bytes_received(&mut self, bytes: u64) {
        self.bytes_received += bytes;
    }
    
    /// Als verdächtig markieren
    pub fn mark_suspicious(&mut self, score: f64) {
        self.is_suspicious = true;
        self.anomaly_score = score;
    }
}

/// Session Manager für alle aktiven Sessions
pub struct SessionManager {
    sessions: tokio::sync::RwLock<std::collections::HashMap<String, Session>>,
    event_tx: mpsc::UnboundedSender<SessionEvent>,
}

#[derive(Debug, Clone)]
pub enum SessionEvent {
    Created(Session),
    Updated(Session),
    Closed(String),
    Suspicious(Session),
}

impl SessionManager {
    /// Neuer Session Manager
    pub fn new() -> (Self, mpsc::UnboundedReceiver<SessionEvent>) {
        let (tx, rx) = mpsc::unbounded_channel();
        
        (
            Self {
                sessions: tokio::sync::RwLock::new(std::collections::HashMap::new()),
                event_tx: tx,
            },
            rx,
        )
    }
    
    /// Neue Session registrieren
    pub async fn register(&self, peer_addr: SocketAddr) -> Session {
        let session = Session::new(peer_addr);
        
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.clone(), session.clone());
        
        let _ = self.event_tx.send(SessionEvent::Created(session.clone()));
        
        session
    }
    
    /// Session aktualisieren
    pub async fn update(&self, session: Session) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.clone(), session.clone());
        
        let _ = self.event_tx.send(SessionEvent::Updated(session));
    }
    
    /// Session schließen
    pub async fn close(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
        
        let _ = self.event_tx.send(SessionEvent::Closed(session_id.to_string()));
    }
    
    /// Session als verdächtig markieren
    pub async fn mark_suspicious(&self, session_id: &str, score: f64) {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            session.mark_suspicious(score);
            let _ = self.event_tx.send(SessionEvent::Suspicious(session.clone()));
        }
    }
    
    /// Alle aktiven Sessions
    pub async fn active_sessions(&self) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions.values().cloned().collect()
    }
    
    /// Anzahl aktiver Sessions
    pub async fn count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }
}

use crate::session::{Session, SessionManager};
use honeytrap_ai::AnomalyDetector;
use honeytrap_deception::{Connection, DeceptionSystem};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Router - Leitet Traffic basierend auf AI-Analyse
pub struct Router {
    ai_engine: Arc<RwLock<AnomalyDetector>>,
    deception: Arc<DeceptionSystem>,
    session_manager: Arc<SessionManager>,
    total_connections: AtomicU64,
    anomalies_detected: AtomicU64,
}

impl Router {
    /// Neuer Router
    pub fn new(ai_engine: Arc<RwLock<AnomalyDetector>>, deception: Arc<DeceptionSystem>) -> Self {
        let (session_manager, _event_rx) = SessionManager::new();

        Self {
            ai_engine,
            deception,
            session_manager: Arc::new(session_manager),
            total_connections: AtomicU64::new(0),
            anomalies_detected: AtomicU64::new(0),
        }
    }

    /// Verbindung verarbeiten
    pub async fn handle_connection(
        &self,
        connection: Connection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Statistik
        self.total_connections.fetch_add(1, Ordering::SeqCst);

        // Session erstellen
        let mut session = self.session_manager.register(connection.peer_addr).await;

        tracing::debug!(
            "📊 Session {} created for {}",
            session.id,
            session.peer_addr
        );

        // Features extrahieren
        let features = self.extract_features(&connection, &session).await;

        // AI-Analyse
        let mut ai = self.ai_engine.write().await;
        let (is_anomaly, score) = ai.analyze(&features).await?;
        drop(ai);

        if is_anomaly {
            self.anomalies_detected.fetch_add(1, Ordering::SeqCst);
            session.mark_suspicious(score);

            tracing::warn!(
                "🚨 Anomaly detected! Session {} from {} (score: {:.2})",
                session.id,
                session.peer_addr,
                score
            );

            // Zu Honeypot umleiten
            self.redirect_to_honeypot(connection, session).await?;
        } else {
            tracing::debug!(
                "✅ Normal traffic from {} (score: {:.2})",
                session.peer_addr,
                score
            );

            // Normale Weiterleitung (TODO: Backend)
            self.forward_to_backend(connection, session).await?;
        }

        Ok(())
    }

    /// Features aus Connection extrahieren
    async fn extract_features(&self, connection: &Connection, session: &Session) -> Vec<f64> {
        // TODO: Echte Feature-Extraktion
        // Für jetzt: Dummy-Features
        vec![
            connection.peer_addr.port() as f64,
            session.duration().as_secs_f64(),
            session.bytes_sent as f64,
            session.bytes_received as f64,
        ]
    }

    /// Zu Honeypot umleiten
    async fn redirect_to_honeypot(
        &self,
        connection: Connection,
        session: Session,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("🍯 Redirecting session {} to honeypot", session.id);

        // Session als suspicious markieren
        self.session_manager
            .mark_suspicious(&session.id, session.anomaly_score)
            .await;

        // Session in Deception-Format konvertieren
        let deception_session = honeytrap_deception::honeypots::Session {
            id: session.id.clone(),
            peer_addr: session.peer_addr,
            started_at: session.started_at,
            bytes_sent: session.bytes_sent,
            bytes_received: session.bytes_received,
            is_suspicious: session.is_suspicious,
            anomaly_score: session.anomaly_score,
        };

        // An Deception System übergeben
        self.deception
            .handle_connection(connection, deception_session)
            .await?;

        Ok(())
    }

    /// Zu Backend weiterleiten
    async fn forward_to_backend(
        &self,
        _connection: Connection,
        session: Session,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("➡️ Forwarding session {} to backend", session.id);

        // TODO: Implementierung der Backend-Weiterleitung
        // Für jetzt: Connection wird automatisch geschlossen (Drop)

        // Session schließen
        self.session_manager.close(&session.id).await;

        Ok(())
    }

    /// Statistiken
    pub fn total_connections(&self) -> u64 {
        self.total_connections.load(Ordering::SeqCst)
    }

    pub fn anomalies_detected(&self) -> u64 {
        self.anomalies_detected.load(Ordering::SeqCst)
    }
}

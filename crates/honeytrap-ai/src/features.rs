use std::net::SocketAddr;
use std::time::Duration;

/// Netzwerk-Features für ML
#[derive(Debug, Clone)]
pub struct NetworkFeatures {
    // Connection Info
    pub source_port: u16,
    pub destination_port: u16,
    
    // Timing
    pub connection_duration: f64,
    pub inter_packet_time: f64,
    
    // Traffic
    pub bytes_sent: f64,
    pub bytes_received: f64,
    pub packets_sent: f64,
    pub packets_received: f64,
    
    // Behavioral
    pub failed_login_attempts: f64,
    pub command_frequency: f64,
}

impl NetworkFeatures {
    /// Features als Vektor
    pub fn as_vector(&self) -> Vec<f64> {
        vec![
            self.source_port as f64,
            self.destination_port as f64,
            self.connection_duration,
            self.inter_packet_time,
            self.bytes_sent,
            self.bytes_received,
            self.packets_sent,
            self.packets_received,
            self.failed_login_attempts,
            self.command_frequency,
        ]
    }
    
    /// Feature-Namen
    pub fn feature_names() -> Vec<&'static str> {
        vec![
            "source_port",
            "destination_port",
            "connection_duration",
            "inter_packet_time",
            "bytes_sent",
            "bytes_received",
            "packets_sent",
            "packets_received",
            "failed_login_attempts",
            "command_frequency",
        ]
    }
}

impl Default for NetworkFeatures {
    fn default() -> Self {
        Self {
            source_port: 0,
            destination_port: 0,
            connection_duration: 0.0,
            inter_packet_time: 0.0,
            bytes_sent: 0.0,
            bytes_received: 0.0,
            packets_sent: 0.0,
            packets_received: 0.0,
            failed_login_attempts: 0.0,
            command_frequency: 0.0,
        }
    }
}

/// Feature Extractor
pub struct FeatureExtractor;

impl FeatureExtractor {
    /// Features aus Connection-Daten extrahieren
    pub fn extract(
        peer_addr: SocketAddr,
        duration: Duration,
        bytes_sent: u64,
        bytes_received: u64,
    ) -> NetworkFeatures {
        NetworkFeatures {
            source_port: peer_addr.port(),
            destination_port: 8443, // Default HoneyTrap Port
            connection_duration: duration.as_secs_f64(),
            inter_packet_time: 0.0, // TODO: Calculate
            bytes_sent: bytes_sent as f64,
            bytes_received: bytes_received as f64,
            packets_sent: 0.0, // TODO: Count
            packets_received: 0.0, // TODO: Count
            failed_login_attempts: 0.0, // TODO: Track
            command_frequency: 0.0, // TODO: Calculate
        }
    }
}

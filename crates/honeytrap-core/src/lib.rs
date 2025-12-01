pub mod config;
pub mod session;
pub mod router;

pub use config::Config;
pub use session::{Session, SessionManager};
pub use router::Router;

use honeytrap_ai::{AnomalyDetector, LLMClient, LLMProvider};
use honeytrap_deception::DeceptionSystem;
use honeytrap_protocol::SecureQuicTransport;
use std::sync::Arc;
use tokio::sync::RwLock;

/// HoneyTrap - Hauptsystem
pub struct HoneyTrap {
    /// AI-Engine für Anomalie-Erkennung
    pub ai_engine: Arc<RwLock<AnomalyDetector>>,
    
    /// Deception System (Honeypots)
    pub deception: Arc<DeceptionSystem>,
    
    /// Secure Transport
    pub transport: Arc<SecureQuicTransport>,
    
    /// Router für Traffic-Handling
    pub router: Arc<Router>,
    
    /// Konfiguration
    pub config: Config,
}

impl HoneyTrap {
    /// Neues HoneyTrap System initialisieren
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        tracing::info!("🍯 Initializing HoneyTrap v{}", env!("CARGO_PKG_VERSION"));
        
        // AI Engine
        let mut detector = AnomalyDetector::new(config.ai.window_size);
        
        // LLM Integration
        if config.llm.enabled {
            if let Some(api_key) = &config.llm.api_key {
                let provider = match config.llm.provider.as_str() {
                    "deepseek" => LLMProvider::DeepSeek {
                        api_key: api_key.clone(),
                        model: config.llm.model.clone(),
                    },
                    "openai" => LLMProvider::OpenAI {
                        api_key: api_key.clone(),
                        model: config.llm.model.clone(),
                    },
                    _ => {
                        tracing::warn!("Unknown LLM provider: {}, using DeepSeek", config.llm.provider);
                        LLMProvider::DeepSeek {
                            api_key: api_key.clone(),
                            model: config.llm.model.clone(),
                        }
                    }
                };
                
                let llm_client = LLMClient::new(provider);
                detector = detector.with_llm(llm_client);
                tracing::info!("🧠 LLM enabled: {} ({})", config.llm.provider, config.llm.model);
            } else {
                tracing::warn!("LLM enabled but no API key provided");
            }
        }
        
        let ai_engine = Arc::new(RwLock::new(detector));
        
        // Deception System
        let deception = Arc::new(DeceptionSystem::new());
        
        // Deploy configured honeypots
        for honeypot_config in &config.honeypots {
            let hp_config = honeytrap_deception::HoneypotConfig {
                port: honeypot_config.port,
                honeypot_type: match honeypot_config.service_type.as_str() {
                    "ssh" => honeytrap_deception::HoneypotType::Ssh,
                    "http" => honeytrap_deception::HoneypotType::Http,
                    "mysql" => honeytrap_deception::HoneypotType::Mysql,
                    _ => honeytrap_deception::HoneypotType::Ssh,
                },
                interaction_level: match honeypot_config.interaction_level.as_str() {
                    "low" => honeytrap_deception::InteractionLevel::Low,
                    "medium" => honeytrap_deception::InteractionLevel::Medium,
                    "high" => honeytrap_deception::InteractionLevel::High,
                    _ => honeytrap_deception::InteractionLevel::Medium,
                },
            };
            deception.deploy_honeypot(hp_config).await?;
        }
        
        // Transport
        let transport = Arc::new(
            SecureQuicTransport::new_server(config.network.bind_addr).await?
        );
        
        // Router
        let router = Arc::new(Router::new(
            ai_engine.clone(),
            deception.clone(),
        ));
        
        tracing::info!("✅ HoneyTrap initialized successfully");
        
        Ok(Self {
            ai_engine,
            deception,
            transport,
            router,
            config,
        })
    }
    
    /// HoneyTrap starten
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("🚀 Starting HoneyTrap on {}", self.config.network.bind_addr);
        
        loop {
            // Eingehende Verbindung
            let (connection, peer_addr) = self.transport.accept().await?;
            
            tracing::debug!("📥 New connection from {}", peer_addr);
            
            // Router-Handler
            let router = self.router.clone();
            tokio::spawn(async move {
                if let Err(e) = router.handle_connection(connection).await {
                    tracing::error!("Connection handler error: {}", e);
                }
            });
        }
    }
    
    /// Statistiken abrufen
    pub async fn stats(&self) -> HoneyTrapStats {
        let ai = self.ai_engine.read().await;
        let deception = self.deception.generate_report().await;
        
        HoneyTrapStats {
            total_connections: self.router.total_connections(),
            anomalies_detected: ai.anomalies_detected(),
            active_honeypots: deception.active_sessions(),
            blocked_ips: deception.blocked_count(),
            model_accuracy: ai.model_accuracy(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HoneyTrapStats {
    pub total_connections: u64,
    pub anomalies_detected: u64,
    pub active_honeypots: usize,
    pub blocked_ips: usize,
    pub model_accuracy: f64,
}

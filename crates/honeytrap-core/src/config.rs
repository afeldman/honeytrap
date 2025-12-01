use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub ai: AIConfig,
    pub honeypots: Vec<HoneypotConfig>,
    pub security: SecurityConfig,
    #[serde(default)]
    pub llm: LLMConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub bind_addr: SocketAddr,
    pub enable_quic: bool,
    pub enable_nat_traversal: bool,
    pub stun_servers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AIConfig {
    pub window_size: usize,
    pub anomaly_threshold: f64,
    pub model_path: Option<String>,
    pub training_enabled: bool,
    pub auto_retrain_interval: u64, // seconds
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HoneypotConfig {
    pub port: u16,
    pub service_type: String,
    pub interaction_level: String,
    pub auto_deploy: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecurityConfig {
    pub max_failed_attempts: u32,
    pub block_duration: u64, // seconds
    pub enable_tarpit: bool,
    pub tarpit_delay: u64, // seconds
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LLMConfig {
    pub enabled: bool,
    pub provider: String, // "deepseek" or "openai"
    pub api_key: Option<String>,
    pub model: String,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "deepseek".to_string(),
            api_key: None,
            model: "deepseek-chat".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                bind_addr: "0.0.0.0:8443".parse().unwrap(),
                enable_quic: true,
                enable_nat_traversal: true,
                stun_servers: vec!["stun:stun.l.google.com:19302".to_string()],
            },
            ai: AIConfig {
                window_size: 100,
                anomaly_threshold: 0.7,
                model_path: Some("./models/honeytrap.pkl".to_string()),
                training_enabled: true,
                auto_retrain_interval: 86400, // 24h
            },
            honeypots: vec![
                HoneypotConfig {
                    port: 22,
                    service_type: "ssh".to_string(),
                    interaction_level: "medium".to_string(),
                    auto_deploy: true,
                },
                HoneypotConfig {
                    port: 80,
                    service_type: "http".to_string(),
                    interaction_level: "high".to_string(),
                    auto_deploy: true,
                },
            ],
            security: SecurityConfig {
                max_failed_attempts: 5,
                block_duration: 3600,
                enable_tarpit: true,
                tarpit_delay: 300,
            },
            llm: LLMConfig::default(),
        }
    }
}

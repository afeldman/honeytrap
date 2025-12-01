use serde::{Deserialize, Serialize};

/// Policy file structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PolicyFile {
    pub version: String,
    pub policies: Vec<Policy>,
}

/// A single policy definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Policy {
    pub name: String,
    pub description: String,
    pub priority: u32,
    pub conditions: PolicyConditions,
    pub action: PolicyAction,
}

/// Conditions for a policy
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PolicyConditions {
    #[serde(default)]
    pub any: Vec<Condition>,
    #[serde(default)]
    pub all: Vec<Condition>,
}

/// A single condition
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Condition {
    #[serde(default)]
    pub mtls_verified: Option<bool>,
    #[serde(default)]
    pub client_san_contains: Option<String>,
    #[serde(default)]
    pub max_risk_score: Option<u32>,
    #[serde(default)]
    pub min_risk_score: Option<u32>,
    #[serde(default)]
    pub src_ip_in_cidr: Option<Vec<String>>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub request_path_contains: Option<Vec<String>>,
    #[serde(default)]
    pub payload_regex: Option<Vec<String>>,
    #[serde(default)]
    pub failed_logins_last_60s_gte: Option<u32>,
}

/// Action to take when policy matches
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PolicyAction {
    #[serde(rename = "type")]
    pub action_type: ActionType,
    #[serde(default)]
    pub log: bool,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub deception_profile: Option<String>,
    #[serde(default)]
    pub tarpit: Option<TarpitConfig>,
}

/// Tarpit configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TarpitConfig {
    pub enabled: bool,
    #[serde(default)]
    pub max_delay_ms: Option<u32>,
}

/// Types of actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ActionType {
    #[serde(rename = "ALLOW")]
    Allow,
    #[serde(rename = "BLOCK")]
    Block,
    #[serde(rename = "DECEPTION")]
    Deception,
}

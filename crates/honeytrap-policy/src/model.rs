//! Policy Data Model Module
//!
//! This module defines the data structures for policy files and their components.
//!
//! # Structure
//!
//! A policy file contains:
//! - **PolicyFile**: Top-level container with version and list of policies
//! - **Policy**: Individual rule with conditions and an action
//! - **PolicyConditions**: "any" (OR) and "all" (AND) condition groups
//! - **Condition**: Individual matching criteria (IP, protocol, risk score, etc.)
//! - **PolicyAction**: What to do when a policy matches (allow/block/deception)
//! - **ActionType**: Enum of possible actions
//!
//! # Example Policy Structure
//!
//! ```yaml
//! version: v1
//! policies:
//!   - name: block_brute_force
//!     description: "Block SSH brute force attempts"
//!     priority: 90
//!     conditions:
//!       all:
//!         - protocol: "ssh"
//!         - failed_logins_last_60s_gte: 5
//!     action:
//!       type: BLOCK
//!       log: true
//!       reason: "SSH brute force detected"
//! ```

use serde::{Deserialize, Serialize};

/// Top-level policy file structure.
///
/// Represents a complete policy file containing a version identifier
/// and a list of policy rules.
///
/// # Examples
///
/// ```
/// use honeytrap_policy::model::{PolicyFile, Policy, PolicyConditions, PolicyAction, ActionType};
///
/// let policy_file = PolicyFile {
///     version: "v1".to_string(),
///     policies: vec![
///         Policy {
///             name: "test_policy".to_string(),
///             description: "Test policy".to_string(),
///             priority: 50,
///             conditions: PolicyConditions {
///                 all: vec![],
///                 any: vec![],
///             },
///             action: PolicyAction {
///                 action_type: ActionType::Allow,
///                 log: true,
///                 reason: None,
///                 deception_profile: None,
///                 tarpit: None,
///             },
///         },
///     ],
/// };
///
/// assert_eq!(policy_file.version, "v1");
/// assert_eq!(policy_file.policies.len(), 1);
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PolicyFile {
    /// Version identifier for the policy file format
    pub version: String,
    
    /// List of policy rules
    pub policies: Vec<Policy>,
}

/// A single policy rule definition.
///
/// Policies consist of conditions that must be met and an action to take
/// when those conditions match. Policies are evaluated in priority order
/// (higher priority numbers first).
///
/// # Examples
///
/// ```
/// use honeytrap_policy::model::{Policy, PolicyConditions, PolicyAction, ActionType, Condition};
///
/// let policy = Policy {
///     name: "allow_internal".to_string(),
///     description: "Allow connections from internal network".to_string(),
///     priority: 100,
///     conditions: PolicyConditions {
///         all: vec![
///             Condition {
///                 mtls_verified: Some(true),
///                 ..Default::default()
///             },
///         ],
///         any: vec![],
///     },
///     action: PolicyAction {
///         action_type: ActionType::Allow,
///         log: true,
///         reason: Some("Trusted internal service".to_string()),
///         deception_profile: None,
///         tarpit: None,
///     },
/// };
///
/// assert_eq!(policy.priority, 100);
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Policy {
    /// Unique identifier for the policy
    pub name: String,
    
    /// Human-readable description of what the policy does
    pub description: String,
    
    /// Priority (higher values evaluated first, range: 0-4294967295)
    pub priority: u32,
    
    /// Conditions that must match for this policy to trigger
    pub conditions: PolicyConditions,
    
    /// Action to take when conditions match
    pub action: PolicyAction,
}

/// Grouping of conditions with AND/OR logic.
///
/// A policy's conditions can be grouped into:
/// - `all`: All conditions must match (AND logic)
/// - `any`: At least one condition must match (OR logic)
///
/// If both are specified, `all` takes precedence.
///
/// # Examples
///
/// ```
/// use honeytrap_policy::model::{PolicyConditions, Condition};
///
/// // Match if ALL conditions are true
/// let all_conditions = PolicyConditions {
///     all: vec![
///         Condition {
///             protocol: Some("ssh".to_string()),
///             min_risk_score: Some(50),
///             ..Default::default()
///         },
///     ],
///     any: vec![],
/// };
///
/// // Match if ANY condition is true
/// let any_conditions = PolicyConditions {
///     all: vec![],
///     any: vec![
///         Condition {
///             protocol: Some("ssh".to_string()),
///             ..Default::default()
///         },
///         Condition {
///             protocol: Some("telnet".to_string()),
///             ..Default::default()
///         },
///     ],
/// };
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PolicyConditions {
    /// Conditions joined with AND logic (all must match)
    #[serde(default)]
    pub any: Vec<Condition>,
    
    /// Conditions joined with OR logic (at least one must match)
    #[serde(default)]
    pub all: Vec<Condition>,
}

/// Individual matching condition.
///
/// A condition specifies criteria that a connection must meet. All specified
/// fields in a condition must match for the condition to be true. Fields set
/// to `None` are ignored (wildcards).
///
/// # Available Condition Fields
///
/// - **mtls_verified**: Whether mutual TLS authentication succeeded
/// - **client_san_contains**: Client certificate SAN must contain this string
/// - **max_risk_score**: AI risk score must be <= this value (0-100)
/// - **min_risk_score**: AI risk score must be >= this value (0-100)
/// - **src_ip_in_cidr**: Source IP must be in one of these CIDR ranges
/// - **protocol**: Protocol must match (e.g., "ssh", "http", "mysql")
/// - **request_path_contains**: HTTP path must contain one of these strings
/// - **payload_regex**: Payload must match one of these regex patterns
/// - **failed_logins_last_60s_gte**: Failed logins in last 60s must be >= this
///
/// # Examples
///
/// ```
/// use honeytrap_policy::model::Condition;
///
/// // Match high-risk SSH connections
/// let condition = Condition {
///     protocol: Some("ssh".to_string()),
///     min_risk_score: Some(70),
///     ..Default::default()
/// };
///
/// // Match specific IP ranges
/// let ip_condition = Condition {
///     src_ip_in_cidr: Some(vec![
///         "203.0.113.0/24".to_string(),
///         "198.51.100.0/24".to_string(),
///     ]),
///     ..Default::default()
/// };
///
/// // Match brute force attempts
/// let brute_force = Condition {
///     protocol: Some("ssh".to_string()),
///     failed_logins_last_60s_gte: Some(5),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Condition {
    /// Whether mutual TLS client authentication was verified
    #[serde(default)]
    pub mtls_verified: Option<bool>,
    
    /// Client certificate SAN must contain this substring
    #[serde(default)]
    pub client_san_contains: Option<String>,
    
    /// Maximum allowed AI risk score (0-100, higher = more suspicious)
    #[serde(default)]
    pub max_risk_score: Option<u32>,
    
    /// Minimum required AI risk score (0-100, higher = more suspicious)
    #[serde(default)]
    pub min_risk_score: Option<u32>,
    
    /// Source IP must be in one of these CIDR ranges (e.g., "192.168.0.0/16")
    #[serde(default)]
    pub src_ip_in_cidr: Option<Vec<String>>,
    
    /// Protocol identifier (e.g., "ssh", "http", "mysql")
    #[serde(default)]
    pub protocol: Option<String>,
    
    /// HTTP request path must contain one of these strings
    #[serde(default)]
    pub request_path_contains: Option<Vec<String>>,
    
    /// Payload must match one of these regex patterns
    #[serde(default)]
    pub payload_regex: Option<Vec<String>>,
    
    /// Number of failed login attempts in last 60 seconds must be >= this value
    #[serde(default)]
    pub failed_logins_last_60s_gte: Option<u32>,
}

/// Action to take when a policy matches.
///
/// Defines what should happen when a policy's conditions are met, including
/// the type of action, logging behavior, and optional metadata.
///
/// # Examples
///
/// ```
/// use honeytrap_policy::model::{PolicyAction, ActionType, TarpitConfig};
///
/// // Simple block action
/// let block_action = PolicyAction {
///     action_type: ActionType::Block,
///     log: true,
///     reason: Some("Known malicious IP".to_string()),
///     deception_profile: None,
///     tarpit: None,
/// };
///
/// // Deception with tarpit
/// let deception_action = PolicyAction {
///     action_type: ActionType::Deception,
///     log: true,
///     reason: Some("Suspicious activity detected".to_string()),
///     deception_profile: Some("ssh-advanced".to_string()),
///     tarpit: Some(TarpitConfig {
///         enabled: true,
///         max_delay_ms: Some(5000),
///     }),
/// };
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PolicyAction {
    /// Type of action to take
    #[serde(rename = "type")]
    pub action_type: ActionType,
    
    /// Whether to log this action
    #[serde(default)]
    pub log: bool,
    
    /// Human-readable reason for this action
    #[serde(default)]
    pub reason: Option<String>,
    
    /// Deception profile to use (only for Deception action type)
    #[serde(default)]
    pub deception_profile: Option<String>,
    
    /// Tarpit configuration to slow down attacker
    #[serde(default)]
    pub tarpit: Option<TarpitConfig>,
}

/// Tarpit configuration for slowing down attackers.
///
/// A tarpit intentionally delays responses to waste the attacker's time
/// and resources.
///
/// # Examples
///
/// ```
/// use honeytrap_policy::model::TarpitConfig;
///
/// // Enable tarpit with 10-second max delay
/// let tarpit = TarpitConfig {
///     enabled: true,
///     max_delay_ms: Some(10000),
/// };
///
/// // Disable tarpit
/// let no_tarpit = TarpitConfig {
///     enabled: false,
///     max_delay_ms: None,
/// };
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TarpitConfig {
    /// Whether tarpit is enabled
    pub enabled: bool,
    
    /// Maximum delay in milliseconds
    #[serde(default)]
    pub max_delay_ms: Option<u32>,
}

/// Types of actions that can be taken when a policy matches.
///
/// # Variants
///
/// - **Allow**: Permit the connection to proceed normally
/// - **Block**: Reject the connection immediately
/// - **Deception**: Redirect to a honeypot for observation and data collection
///
/// # Examples
///
/// ```
/// use honeytrap_policy::model::ActionType;
///
/// let action = ActionType::Deception;
///
/// match action {
///     ActionType::Allow => println!("Allowing connection"),
///     ActionType::Block => println!("Blocking connection"),
///     ActionType::Deception => println!("Redirecting to honeypot"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ActionType {
    /// Allow the connection to proceed
    #[serde(rename = "ALLOW")]
    Allow,
    
    /// Block/reject the connection
    #[serde(rename = "BLOCK")]
    Block,
    
    /// Redirect to deception/honeypot system
    #[serde(rename = "DECEPTION")]
    Deception,
}

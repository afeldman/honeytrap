//! Policy Engine Module
//!
//! This module provides a flexible policy evaluation engine for making decisions
//! about network traffic based on configurable rules.
//!
//! # Overview
//!
//! The PolicyEngine evaluates incoming connection contexts against a set of loaded
//! policies to determine whether traffic should be allowed, blocked, or redirected
//! to a deception (honeypot) system.
//!
//! # Architecture
//!
//! - **PolicyEngine**: Main engine that holds policies and evaluates them
//! - **Decision**: Result of policy evaluation containing the action to take
//! - **EvaluationContext**: Input data about a connection to evaluate
//! - **Policy**: Individual rule with conditions and actions (from model.rs)
//!
//! # Example
//!
//! ```no_run
//! use honeytrap_policy::{PolicyEngine, EvaluationContext, ActionType};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create engine with default action for unmatched traffic
//! let engine = PolicyEngine::new(ActionType::Deception);
//!
//! // Load policies from YAML files
//! let policy_files = vec![
//!     "policies/base-policies.yaml".to_string(),
//!     "policies/high-risk.yaml".to_string(),
//! ];
//! engine.load_policies(&policy_files).await?;
//!
//! // Evaluate a connection
//! let context = EvaluationContext {
//!     src_ip: Some("192.168.1.100".to_string()),
//!     protocol: Some("ssh".to_string()),
//!     risk_score: 75,
//!     mtls_verified: false,
//!     client_san: None,
//!     request_path: None,
//!     failed_logins_count: 0,
//! };
//!
//! let decision = engine.evaluate(&context).await;
//! println!("Action: {:?}, Reason: {:?}", decision.action, decision.reason);
//! # Ok(())
//! # }
//! ```

use crate::loader::PolicyLoader;
use crate::model::{ActionType, Policy};
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;

/// Errors that can occur during policy operations.
#[derive(Error, Debug)]
pub enum PolicyError {
    /// Failed to load policies from file(s)
    #[error("Failed to load policies: {0}")]
    LoadError(String),
}

/// Result of evaluating policies against a connection context.
///
/// Contains the decision made by the policy engine, including which action
/// to take and metadata about why that decision was made.
///
/// # Fields
///
/// * `action` - The action to take (Allow, Block, or Deception)
/// * `matched_policy` - Name of the policy that matched (if any)
/// * `reason` - Human-readable explanation for the decision
/// * `deception_profile` - Which deception profile to use (if action is Deception)
/// * `should_log` - Whether this decision should be logged
///
/// # Examples
///
/// ```
/// use honeytrap_policy::{Decision, ActionType};
///
/// let decision = Decision {
///     action: ActionType::Block,
///     matched_policy: Some("block_known_bad_ips".to_string()),
///     reason: Some("Source IP in blocklist".to_string()),
///     deception_profile: None,
///     should_log: true,
/// };
///
/// if decision.should_log {
///     println!("Policy '{}' triggered: {}",
///         decision.matched_policy.as_ref().unwrap(),
///         decision.reason.as_ref().unwrap()
///     );
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Decision {
    pub action: ActionType,
    pub matched_policy: Option<String>,
    pub reason: Option<String>,
    pub deception_profile: Option<String>,
    pub should_log: bool,
}

impl Default for Decision {
    fn default() -> Self {
        Self {
            action: ActionType::Deception,
            matched_policy: None,
            reason: Some("Default action".to_string()),
            deception_profile: None,
            should_log: true,
        }
    }
}

/// Context information about a connection for policy evaluation.
///
/// This struct contains all the relevant information about an incoming connection
/// that policies can use to make decisions. Not all fields need to be populated;
/// use `None` or default values for unavailable data.
///
/// # Fields
///
/// * `src_ip` - Source IP address of the connection
/// * `protocol` - Protocol being used (e.g., "ssh", "http", "mysql")
/// * `risk_score` - AI-calculated risk score (0-100, higher = more suspicious)
/// * `mtls_verified` - Whether mutual TLS authentication succeeded
/// * `client_san` - Subject Alternative Name from client certificate (if mTLS)
/// * `request_path` - HTTP request path (for HTTP traffic)
/// * `failed_logins_count` - Number of failed login attempts in the last 60 seconds
///
/// # Examples
///
/// ```
/// use honeytrap_policy::EvaluationContext;
///
/// // Minimal context for a basic connection
/// let context = EvaluationContext {
///     src_ip: Some("203.0.113.42".to_string()),
///     protocol: Some("ssh".to_string()),
///     risk_score: 85,
///     ..Default::default()
/// };
///
/// // Full context for an mTLS-authenticated HTTP request
/// let context = EvaluationContext {
///     src_ip: Some("10.0.1.50".to_string()),
///     protocol: Some("http".to_string()),
///     risk_score: 15,
///     mtls_verified: true,
///     client_san: Some("internal.service.example.com".to_string()),
///     request_path: Some("/api/users".to_string()),
///     failed_logins_count: 0,
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct EvaluationContext {
    pub src_ip: Option<String>,
    pub protocol: Option<String>,
    pub risk_score: u32,
    pub mtls_verified: bool,
    pub client_san: Option<String>,
    pub request_path: Option<String>,
    pub failed_logins_count: u32,
}

/// Policy evaluation engine for network traffic decisions.
///
/// The PolicyEngine maintains a set of ordered policies and evaluates incoming
/// connections against them to determine appropriate actions (allow, block, or
/// deception). Policies are evaluated in priority order (highest first) until
/// a match is found.
///
/// # Thread Safety
///
/// PolicyEngine uses internal Arc and RwLock for thread-safe policy updates,
/// allowing policies to be reloaded while the engine is actively evaluating
/// connections.
///
/// # Examples
///
/// ```no_run
/// use honeytrap_policy::{PolicyEngine, EvaluationContext, ActionType};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create engine with default action for non-matching traffic
/// let engine = PolicyEngine::new(ActionType::Deception);
///
/// // Load policies from files
/// engine.load_policies(&vec![
///     "policies/allow-internal.yaml".to_string(),
///     "policies/block-threats.yaml".to_string(),
/// ]).await?;
///
/// // Evaluate connection
/// let context = EvaluationContext {
///     src_ip: Some("192.168.1.100".to_string()),
///     protocol: Some("http".to_string()),
///     risk_score: 25,
///     ..Default::default()
/// };
///
/// let decision = engine.evaluate(&context).await;
/// match decision.action {
///     ActionType::Allow => println!("Connection allowed"),
///     ActionType::Block => println!("Connection blocked"),
///     ActionType::Deception => println!("Redirecting to honeypot"),
/// }
/// # Ok(())
/// # }
/// ```
pub struct PolicyEngine {
    policies: Arc<RwLock<Vec<Policy>>>,
    default_action: ActionType,
}

impl PolicyEngine {
    /// Creates a new PolicyEngine with a specified default action.
    ///
    /// The default action is used when no policies match a given connection context.
    ///
    /// # Arguments
    ///
    /// * `default_action` - Action to take when no policy matches
    ///
    /// # Returns
    ///
    /// A new PolicyEngine instance with no policies loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use honeytrap_policy::{PolicyEngine, ActionType};
    ///
    /// // Create engine that defaults to deception
    /// let engine = PolicyEngine::new(ActionType::Deception);
    ///
    /// // Create engine that defaults to blocking
    /// let strict_engine = PolicyEngine::new(ActionType::Block);
    /// ```
    pub fn new(default_action: ActionType) -> Self {
        Self {
            policies: Arc::new(RwLock::new(Vec::new())),
            default_action,
        }
    }
    
    /// Loads policies from one or more YAML or JSON files.
    ///
    /// This method reads policy files, parses them, and stores them in priority order
    /// (higher priority values are evaluated first). Existing policies are replaced.
    ///
    /// # Arguments
    ///
    /// * `paths` - Slice of file paths to policy files (YAML or JSON format)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Policies loaded successfully
    /// * `Err(PolicyError)` - Failed to load or parse policy files
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use honeytrap_policy::{PolicyEngine, ActionType};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let engine = PolicyEngine::new(ActionType::Deception);
    ///
    /// // Load from multiple files
    /// engine.load_policies(&vec![
    ///     "policies/base-policies.yaml".to_string(),
    ///     "policies/high-risk.yaml".to_string(),
    /// ]).await?;
    ///
    /// println!("Loaded {} policies", engine.policy_count().await);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Behavior
    ///
    /// - Files that don't exist are logged as warnings but don't cause failure
    /// - Files with parse errors are logged as warnings but don't cause failure
    /// - At least one successfully loaded file results in Ok(())
    /// - All policies from all files are merged and sorted by priority
    pub async fn load_policies(&self, paths: &[String]) -> Result<(), PolicyError> {
        let policy_files = PolicyLoader::load_from_files(paths)
            .map_err(|e| PolicyError::LoadError(e.to_string()))?;
        
        let mut all_policies: Vec<Policy> = Vec::new();
        for file in policy_files {
            all_policies.extend(file.policies);
        }
        
        // Sort by priority (higher priority = checked first)
        all_policies.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        let policy_count = all_policies.len();
        let mut policies = self.policies.write().await;
        *policies = all_policies;
        
        tracing::info!("Loaded {} policies", policy_count);
        
        Ok(())
    }
    
    /// Evaluates policies against a connection context to make a decision.
    ///
    /// Iterates through policies in priority order (highest first) and returns
    /// a decision from the first matching policy. If no policies match, returns
    /// the engine's default decision.
    ///
    /// # Arguments
    ///
    /// * `context` - Connection context containing information for policy matching
    ///
    /// # Returns
    ///
    /// A Decision indicating the action to take and associated metadata
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use honeytrap_policy::{PolicyEngine, EvaluationContext, ActionType};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let engine = PolicyEngine::new(ActionType::Deception);
    /// engine.load_policies(&vec!["policies/rules.yaml".to_string()]).await?;
    ///
    /// // Evaluate a suspicious SSH connection
    /// let context = EvaluationContext {
    ///     src_ip: Some("198.51.100.42".to_string()),
    ///     protocol: Some("ssh".to_string()),
    ///     risk_score: 90,
    ///     failed_logins_count: 5,
    ///     ..Default::default()
    /// };
    ///
    /// let decision = engine.evaluate(&context).await;
    /// if let Some(policy_name) = decision.matched_policy {
    ///     println!("Matched policy: {}", policy_name);
    ///     println!("Action: {:?}", decision.action);
    ///     if let Some(reason) = decision.reason {
    ///         println!("Reason: {}", reason);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Thread Safety
    ///
    /// This method is async and acquires a read lock on the policies, allowing
    /// concurrent evaluations while preventing concurrent policy updates.
    pub async fn evaluate(&self, context: &EvaluationContext) -> Decision {
        let policies = self.policies.read().await;
        
        for policy in policies.iter() {
            if self.matches_policy(policy, context) {
                tracing::debug!("Policy matched: {}", policy.name);
                return Decision {
                    action: policy.action.action_type,
                    matched_policy: Some(policy.name.clone()),
                    reason: policy.action.reason.clone(),
                    deception_profile: policy.action.deception_profile.clone(),
                    should_log: policy.action.log,
                };
            }
        }
        
        // Default decision
        Decision {
            action: self.default_action,
            matched_policy: None,
            reason: Some("No policy matched, using default".to_string()),
            deception_profile: None,
            should_log: true,
        }
    }
    
    /// Checks if a policy matches the given evaluation context.
    ///
    /// A policy matches if:
    /// - All conditions in the "all" list match (AND logic), OR
    /// - At least one condition in the "any" list matches (OR logic), OR
    /// - The policy has no conditions (always matches)
    ///
    /// # Arguments
    ///
    /// * `policy` - The policy to check
    /// * `context` - The evaluation context to match against
    ///
    /// # Returns
    ///
    /// `true` if the policy matches, `false` otherwise
    fn matches_policy(&self, policy: &Policy, context: &EvaluationContext) -> bool {
        // Check "all" conditions (all must match)
        if !policy.conditions.all.is_empty() {
            for condition in &policy.conditions.all {
                if !self.matches_condition(condition, context) {
                    return false;
                }
            }
            return true;
        }
        
        // Check "any" conditions (at least one must match)
        if !policy.conditions.any.is_empty() {
            for condition in &policy.conditions.any {
                if self.matches_condition(condition, context) {
                    return true;
                }
            }
            return false;
        }
        
        // No conditions = always matches
        true
    }
    
    /// Checks if a single condition matches the evaluation context.
    ///
    /// Evaluates individual condition fields against the context. A condition matches
    /// only if ALL of its specified fields match the context. Fields that are `None`
    /// in the condition are not checked (wildcards).
    ///
    /// # Arguments
    ///
    /// * `condition` - The condition to evaluate
    /// * `context` - The evaluation context to match against
    ///
    /// # Returns
    ///
    /// `true` if all specified condition fields match, `false` otherwise
    ///
    /// # Supported Conditions
    ///
    /// - `protocol`: Exact protocol match (e.g., "ssh", "http")
    /// - `mtls_verified`: Whether mTLS authentication succeeded
    /// - `max_risk_score`: Risk score must be <= this value
    /// - `min_risk_score`: Risk score must be >= this value
    /// - `failed_logins_last_60s_gte`: Failed login count must be >= this value
    /// - `client_san_contains`: Client SAN must contain this substring
    fn matches_condition(&self, condition: &crate::model::Condition, context: &EvaluationContext) -> bool {
        // Protocol check
        if let Some(proto) = &condition.protocol {
            if let Some(ctx_proto) = &context.protocol {
                if proto != ctx_proto {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        // mTLS verification
        if let Some(mtls_required) = condition.mtls_verified {
            if context.mtls_verified != mtls_required {
                return false;
            }
        }
        
        // Risk score range
        if let Some(max_risk) = condition.max_risk_score {
            if context.risk_score > max_risk {
                return false;
            }
        }
        
        if let Some(min_risk) = condition.min_risk_score {
            if context.risk_score < min_risk {
                return false;
            }
        }
        
        // Failed logins threshold
        if let Some(threshold) = condition.failed_logins_last_60s_gte {
            if context.failed_logins_count < threshold {
                return false;
            }
        }
        
        // Client SAN contains
        if let Some(san_pattern) = &condition.client_san_contains {
            if let Some(client_san) = &context.client_san {
                if !client_san.contains(san_pattern) {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        true
    }
    
    /// Returns the current number of loaded policies.
    ///
    /// # Returns
    ///
    /// The count of policies currently loaded in the engine
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use honeytrap_policy::{PolicyEngine, ActionType};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let engine = PolicyEngine::new(ActionType::Deception);
    /// engine.load_policies(&vec!["policies/rules.yaml".to_string()]).await?;
    ///
    /// let count = engine.policy_count().await;
    /// println!("Loaded {} policies", count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn policy_count(&self) -> usize {
        self.policies.read().await.len()
    }
}

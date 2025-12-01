use crate::loader::PolicyLoader;
use crate::model::{ActionType, Policy};
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PolicyError {
    #[error("Failed to load policies: {0}")]
    LoadError(String),
}

/// Decision made by the policy engine
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

/// Context for policy evaluation
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

/// Policy Engine - evaluates policies against incoming connections
pub struct PolicyEngine {
    policies: Arc<RwLock<Vec<Policy>>>,
    default_action: ActionType,
}

impl PolicyEngine {
    /// Create a new policy engine
    pub fn new(default_action: ActionType) -> Self {
        Self {
            policies: Arc::new(RwLock::new(Vec::new())),
            default_action,
        }
    }
    
    /// Load policies from files
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
    
    /// Evaluate policies for a given context
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
    
    /// Check if a policy matches the context
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
    
    /// Check if a single condition matches
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
    
    /// Get current policy count
    pub async fn policy_count(&self) -> usize {
        self.policies.read().await.len()
    }
}

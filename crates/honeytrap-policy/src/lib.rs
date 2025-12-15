//! HoneyTrap Policy Engine
//!
//! This crate provides a flexible, file-based policy engine for making traffic
//! routing decisions in the HoneyTrap security system.
//!
//! # Overview
//!
//! The policy engine evaluates network traffic against configurable rules to
//! determine whether connections should be:
//! - **Allowed**: Permitted to proceed normally
//! - **Blocked**: Rejected immediately
//! - **Redirected to Deception**: Sent to honeypots for observation
//!
//! # Features
//!
//! - Load policies from YAML or JSON files
//! - Priority-based policy evaluation (higher priority checked first)
//! - Rich condition matching (IP ranges, protocols, risk scores, mTLS, etc.)
//! - Thread-safe concurrent evaluation
//! - Hot-reload support (policies can be updated at runtime)
//!
//! # Quick Start
//!
//! ```no_run
//! use honeytrap_policy::{PolicyEngine, EvaluationContext, ActionType};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create engine with default action
//! let engine = PolicyEngine::new(ActionType::Deception);
//!
//! // Load policies from files
//! engine.load_policies(&vec![
//!     "policies/base-policies.yaml".to_string(),
//! ]).await?;
//!
//! // Evaluate a connection
//! let context = EvaluationContext {
//!     src_ip: Some("192.168.1.100".to_string()),
//!     protocol: Some("ssh".to_string()),
//!     risk_score: 75,
//!     ..Default::default()
//! };
//!
//! let decision = engine.evaluate(&context).await;
//! println!("Action: {:?}", decision.action);
//! # Ok(())
//! # }
//! ```
//!
//! # Policy File Format
//!
//! Policies are defined in YAML or JSON files with the following structure:
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
//!
//! # Modules
//!
//! - [`engine`]: Core policy evaluation engine
//! - [`loader`]: Policy file loading from disk
//! - [`model`]: Data structures for policies and conditions

pub mod model;
pub mod loader;
pub mod engine;

pub use engine::{PolicyEngine, Decision, EvaluationContext};
pub use model::ActionType;

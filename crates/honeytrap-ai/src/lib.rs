pub mod anomaly_detector;
pub mod features;
pub mod llm;
pub mod random_forest;
pub mod rl_agent;

pub use anomaly_detector::AnomalyDetector;
pub use features::NetworkFeatures;
pub use llm::{BehaviorAnalysis, LLMClient, LLMProvider, SessionData};
pub use random_forest::{ModelMetrics, RandomForestModel};
pub use rl_agent::{Action, RLAgent, RLConfig, RLStats, RewardCalculator, State};

// Re-export scripting from honeytrap-scripting
pub use honeytrap_scripting::{PythonScriptEngine, RhaiScriptEngine, ScriptEngine};

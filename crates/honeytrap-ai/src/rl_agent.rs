//! Reinforcement Learning Agent für adaptive Honeypot-Strategien
//!
//! Implementiert einen Q-Learning Agenten, der optimale Antwortstrategien lernt

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

/// State representation für RL Agent
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct State {
    /// Attack type category (0-10)
    pub attack_type: u8,
    /// Connection count in last minute
    pub connection_intensity: u8,
    /// Source reputation score (0-10)
    pub source_reputation: u8,
}

impl State {
    /// Convert state to string key for serialization
    fn to_key(&self) -> String {
        format!(
            "{}-{}-{}",
            self.attack_type, self.connection_intensity, self.source_reputation
        )
    }

    /// Parse state from string key
    fn from_key(key: &str) -> Option<Self> {
        let parts: Vec<&str> = key.split('-').collect();
        if parts.len() == 3 {
            Some(State {
                attack_type: parts[0].parse().ok()?,
                connection_intensity: parts[1].parse().ok()?,
                source_reputation: parts[2].parse().ok()?,
            })
        } else {
            None
        }
    }
}

/// Action choices für Honeypot Response
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Ignore the connection
    Ignore,
    /// Minimal response (low engagement)
    MinimalResponse,
    /// Standard honeypot interaction
    StandardEngagement,
    /// Deep engagement (resource intensive)
    DeepEngagement,
    /// Immediate block
    Block,
}

impl Action {
    pub fn all() -> Vec<Action> {
        vec![
            Action::Ignore,
            Action::MinimalResponse,
            Action::StandardEngagement,
            Action::DeepEngagement,
            Action::Block,
        ]
    }
}

/// Q-Learning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RLConfig {
    /// Learning rate (alpha)
    pub learning_rate: f64,
    /// Discount factor (gamma)
    pub discount_factor: f64,
    /// Exploration rate (epsilon)
    pub epsilon: f64,
    /// Epsilon decay rate
    pub epsilon_decay: f64,
    /// Minimum epsilon
    pub epsilon_min: f64,
}

impl Default for RLConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.1,
            discount_factor: 0.95,
            epsilon: 1.0,
            epsilon_decay: 0.995,
            epsilon_min: 0.01,
        }
    }
}

/// Q-Learning Agent für adaptive Honeypot-Strategien
#[derive(Debug)]
pub struct RLAgent {
    /// Q-Table: State -> Action -> Q-Value
    q_table: HashMap<State, HashMap<Action, f64>>,
    /// Configuration
    config: RLConfig,
    /// Total training episodes
    episodes_trained: usize,
    /// Current epsilon (exploration rate)
    current_epsilon: f64,
}

/// Serializable version of RLAgent
#[derive(Debug, Serialize, Deserialize)]
struct RLAgentSerde {
    q_table: HashMap<String, HashMap<Action, f64>>,
    config: RLConfig,
    episodes_trained: usize,
    current_epsilon: f64,
}

impl RLAgent {
    /// Create new RL agent with default config
    pub fn new() -> Self {
        Self {
            q_table: HashMap::new(),
            config: RLConfig::default(),
            episodes_trained: 0,
            current_epsilon: 1.0,
        }
    }

    /// Create agent with custom config
    pub fn with_config(config: RLConfig) -> Self {
        let epsilon = config.epsilon;
        Self {
            q_table: HashMap::new(),
            config,
            episodes_trained: 0,
            current_epsilon: epsilon,
        }
    }

    /// Get Q-value for state-action pair
    fn get_q_value(&self, state: &State, action: &Action) -> f64 {
        self.q_table
            .get(state)
            .and_then(|actions| actions.get(action))
            .copied()
            .unwrap_or(0.0)
    }

    /// Update Q-value using Q-learning formula
    /// Q(s,a) = Q(s,a) + α[r + γ max Q(s',a') - Q(s,a)]
    pub fn update(
        &mut self,
        state: &State,
        action: &Action,
        reward: f64,
        next_state: &State,
    ) {
        let current_q = self.get_q_value(state, action);
        let max_next_q = self.get_max_q_value(next_state);

        let new_q = current_q
            + self.config.learning_rate
                * (reward + self.config.discount_factor * max_next_q - current_q);

        self.q_table
            .entry(state.clone())
            .or_insert_with(HashMap::new)
            .insert(*action, new_q);
    }

    /// Get maximum Q-value for a state
    fn get_max_q_value(&self, state: &State) -> f64 {
        if let Some(actions) = self.q_table.get(state) {
            actions.values().copied().fold(f64::NEG_INFINITY, f64::max)
        } else {
            0.0
        }
    }

    /// Choose action using epsilon-greedy policy
    pub fn choose_action(&self, state: &State) -> Action {
        // Exploration: random action
        if rand::random::<f64>() < self.current_epsilon {
            let actions = Action::all();
            let idx = rand::random::<usize>() % actions.len();
            return actions[idx];
        }

        // Exploitation: best known action
        self.get_best_action(state)
    }

    /// Get best action for state (greedy)
    pub fn get_best_action(&self, state: &State) -> Action {
        if let Some(actions) = self.q_table.get(state) {
            actions
                .iter()
                .max_by(|(_, q1), (_, q2)| q1.partial_cmp(q2).unwrap())
                .map(|(action, _)| *action)
                .unwrap_or(Action::StandardEngagement)
        } else {
            Action::StandardEngagement
        }
    }

    /// Decay epsilon (reduce exploration over time)
    pub fn decay_epsilon(&mut self) {
        self.current_epsilon = (self.current_epsilon * self.config.epsilon_decay)
            .max(self.config.epsilon_min);
    }

    /// Complete training episode
    pub fn finish_episode(&mut self) {
        self.episodes_trained += 1;
        self.decay_epsilon();
    }

    /// Get training statistics
    pub fn get_stats(&self) -> RLStats {
        RLStats {
            episodes_trained: self.episodes_trained,
            states_explored: self.q_table.len(),
            current_epsilon: self.current_epsilon,
            avg_q_value: self.calculate_avg_q_value(),
        }
    }

    /// Calculate average Q-value across all state-action pairs
    fn calculate_avg_q_value(&self) -> f64 {
        let mut sum = 0.0;
        let mut count = 0;

        for actions in self.q_table.values() {
            for q_value in actions.values() {
                sum += q_value;
                count += 1;
            }
        }

        if count > 0 {
            sum / count as f64
        } else {
            0.0
        }
    }

    /// Save model to file
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        // Convert to serializable format
        let mut q_table_serde = HashMap::new();
        for (state, actions) in &self.q_table {
            q_table_serde.insert(state.to_key(), actions.clone());
        }

        let agent_serde = RLAgentSerde {
            q_table: q_table_serde,
            config: self.config.clone(),
            episodes_trained: self.episodes_trained,
            current_epsilon: self.current_epsilon,
        };

        let json = serde_json::to_string_pretty(&agent_serde)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load model from file
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let json = fs::read_to_string(path)?;
        let agent_serde: RLAgentSerde = serde_json::from_str(&json)?;

        // Convert from serializable format
        let mut q_table = HashMap::new();
        for (key, actions) in agent_serde.q_table {
            if let Some(state) = State::from_key(&key) {
                q_table.insert(state, actions);
            }
        }

        Ok(RLAgent {
            q_table,
            config: agent_serde.config,
            episodes_trained: agent_serde.episodes_trained,
            current_epsilon: agent_serde.current_epsilon,
        })
    }

    /// Reset agent (clear Q-table)
    pub fn reset(&mut self) {
        self.q_table.clear();
        self.episodes_trained = 0;
        self.current_epsilon = self.config.epsilon;
    }
}

impl Default for RLAgent {
    fn default() -> Self {
        Self::new()
    }
}

/// Training statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RLStats {
    pub episodes_trained: usize,
    pub states_explored: usize,
    pub current_epsilon: f64,
    pub avg_q_value: f64,
}

/// Reward calculator für Training
pub struct RewardCalculator;

impl RewardCalculator {
    /// Calculate reward based on outcome
    /// Positive rewards: Information gained, attacker time wasted
    /// Negative rewards: Resources consumed, false positives
    pub fn calculate(
        action: &Action,
        info_gained: f64,
        time_wasted: f64,
        resources_used: f64,
    ) -> f64 {
        let base_reward = match action {
            Action::Ignore => -1.0,              // Missed opportunity
            Action::MinimalResponse => 2.0,      // Low cost, some info
            Action::StandardEngagement => 5.0,   // Good balance
            Action::DeepEngagement => 8.0,       // High info gain
            Action::Block => 1.0,                // Safe but no intel
        };

        // Adjust based on actual outcome
        base_reward + info_gained * 2.0 + time_wasted * 1.5 - resources_used * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rl_agent_creation() {
        let agent = RLAgent::new();
        assert_eq!(agent.episodes_trained, 0);
        assert_eq!(agent.current_epsilon, 1.0);
    }

    #[test]
    fn test_q_value_update() {
        let mut agent = RLAgent::new();
        let state = State {
            attack_type: 5,
            connection_intensity: 3,
            source_reputation: 2,
        };
        let next_state = State {
            attack_type: 5,
            connection_intensity: 4,
            source_reputation: 2,
        };

        agent.update(&state, &Action::StandardEngagement, 10.0, &next_state);
        let q = agent.get_q_value(&state, &Action::StandardEngagement);
        assert!(q > 0.0);
    }

    #[test]
    fn test_action_selection() {
        let mut agent = RLAgent::new();
        let state = State {
            attack_type: 3,
            connection_intensity: 2,
            source_reputation: 5,
        };

        // Train with high reward for deep engagement
        for _ in 0..10 {
            agent.update(&state, &Action::DeepEngagement, 10.0, &state);
        }

        // With low epsilon, should choose deep engagement
        agent.current_epsilon = 0.0;
        let action = agent.choose_action(&state);
        assert_eq!(action, Action::DeepEngagement);
    }

    #[test]
    fn test_epsilon_decay() {
        let mut agent = RLAgent::new();
        let initial = agent.current_epsilon;
        agent.decay_epsilon();
        assert!(agent.current_epsilon < initial);
        assert!(agent.current_epsilon >= agent.config.epsilon_min);
    }

    #[test]
    fn test_reward_calculation() {
        let reward = RewardCalculator::calculate(
            &Action::DeepEngagement,
            5.0,  // info_gained
            3.0,  // time_wasted
            2.0,  // resources_used
        );
        // 8.0 + 5.0*2.0 + 3.0*1.5 - 2.0*0.5 = 8 + 10 + 4.5 - 1 = 21.5
        assert!((reward - 21.5).abs() < 0.01);
    }

    #[test]
    fn test_save_load() {
        let mut agent = RLAgent::new();
        let state = State {
            attack_type: 1,
            connection_intensity: 2,
            source_reputation: 3,
        };
        agent.update(&state, &Action::StandardEngagement, 5.0, &state);

        let path = "/tmp/test_rl_agent.json";
        agent.save(path).unwrap();

        let loaded = RLAgent::load(path).unwrap();
        assert_eq!(loaded.episodes_trained, agent.episodes_trained);
        assert_eq!(loaded.q_table.len(), agent.q_table.len());

        std::fs::remove_file(path).ok();
    }
}

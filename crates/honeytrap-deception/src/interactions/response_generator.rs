//! Response Generator
//!
//! Intelligente Response-Strategien basierend auf Angreifer-Verhalten

use serde::{Deserialize, Serialize};

/// Response Strategy
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ResponseStrategy {
    /// Quick responses, minimal engagement
    Minimal,
    /// Standard realistic responses
    Standard,
    /// Deep engagement, waste attacker time
    Deep,
    /// Adaptive based on behavior
    Adaptive,
}

/// Response Generator
pub struct ResponseGenerator {
    strategy: ResponseStrategy,
    engagement_level: f64,
    time_wasted: std::time::Duration,
}

impl ResponseGenerator {
    pub fn new(strategy: ResponseStrategy) -> Self {
        Self {
            strategy,
            engagement_level: 0.5,
            time_wasted: std::time::Duration::from_secs(0),
        }
    }

    /// Calculate response delay based on strategy
    pub fn calculate_delay(&self, command_complexity: f64) -> std::time::Duration {
        let base_delay = match self.strategy {
            ResponseStrategy::Minimal => 50,
            ResponseStrategy::Standard => 200,
            ResponseStrategy::Deep => 1000,
            ResponseStrategy::Adaptive => (self.engagement_level * 1000.0) as u64,
        };

        let complexity_factor = (command_complexity * 500.0) as u64;
        std::time::Duration::from_millis(base_delay + complexity_factor)
    }

    /// Decide if should provide detailed error
    pub fn should_provide_detailed_error(&self) -> bool {
        match self.strategy {
            ResponseStrategy::Minimal => false,
            ResponseStrategy::Standard => true,
            ResponseStrategy::Deep => true,
            ResponseStrategy::Adaptive => self.engagement_level > 0.3,
        }
    }

    /// Decide if should simulate vulnerability
    pub fn should_simulate_vulnerability(&self) -> bool {
        match self.strategy {
            ResponseStrategy::Minimal => false,
            ResponseStrategy::Standard => false,
            ResponseStrategy::Deep => true,
            ResponseStrategy::Adaptive => self.engagement_level > 0.7,
        }
    }

    /// Update engagement level based on attacker behavior
    pub fn update_engagement(&mut self, is_sophisticated: bool, is_automated: bool) {
        if is_sophisticated && !is_automated {
            // Engage more with skilled manual attackers
            self.engagement_level = (self.engagement_level + 0.1).min(1.0);
        } else if is_automated {
            // Minimal engagement with bots
            self.engagement_level = (self.engagement_level - 0.2).max(0.1);
        }
    }

    /// Record time wasted
    pub fn add_wasted_time(&mut self, duration: std::time::Duration) {
        self.time_wasted += duration;
    }

    /// Get total time wasted
    pub fn total_time_wasted(&self) -> std::time::Duration {
        self.time_wasted
    }

    /// Get current engagement level
    pub fn engagement_level(&self) -> f64 {
        self.engagement_level
    }
}

impl Default for ResponseGenerator {
    fn default() -> Self {
        Self::new(ResponseStrategy::Standard)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_strategy() {
        let gen = ResponseGenerator::new(ResponseStrategy::Minimal);
        let delay = gen.calculate_delay(0.5);
        assert!(delay < std::time::Duration::from_millis(500));
    }

    #[test]
    fn test_deep_strategy() {
        let gen = ResponseGenerator::new(ResponseStrategy::Deep);
        let delay = gen.calculate_delay(0.5);
        assert!(delay > std::time::Duration::from_millis(1000));
    }

    #[test]
    fn test_engagement_update() {
        let mut gen = ResponseGenerator::new(ResponseStrategy::Adaptive);
        let initial = gen.engagement_level();
        
        gen.update_engagement(true, false); // Sophisticated manual attack
        assert!(gen.engagement_level() > initial);
    }

    #[test]
    fn test_time_tracking() {
        let mut gen = ResponseGenerator::new(ResponseStrategy::Standard);
        gen.add_wasted_time(std::time::Duration::from_secs(5));
        gen.add_wasted_time(std::time::Duration::from_secs(3));
        
        assert_eq!(gen.total_time_wasted(), std::time::Duration::from_secs(8));
    }
}

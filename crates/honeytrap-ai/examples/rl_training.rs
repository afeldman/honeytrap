//! Reinforcement Learning Training Example
//!
//! Zeigt Q-Learning für adaptive Honeypot-Strategien

use honeytrap_ai::{Action, RLAgent, RLConfig, RewardCalculator, State};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Reinforcement Learning Training Example\n");

    // Create RL agent with custom config
    let config = RLConfig {
        learning_rate: 0.1,
        discount_factor: 0.95,
        epsilon: 1.0,
        epsilon_decay: 0.995,
        epsilon_min: 0.01,
    };
    let mut agent = RLAgent::with_config(config);

    println!("📊 Initial Stats:");
    let stats = agent.get_stats();
    println!("   Episodes: {}", stats.episodes_trained);
    println!("   States explored: {}", stats.states_explored);
    println!("   Epsilon: {:.4}", stats.current_epsilon);
    println!("   Avg Q-value: {:.4}\n", stats.avg_q_value);

    // Simulate training episodes
    println!("🎯 Training for 1000 episodes...\n");
    
    for episode in 0..1000 {
        // Simulate different attack scenarios
        let state = State {
            attack_type: (episode % 10) as u8,
            connection_intensity: ((episode / 10) % 10) as u8,
            source_reputation: ((episode / 100) % 10) as u8,
        };

        // Agent chooses action
        let action = agent.choose_action(&state);

        // Simulate outcome and calculate reward
        let (info_gained, time_wasted, resources) = match action {
            Action::Ignore => (0.0, 0.0, 0.0),
            Action::MinimalResponse => (2.0, 1.0, 0.5),
            Action::StandardEngagement => (5.0, 3.0, 2.0),
            Action::DeepEngagement => (8.0, 5.0, 4.0),
            Action::Block => (0.5, 0.0, 0.1),
        };

        let reward = RewardCalculator::calculate(&action, info_gained, time_wasted, resources);

        // Update Q-table
        let next_state = State {
            attack_type: state.attack_type,
            connection_intensity: (state.connection_intensity + 1) % 10,
            source_reputation: state.source_reputation,
        };
        agent.update(&state, &action, reward, &next_state);

        // Complete episode
        agent.finish_episode();

        // Print progress every 100 episodes
        if (episode + 1) % 100 == 0 {
            let stats = agent.get_stats();
            println!("   Episode {}: ε={:.4}, States={}, Avg Q={:.2}", 
                     episode + 1, 
                     stats.current_epsilon,
                     stats.states_explored,
                     stats.avg_q_value);
        }
    }

    println!("\n✅ Training completed!\n");

    // Final stats
    let stats = agent.get_stats();
    println!("📈 Final Stats:");
    println!("   Episodes trained: {}", stats.episodes_trained);
    println!("   States explored: {}", stats.states_explored);
    println!("   Current epsilon: {:.4}", stats.current_epsilon);
    println!("   Avg Q-value: {:.4}\n", stats.avg_q_value);

    // Test learned policy
    println!("🎭 Testing Learned Policy:\n");
    
    let test_states = vec![
        State { attack_type: 1, connection_intensity: 2, source_reputation: 8 },
        State { attack_type: 5, connection_intensity: 8, source_reputation: 2 },
        State { attack_type: 9, connection_intensity: 9, source_reputation: 0 },
        State { attack_type: 2, connection_intensity: 3, source_reputation: 5 },
    ];

    for (i, state) in test_states.iter().enumerate() {
        let action = agent.get_best_action(state);
        println!("   Test {} - Attack:{} Intensity:{} Reputation:{}", 
                 i + 1,
                 state.attack_type,
                 state.connection_intensity,
                 state.source_reputation);
        println!("   → Recommended Action: {:?}\n", action);
    }

    // Save trained model
    let model_path = "rl_agent_model.json";
    agent.save(model_path)?;
    println!("💾 Model saved to: {}\n", model_path);

    // Load and verify
    let loaded_agent = RLAgent::load(model_path)?;
    let loaded_stats = loaded_agent.get_stats();
    println!("✅ Model loaded successfully!");
    println!("   Verified episodes: {}", loaded_stats.episodes_trained);

    Ok(())
}

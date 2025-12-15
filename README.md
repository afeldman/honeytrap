# HoneyTrap - README

🍯 **HoneyTrap** - AI-Powered Zero Trust Network Access with Intelligent Deception

## 🎯 Overview

HoneyTrap is a Rust-based security system that uses **AI-powered anomaly detection** to identify potential attackers and intelligently redirect them into **honeypots** for analysis, while allowing legitimate traffic to pass through.

## ✨ Features

- **🤖 AI Anomaly Detection**: Machine Learning-based traffic analysis
- **🧠 LLM Integration**: DeepSeek & OpenAI for intelligent behavior analysis
- **🍯 Multi-Protocol Honeypots**: SSH, HTTP, MySQL emulation
- **🔐 Secure QUIC Transport**: Modern, encrypted networking
- **📊 Real-time Monitoring**: Session tracking and statistics
- **🎯 Zero Trust Architecture**: Every connection is analyzed
- **📜 Policy Engine**: Flexible, file-based traffic routing decisions

## 🚀 Quick Start

```bash
# Build
cargo build --release

# Run server
cargo run --bin honeytrap-server

# Run with custom log level
RUST_LOG=debug cargo run --bin honeytrap-server

# Run CLI
cargo run --bin honeytrap -- --help
```

## 📁 Project Structure

```
honeytrap/
├── crates/
│   ├── honeytrap-core/          # Main orchestration
│   ├── honeytrap-ai/            # ML anomaly detection + LLM
│   ├── honeytrap-deception/     # Honeypot implementations
│   ├── honeytrap-protocol/      # QUIC transport
│   ├── honeytrap-policy/        # Policy engine (NEW)
│   ├── honeytrap-server/        # Server binary (NEW)
│   └── honeytrap-cli/           # CLI interface
├── policies/
│   ├── base-policies.yaml       # Base policy rules
│   └── high-risk.yaml           # High-risk traffic rules
├── config.toml                  # Main configuration
└── Cargo.toml                   # Workspace
```

## 🔧 Configuration

### Basic Setup

The server reads from `config.toml` in the project root. If the file doesn't exist or has parse errors, it falls back to defaults.

```toml
[network]
bind_addr = "0.0.0.0:8443"
enable_quic = true
enable_nat_traversal = true
stun_servers = ["stun:stun.l.google.com:19302"]

[ai]
window_size = 100
anomaly_threshold = 0.7
model_path = "./models/honeytrap.pkl"
training_enabled = true
auto_retrain_interval = 86400  # 24 hours in seconds

[llm]
enabled = false
provider = "deepseek"  # or "openai"
model = "deepseek-chat"  # or "gpt-4o-mini"
api_key = "sk-..."

[[honeypots]]
port = 22
service_type = "ssh"
interaction_level = "medium"
auto_deploy = true

[[honeypots]]
port = 80
service_type = "http"
interaction_level = "high"
auto_deploy = true

[security]
max_failed_attempts = 5
block_duration = 3600  # seconds
enable_tarpit = true
tarpit_delay = 300  # seconds
```

### Policy Configuration

Policies are defined in YAML files (see `policies/` directory). Example:

```yaml
version: v1
policies:
  - name: block_brute_force
    description: "Block SSH brute force attempts"
    priority: 90
    conditions:
      all:
        - protocol: "ssh"
        - failed_logins_last_60s_gte: 5
    action:
      type: BLOCK
      log: true
      reason: "SSH brute force detected"
  
  - name: allow_trusted_mtls
    description: "Allow trusted mTLS connections"
    priority: 10
    conditions:
      all:
        - mtls_verified: true
        - client_san_contains: "internal.service"
    action:
      type: ALLOW
      log: true
```

### LLM Setup

1. **DeepSeek (Recommended - Cheaper)**

   ```bash
   # Get API key from https://platform.deepseek.com
   export HONEYTRAP_LLM_API_KEY="sk-..."
   ```

2. **OpenAI**

   ```bash
   # Get API key from https://platform.openai.com
   export HONEYTRAP_LLM_API_KEY="sk-..."
   ```

3. Update config.toml:
   ```toml
   [llm]
   enabled = true
   provider = "deepseek"  # or "openai"
   api_key = "${HONEYTRAP_LLM_API_KEY}"
   ```

## 📖 API Documentation

### Policy Engine

The policy engine provides flexible, priority-based traffic routing decisions.

#### Creating a Policy Engine

```rust
use honeytrap_policy::{PolicyEngine, ActionType};

let engine = PolicyEngine::new(ActionType::Deception);
```

#### Loading Policies

```rust
// Load from YAML/JSON files
engine.load_policies(&vec![
    "policies/base-policies.yaml".to_string(),
    "policies/high-risk.yaml".to_string(),
]).await?;
```

#### Evaluating Traffic

```rust
use honeytrap_policy::EvaluationContext;

let context = EvaluationContext {
    src_ip: Some("192.168.1.100".to_string()),
    protocol: Some("ssh".to_string()),
    risk_score: 75,
    mtls_verified: false,
    failed_logins_count: 3,
    ..Default::default()
};

let decision = engine.evaluate(&context).await;

match decision.action {
    ActionType::Allow => println!("Allowing connection"),
    ActionType::Block => println!("Blocking connection"),
    ActionType::Deception => println!("Redirecting to honeypot"),
}
```

### Server Main Loop

The server follows this flow:

1. **Initialization**: Load config, initialize policy engine, create HoneyTrap system
2. **Listener Start**: Begin accepting connections on configured address/port
3. **Connection Handling**: For each connection:
   - Extract features (IP, protocol, behavior patterns)
   - Run AI anomaly detection
   - Evaluate policies
   - Route to appropriate handler (allow/block/honeypot)
4. **Graceful Shutdown**: Handle SIGINT (Ctrl+C) and clean up

```rust
// Example: Minimal server setup
use honeytrap_core::{Config, HoneyTrap};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let honeytrap = HoneyTrap::new(config).await?;
    
    honeytrap.run().await?;
    Ok(())
}
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests for specific package
cargo test --package honeytrap-policy

# Run doc tests
cargo test --doc

# Run with output
cargo test -- --nocapture
```

## 📚 Documentation Generation

Generate and view the full API documentation:

```bash
# Generate docs for all crates
cargo doc --no-deps --open

# Generate docs for specific packages
cargo doc --no-deps --package honeytrap-policy --package honeytrap-server --open
```

Documentation follows **Google style** with:
- Comprehensive module-level docs
- Function/method documentation with:
  - Purpose description
  - Arguments explanation
  - Return value description
  - Example code snippets
  - Error cases
  - Thread safety notes (where applicable)

## 🛠️ Development Status

**Phase 1: Core Implementation** ✅

- [x] Project structure
- [x] Core orchestration
- [x] Session management
- [x] Router implementation
- [x] AI anomaly detection (basic)
- [x] Honeypot system (basic)
- [x] QUIC transport (skeleton)
- [x] Policy engine implementation
- [x] Server binary with listener
- [x] Comprehensive documentation

**Phase 2: Advanced Features** 🚧

- [ ] Full QUIC implementation with Quinn
- [ ] RandomForest ML model
- [ ] Advanced honeypot interactions
- [ ] Metrics & monitoring
- [ ] Dashboard
- [ ] Policy hot-reload
- [ ] Database persistence

## 🔍 Troubleshooting

### Server won't start

Check that the configured port is not in use:
```bash
# Linux/macOS
lsof -i :8443

# Stop conflicting service
sudo systemctl stop <service-name>
```

### Config parsing errors

The server falls back to defaults if config.toml has errors. Check logs:
```bash
RUST_LOG=info cargo run --bin honeytrap-server 2>&1 | grep -i "config"
```

### Policy files not loading

Ensure paths in config are absolute or relative to working directory:
```yaml
# Absolute path
files = ["/etc/honeytrap/policies/base.yaml"]

# Relative path (from where you run the binary)
files = ["policies/base.yaml"]
```

## 📝 License

MIT OR Apache-2.0

## 👥 Author

Anton Feldmann

## 🤝 Contributing

Contributions welcome! Please follow these guidelines:

1. **Code Style**: Use `cargo fmt` and `cargo clippy`
2. **Documentation**: Follow Google style, include examples
3. **Testing**: Add tests for new features
4. **Commit Messages**: Use conventional commits format

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test
```

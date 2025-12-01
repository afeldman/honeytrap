# HoneyTrap

🍯 **HoneyTrap** - AI-Powered Zero Trust Network Access with Intelligent Deception

[![CI](https://github.com/yourusername/honeytrap/workflows/CI/badge.svg)](https://github.com/yourusername/honeytrap/actions)
[![Release](https://github.com/yourusername/honeytrap/workflows/Release/badge.svg)](https://github.com/yourusername/honeytrap/releases)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

## 🎯 Overview

HoneyTrap is a Rust-based security system that uses **AI-powered anomaly detection** to identify potential attackers and intelligently redirect them into **honeypots** for analysis, while allowing legitimate traffic to pass through.

## ✨ Features

- **🤖 AI Anomaly Detection**: RandomForest ML model + heuristic analysis
- **🧠 LLM Integration**: DeepSeek & OpenAI for intelligent behavior analysis
- **🎓 Reinforcement Learning**: Q-Learning for adaptive defense strategies
- **🌲 RandomForest Model**: Supervised learning for accurate anomaly detection
- **🍯 Advanced Honeypot Interactions**: Realistic SSH, HTTP, MySQL protocol emulation
- **💻 Fake Shell Environment**: Command parsing, filesystem simulation, credential capture
- **🎭 Intelligent Response Strategies**: Adaptive engagement based on attacker behavior
- **📜 Dual Scripting Engines**: Python & Rhai for flexible customization
- **🔐 Secure QUIC Transport**: Modern, encrypted networking with Quinn
- **📊 Real-time Monitoring**: Session tracking and statistics
- **🎯 Zero Trust Architecture**: Every connection is analyzed
- **🐳 Container Ready**: Docker and Kubernetes support
- **📦 Easy Deployment**: systemd, Docker Compose, K8s

## 🚀 Quick Start

### Using Make (Recommended)

```bash
# Setup development environment
make deps

# Build
make build

# Run server
make run-server

# Run with Docker
make docker

# Show all commands
make help
```

### Using Cargo

```bash
# Build
cargo build --release

# Run CLI
cargo run --bin honeytrap

# Run server
cargo run --bin honeytrap-server

# With config
HONEYTRAP_CONFIG=config/honeytrap.toml cargo run --bin honeytrap-server
```

### Docker

```bash
# Using docker-compose (Production)
docker-compose -f docker/docker-compose.yml up -d

# Using docker-compose (Development)
docker-compose -f docker/docker-compose.dev.yml up

# Using Makefile
make docker                  # Start production stack
make docker-dev             # Start development environment
make docker-logs            # View logs
make docker-stop            # Stop services

# Build specific images
make docker-build-server    # Production server
make docker-build-cli       # CLI tool
make docker-build-dev       # Development environment

# View all images
ls -la docker/
```

See `docker/README.md` for comprehensive Docker documentation.

### Kubernetes

```bash
kubectl apply -f crates/honeytrap-server/k8s-deployment.yaml
```

## 📁 Project Structure

```text
honeytrap/
├── crates/
│   ├── honeytrap-core/          # Core routing and session management
│   ├── honeytrap-ai/            # AI: RandomForest, RL Agent, LLM integration
│   ├── honeytrap-deception/     # Advanced honeypot interactions
│   ├── honeytrap-protocol/      # QUIC transport layer
│   ├── honeytrap-scripting/     # Python & Rhai scripting engines
│   ├── honeytrap-metrics/       # Prometheus metrics & monitoring
│   ├── honeytrap-cli/           # Command-line interface
│   └── honeytrap-server/        # Production server binary
├── docker/                      # Docker configurations
│   ├── Dockerfile.server        # Production server image
│   ├── Dockerfile.cli           # CLI tool image
│   ├── Dockerfile.dev           # Development environment
│   ├── docker-compose.yml       # Production stack
│   └── docker-compose.dev.yml   # Development stack
├── scripts/                     # Development and deployment scripts
├── .github/workflows/           # CI/CD pipelines
├── Makefile                     # Build automation
└── DEVELOPMENT.md              # Development guide
```

## 🔧 Configuration

### Basic Setup

Edit `config/honeytrap.toml`:

```toml
[network]
bind_addr = "0.0.0.0:8443"

[ai]
window_size = 100
anomaly_threshold = 0.7

[llm]
enabled = true
provider = "deepseek"  # or "openai"
model = "deepseek-chat"  # or "gpt-4o-mini"

[[honeypots]]
port = 22
service_type = "ssh"
interaction_level = "medium"
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

3. Update config:

   ```toml
   [llm]
   enabled = true
   provider = "deepseek"  # or "openai"
   ```

## 🤖 AI & Machine Learning

HoneyTrap integrates multiple AI/ML techniques:

### 1. RandomForest Classifier

Supervised learning for anomaly detection:

### Training the Model

```bash
# Run training example
cargo run --example train_model

# Output:
# 🌲 RandomForest Training Example
# 📊 Generating training dataset...
#    Generated 210 samples
# 🧠 Training RandomForest model...
# ✅ Training accuracy: 1.0000
```

### Using the Model

```rust
use honeytrap_ai::AnomalyDetector;

let mut detector = AnomalyDetector::new(100);

// Train with your data
let training_data = vec![
    (vec![443.0, 8443.0, 10.0, ...], false), // normal
    (vec![12345.0, 8443.0, 0.5, ...], true), // anomaly
];
detector.train(training_data).await?;

// Analyze traffic
let (is_anomaly, score) = detector.analyze(&features).await?;
```

### Features

The model uses 10 network features:

- Source port, Destination port
- Duration, Inter-packet time
- Bytes sent/received
- Packets sent/received
- Failed logins, Command frequency

### Model Persistence

```bash
# Save model
detector.save_model("model.json").await?;

# Load model
detector.load_model("model.json").await?;
```

### 2. Reinforcement Learning Agent

Q-Learning for adaptive honeypot strategies:

```bash
# Train RL agent
cargo run --package honeytrap-ai --example rl_training

# Output:
# 🤖 Reinforcement Learning Training Example
# 🎯 Training for 1000 episodes...
# ✅ Training completed!
# 📈 Final Stats:
#    Episodes trained: 1000
#    States explored: 1000
#    Avg Q-value: 1.71
```

**Actions**: Ignore, MinimalResponse, StandardEngagement, DeepEngagement, Block

**Strategy**: Epsilon-greedy policy with adaptive engagement based on attacker sophistication

### 3. LLM Integration

DeepSeek/OpenAI for behavior analysis and intelligent decision-making

## 🍯 Advanced Honeypot Interactions

Realistic protocol emulation with intelligent engagement:

### SSH Honeypot

```bash
# Run SSH interaction demo
cargo run --package honeytrap-deception --example advanced_interactions
```

**Features:**

- Fake shell with command parsing
- Realistic Linux filesystem simulation
- Credential capture and logging
- Malicious command detection (wget, curl, rm -rf, etc.)
- Command history tracking

### HTTP Honeypot

**Features:**

- Fake web application (login, admin panel)
- Attack detection (SQLi, XSS, directory traversal)
- Credential capture from login forms
- Configurable response strategies

### MySQL Honeypot

**Features:**

- MySQL protocol handshake
- Query parsing and response generation
- SQL injection detection (UNION, SLEEP, etc.)
- Database/table enumeration simulation

### Response Strategies

- **Minimal**: Quick responses, low engagement
- **Standard**: Realistic responses
- **Deep**: Maximum engagement, waste attacker time
- **Adaptive**: Adjust based on attacker behavior

## 📜 Scripting

Extend HoneyTrap with Python or Rhai scripts:

```bash
# Rhai scripting
cargo run --package honeytrap-scripting --example rhai_scripting

# Python scripting
cargo run --package honeytrap-scripting --example python_scripting
```

**Use Cases:**

- Custom anomaly detection logic
- Dynamic honeypot responses
- Attack pattern analysis
- Integration with external systems

## 📊 Metrics & Monitoring

Prometheus metrics for comprehensive monitoring:

### Quick Start

```bash
# Run metrics demo
cargo run --package honeytrap-metrics --example metrics_demo

# Access metrics
curl http://localhost:9090/metrics

# Health check
curl http://localhost:9090/health
```

### Available Metrics

**Connection Metrics:**

- `honeytrap_connections_total` - Total connections received
- `honeytrap_connections_active` - Currently active connections
- `honeytrap_connections_by_result` - Connections by classification (normal/anomaly/blocked)
- `honeytrap_connection_duration_seconds` - Connection duration histogram
- `honeytrap_bytes_total` - Bytes transferred (sent/received)

**Honeypot Metrics:**

- `honeytrap_honeypot_sessions_total` - Total sessions by type (ssh/http/mysql)
- `honeytrap_honeypot_sessions_active` - Active sessions by type
- `honeytrap_credentials_captured_total` - Captured credentials count
- `honeytrap_commands_executed_total` - Commands executed in honeypots
- `honeytrap_malicious_commands_total` - Detected malicious commands
- `honeytrap_honeypot_session_duration_seconds` - Session duration histogram

**ML Metrics:**

- `honeytrap_ml_predictions_total` - ML predictions by model and result
- `honeytrap_ml_inference_duration_seconds` - Model inference time
- `honeytrap_anomaly_scores` - Distribution of anomaly scores
- `honeytrap_rl_actions_total` - RL agent actions taken
- `honeytrap_rl_q_values` - RL agent Q-values

**System Metrics:**

- `honeytrap_uptime_seconds` - System uptime
- `honeytrap_memory_bytes` - Memory usage
- `honeytrap_cpu_usage_percent` - CPU usage
- `honeytrap_active_tasks` - Active async tasks

### Prometheus Configuration

```yaml
scrape_configs:
  - job_name: "honeytrap"
    static_configs:
      - targets: ["localhost:9090"]
    scrape_interval: 15s
```

### Grafana Dashboard

Import the provided Grafana dashboard template for visualization:

- Connection analysis
- Honeypot activity heatmaps
- ML model performance
- System resource usage

## 📖 Documentation

See `/Users/anton.feldmann/lynq/honeytrap/overview.md` for comprehensive documentation.

## 🛠️ Development Status

**Phase 1: Core Implementation** ✅

- [x] Project structure
- [x] Core orchestration
- [x] Session management
- [x] Router implementation
- [x] AI anomaly detection (basic)
- [x] Honeypot system (basic)
- [x] QUIC transport (skeleton)

**Phase 2: Advanced Features** ✅

- [x] Full QUIC implementation with Quinn
- [x] RandomForest ML model
- [x] Reinforcement Learning (Q-Learning)
- [x] Advanced honeypot interactions (SSH, HTTP, MySQL)
- [x] Fake filesystem and command parsing
- [x] Scripting engines (Python & Rhai)
- [x] Intelligent response strategies
- [x] Prometheus metrics & monitoring
- [ ] Grafana dashboards
- [ ] Web UI

**Phase 3: Production Ready** 🚧

- [ ] Grafana dashboards (JSON templates)
- [ ] Web UI for monitoring and configuration
- [ ] Alert manager integration
- [ ] Enhanced structured logging
- [ ] Performance optimization and benchmarks
- [ ] Comprehensive documentation

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details.

## 👥 Author

Anton Feldmann

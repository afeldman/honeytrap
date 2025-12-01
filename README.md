# HoneyTrap

🍯 **HoneyTrap** - AI-Powered Zero Trust Network Access with Intelligent Deception

[![CI](https://github.com/yourusername/honeytrap/workflows/CI/badge.svg)](https://github.com/yourusername/honeytrap/actions)
[![Release](https://github.com/yourusername/honeytrap/workflows/Release/badge.svg)](https://github.com/yourusername/honeytrap/releases)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

## 🎯 Overview

HoneyTrap is a Rust-based security system that uses **AI-powered anomaly detection** to identify potential attackers and intelligently redirect them into **honeypots** for analysis, while allowing legitimate traffic to pass through.

## ✨ Features

- **🤖 AI Anomaly Detection**: Machine Learning-based traffic analysis
- **🧠 LLM Integration**: DeepSeek & OpenAI for intelligent behavior analysis
- **🍯 Multi-Protocol Honeypots**: SSH, HTTP, MySQL emulation
- **🔐 Secure QUIC Transport**: Modern, encrypted networking
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
│   ├── honeytrap-ai/            # AI anomaly detection + LLM
│   ├── honeytrap-deception/     # Honeypot system
│   ├── honeytrap-protocol/      # QUIC transport layer
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

**Phase 2: Advanced Features** 🚧

- [ ] Full QUIC implementation with Quinn
- [ ] RandomForest ML model
- [ ] Advanced honeypot interactions
- [ ] Metrics & monitoring
- [ ] Dashboard

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details.

## 👥 Author

Anton Feldmann

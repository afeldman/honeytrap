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

## 🚀 Quick Start

```bash
# Build
cargo build --release

# Run
cargo run --release -- start --config config/honeytrap.toml

# Verbose logging
cargo run --release -- start --verbose
```

## 📁 Project Structure

```
honeytrap/
├── crates/
│   ├── honeytrap-core/          # Main orchestration
│   ├── honeytrap-ai/            # ML anomaly detection
│   ├── honeytrap-deception/     # Honeypot implementations
│   ├── honeytrap-protocol/      # QUIC transport
│   └── honeytrap-cli/           # CLI interface
├── config/
│   └── honeytrap.toml           # Configuration
└── Cargo.toml                   # Workspace
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

MIT OR Apache-2.0

## 👥 Author

Anton Feldmann

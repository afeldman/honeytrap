# HoneyTrap Development Guide

## Quick Start

### Setup

```bash
# Clone repository
git clone https://github.com/yourusername/honeytrap.git
cd honeytrap

# Run setup script
./scripts/setup-dev.sh

# Or manually
make deps
make build
```

## Development Workflow

### Using Make

```bash
# Show all available commands
make help

# Build
make build              # Build all binaries
make build-cli          # Build CLI only
make build-server       # Build server only

# Development
make dev                # Run with auto-reload
make run-server         # Run server
make run                # Run CLI

# Testing
make test               # Run all tests
make test-unit          # Unit tests only
make test-integration   # Integration tests only
make coverage           # Generate coverage report

# Code Quality
make fmt                # Format code
make lint               # Run clippy
make audit              # Security audit
make ci                 # Run all CI checks

# Docker
make docker             # Start with docker-compose
make docker-build       # Build image
make docker-logs        # View logs

# Kubernetes
make k8s-deploy         # Deploy to cluster
make k8s-status         # Check status
make k8s-logs           # View logs
```

### Without Make

```bash
# Build
cargo build --release --workspace

# Test
cargo test --workspace

# Format
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Run
cargo run --bin honeytrap-server
```

## Project Structure

```
honeytrap/
├── crates/
│   ├── honeytrap-core/         # Core routing and session management
│   ├── honeytrap-ai/           # AI anomaly detection
│   ├── honeytrap-deception/    # Honeypot system
│   ├── honeytrap-protocol/     # QUIC transport
│   ├── honeytrap-cli/          # CLI tool
│   └── honeytrap-server/       # Production server
├── scripts/                    # Development scripts
├── .github/workflows/          # CI/CD pipelines
├── Makefile                    # Build automation
└── Cargo.toml                  # Workspace configuration
```

## Testing

### Unit Tests

```bash
make test-unit
# or
cargo test --lib --workspace
```

### Integration Tests

```bash
make test-integration
# or
cargo test --test '*' --workspace
```

### Coverage

```bash
make coverage
# Opens: target/coverage/index.html
```

## Code Quality

### Pre-commit Checks

Git hooks are automatically set up by `setup-dev.sh`:

```bash
# Manual setup
ln -s ../../scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

### CI Pipeline

All PRs must pass:

- ✅ Formatting check (`cargo fmt`)
- ✅ Linting (`clippy`)
- ✅ Tests on Linux, macOS, Windows
- ✅ Security audit
- ✅ Build verification

## Docker Development

```bash
# Start services
make docker

# View logs
make docker-logs

# Stop services
make docker-stop

# Build custom image
make docker-build
```

## Debugging

### Enable debug logs

```bash
RUST_LOG=debug cargo run --bin honeytrap-server
```

### With specific modules

```bash
RUST_LOG=honeytrap=trace,honeytrap_core=debug cargo run
```

### Backtrace

```bash
RUST_BACKTRACE=1 cargo run
```

## Performance

### Benchmarks

```bash
make bench
```

### Binary size analysis

```bash
make bloat
```

### Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Profile
cargo flamegraph --bin honeytrap-server
```

## Release Process

### Create Release

```bash
# Update version in Cargo.toml
make version

# Create tag and push
make release
```

### Manual Release

```bash
# Tag version
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# GitHub Actions will automatically:
# - Build binaries for all platforms
# - Create Docker images
# - Publish to crates.io
# - Create GitHub release
```

## Troubleshooting

### Build fails

```bash
# Clean and rebuild
make clean
make build

# Update dependencies
make deps-update
```

### Tests fail

```bash
# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name -- --nocapture
```

### Docker issues

```bash
# Clean docker
docker system prune -a

# Rebuild
make docker-build
```

## Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/my-feature`
3. Make changes and test: `make ci`
4. Commit: `git commit -am 'Add feature'`
5. Push: `git push origin feature/my-feature`
6. Create Pull Request

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Tokio Documentation](https://tokio.rs)
- [Project Documentation](https://docs.rs/honeytrap)

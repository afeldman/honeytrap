# Docker Images Changelog

## 2024-01-XX - Initial Release

### Added

#### Production Images

- **Dockerfile.server**: Multi-stage Alpine-based production image
  - Builder stage with dependency caching
  - Runtime stage with minimal footprint (~15 MB)
  - Non-root user security
  - Tini init system
  - Health checks
  - Volume support
- **Dockerfile.alpine**: Ultra-minimal production image

  - Smallest possible size (~10 MB)
  - Static linking
  - Minimal runtime dependencies

- **Dockerfile.cli**: Command-line tool image
  - Standalone CLI binary
  - Optimized for scripting and automation

#### Development Images

- **Dockerfile.dev**: Full development environment

  - cargo-watch for hot-reload
  - All development tools (rustfmt, clippy, rust-analyzer)
  - Debugging tools (gdb, lldb)
  - Development user with UID/GID mapping

- **Dockerfile.test**: Test runner image
  - cargo-tarpaulin for coverage
  - cargo-audit for security
  - Optimized for CI/CD

#### Docker Compose

- **docker-compose.yml**: Production stack

  - honeytrap-server service
  - honeytrap-cli (tools profile)
  - Prometheus monitoring (monitoring profile)
  - Grafana dashboard (monitoring profile)
  - Network isolation
  - Volume management
  - Resource limits

- **docker-compose.dev.yml**: Development stack
  - Development server with hot-reload
  - PostgreSQL for testing
  - Redis for caching
  - Volume mounts for code
  - Cargo cache optimization

#### Configuration

- **.dockerignore**: Build optimization

  - Excludes unnecessary files
  - Reduces build context
  - Speeds up Docker builds

- **prometheus.yml**: Monitoring configuration
  - HoneyTrap metrics scraping
  - Prometheus self-monitoring
  - Configurable intervals

#### Documentation

- **README.md**: Comprehensive Docker documentation
  - Image descriptions and use cases
  - Usage examples
  - Configuration guide
  - Security best practices
  - Monitoring setup
  - Troubleshooting guide

### Build System Integration

- Updated Makefile with new Docker targets:

  - `make docker-build-server`
  - `make docker-build-cli`
  - `make docker-build-dev`
  - `make docker-build-test`
  - `make docker-build-alpine`
  - `make docker-dev`
  - `make docker-test`
  - `make docker-clean`

- Updated GitHub Actions:
  - docker.yml: Matrix build for server, cli, alpine
  - Multi-platform support (amd64, arm64)
  - Build caching per image type
  - Automated image testing

### Features

- Multi-stage builds with dependency caching
- Multi-platform support (amd64/arm64)
- Non-root user execution
- Health checks
- Resource limits
- Volume mounts
- Environment configuration
- Logging configuration
- Metrics endpoints
- Graceful shutdown

### Security

- Non-root user (honeytrap:1000)
- Read-only config mounts
- Network isolation
- Minimal attack surface
- No unnecessary packages
- Security audits

### Performance

- Dependency layer caching
- Optimized build stages
- Binary stripping
- Minimal runtime images
- Cargo incremental builds (dev)

## Migration Notes

### From Old Structure

Old Dockerfiles were in `crates/honeytrap-server/`. They have been:

1. Moved to centralized `docker/` directory
2. Split into specialized variants
3. Enhanced with security features
4. Optimized for different use cases

### Breaking Changes

- Docker file paths changed from `crates/honeytrap-server/Dockerfile` to `docker/Dockerfile.server`
- Makefile targets updated
- GitHub Actions updated
- docker-compose.yml moved to `docker/` directory

### Upgrade Path

1. Pull latest changes
2. Update local scripts referencing old Docker paths
3. Rebuild images: `make docker-build`
4. Update docker-compose: `docker-compose -f docker/docker-compose.yml up`

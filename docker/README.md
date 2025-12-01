# HoneyTrap Docker Images

Docker configurations for HoneyTrap deployment and development.

## Available Images

### Production

#### `Dockerfile.server` - Production Server

Optimized multi-stage build for production deployment.

```bash
# Build
docker build -f docker/Dockerfile.server -t honeytrap-server:latest .

# Run
docker run -d \
  -p 8080:8080 \
  -v $(pwd)/config:/app/config:ro \
  -e DEEPSEEK_API_KEY=your-key \
  honeytrap-server:latest
```

**Image Size:** ~15 MB  
**Base:** Alpine Linux  
**Features:**

- Optimized binary (stripped)
- Non-root user
- Health checks
- Volume mounts
- Tini init system

#### `Dockerfile.alpine` - Ultra-Minimal

Smallest possible image size.

```bash
docker build -f docker/Dockerfile.alpine -t honeytrap:alpine .
```

**Image Size:** ~10 MB  
**Base:** Alpine Linux  
**Use Case:** Resource-constrained environments

### Development

#### `Dockerfile.dev` - Development Environment

Full development setup with tools and hot-reload.

```bash
# Build
docker build -f docker/Dockerfile.dev -t honeytrap-dev .

# Run with volume mount
docker run -it \
  -v $(pwd):/workspace \
  -p 8080:8080 \
  honeytrap-dev
```

**Features:**

- cargo-watch for auto-reload
- rustfmt, clippy, rust-analyzer
- gdb, lldb for debugging
- All development tools

#### `Dockerfile.test` - Test Runner

CI/CD test execution environment.

```bash
docker build -f docker/Dockerfile.test -t honeytrap-test .
docker run honeytrap-test
```

**Features:**

- cargo-tarpaulin for coverage
- cargo-audit for security
- Optimized for CI/CD

### CLI

#### `Dockerfile.cli` - Command-line Tool

Standalone CLI tool.

```bash
docker build -f docker/Dockerfile.cli -t honeytrap-cli .
docker run honeytrap-cli --version
```

## Docker Compose

### Production Stack

```bash
# Start server
docker-compose up -d

# View logs
docker-compose logs -f honeytrap-server

# Stop
docker-compose down
```

**Services:**

- `honeytrap-server` - Main server
- `honeytrap-cli` - CLI (profile: tools)
- `prometheus` - Metrics (profile: monitoring)
- `grafana` - Dashboard (profile: monitoring)

### Development Stack

```bash
# Start development environment
docker-compose -f docker/docker-compose.dev.yml up

# With monitoring
docker-compose -f docker/docker-compose.yml --profile monitoring up
```

**Services:**

- `honeytrap-dev` - Dev server with hot-reload
- `postgres` - Database (optional)
- `redis` - Cache (optional)

## Usage Examples

### Quick Start

```bash
# Production
docker-compose up -d

# Development
docker-compose -f docker/docker-compose.dev.yml up
```

### Build All Images

```bash
# Server
docker build -f docker/Dockerfile.server -t honeytrap-server .

# CLI
docker build -f docker/Dockerfile.cli -t honeytrap-cli .

# Dev
docker build -f docker/Dockerfile.dev -t honeytrap-dev .

# Test
docker build -f docker/Dockerfile.test -t honeytrap-test .
```

### Using Makefile

```bash
# Build
make docker-build-server

# Start
make docker

# Logs
make docker-logs

# Stop
make docker-stop
```

## Configuration

### Environment Variables

| Variable              | Description      | Default                      |
| --------------------- | ---------------- | ---------------------------- |
| `RUST_LOG`            | Log level        | `info,honeytrap=debug`       |
| `HONEYTRAP_CONFIG`    | Config file path | `/app/config/honeytrap.toml` |
| `HONEYTRAP_JSON_LOGS` | JSON logging     | `true`                       |
| `DEEPSEEK_API_KEY`    | DeepSeek API key | -                            |
| `OPENAI_API_KEY`      | OpenAI API key   | -                            |

### Volumes

```yaml
volumes:
  - ./config:/app/config:ro # Configuration
  - honeytrap-logs:/var/log/honeytrap # Logs
  - honeytrap-data:/var/lib/honeytrap # Data
```

### Ports

| Port | Service | Description   |
| ---- | ------- | ------------- |
| 8080 | HTTP    | Main server   |
| 2222 | SSH     | SSH honeypot  |
| 8081 | HTTP    | HTTP honeypot |
| 9090 | HTTP    | Metrics       |

## Profiles

Docker Compose supports profiles for optional services:

```bash
# Start with monitoring
docker-compose --profile monitoring up

# Start with tools
docker-compose --profile tools run honeytrap-cli --help

# Start development
docker-compose --profile dev up
```

## Security

### Best Practices

1. **Non-root User:** All images run as non-root user
2. **Read-only Config:** Mount config as read-only
3. **Secrets Management:** Use Docker secrets or env files
4. **Network Isolation:** Use Docker networks
5. **Resource Limits:** Set CPU and memory limits

### Example Secure Setup

```yaml
services:
  honeytrap-server:
    security_opt:
      - no-new-privileges:true
    cap_drop:
      - ALL
    cap_add:
      - NET_BIND_SERVICE
    read_only: true
    tmpfs:
      - /tmp
```

## Monitoring

### With Prometheus & Grafana

```bash
# Start with monitoring
docker-compose --profile monitoring up -d

# Access Grafana
open http://localhost:3000
# Login: admin / admin
```

### Metrics Endpoint

```bash
curl http://localhost:9090/metrics
```

## Troubleshooting

### Container won't start

```bash
# Check logs
docker-compose logs honeytrap-server

# Check config
docker-compose config

# Rebuild
docker-compose build --no-cache
```

### Permission issues

```bash
# Fix ownership
docker-compose run --rm honeytrap-server chown -R 1000:1000 /var/log/honeytrap
```

### High memory usage

```yaml
# Add limits in docker-compose.yml
deploy:
  resources:
    limits:
      memory: 1G
```

## Development Workflow

```bash
# Start dev environment
docker-compose -f docker/docker-compose.dev.yml up

# In another terminal: run tests
docker-compose exec honeytrap-dev cargo test

# Format code
docker-compose exec honeytrap-dev cargo fmt

# Lint
docker-compose exec honeytrap-dev cargo clippy
```

## CI/CD Integration

### GitHub Actions

```yaml
- name: Build Docker image
  run: docker build -f docker/Dockerfile.test -t honeytrap-test .

- name: Run tests
  run: docker run honeytrap-test
```

### GitLab CI

```yaml
test:
  image: docker:latest
  script:
    - docker build -f docker/Dockerfile.test -t honeytrap-test .
    - docker run honeytrap-test
```

## Advanced

### Multi-stage Build Optimization

The Dockerfiles use dependency caching for faster builds:

1. Copy only Cargo.toml files
2. Create dummy source files
3. Build dependencies (cached layer)
4. Copy real source code
5. Build application

### Custom Build Args

```bash
docker build \
  --build-arg RUST_VERSION=1.75 \
  --build-arg FEATURES="--all-features" \
  -f docker/Dockerfile.server \
  -t honeytrap-server .
```

## License

MIT OR Apache-2.0

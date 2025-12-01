# HoneyTrap Server

Production-ready server binary for HoneyTrap.

## Features

- ✅ Configuration loading from TOML
- ✅ Graceful shutdown handling (SIGTERM, SIGINT, SIGQUIT)
- ✅ Structured logging (plain text or JSON)
- ✅ Environment variable configuration
- ✅ Health monitoring
- ✅ Production-ready error handling

## Usage

### Basic

```bash
cargo run --bin honeytrap-server
```

### With Custom Config

```bash
HONEYTRAP_CONFIG=/path/to/config.toml cargo run --bin honeytrap-server
```

### With JSON Logging

```bash
HONEYTRAP_JSON_LOGS=true cargo run --bin honeytrap-server
```

## Environment Variables

| Variable              | Description         | Default                |
| --------------------- | ------------------- | ---------------------- |
| `HONEYTRAP_CONFIG`    | Path to config file | `honeytrap.toml`       |
| `HONEYTRAP_JSON_LOGS` | Enable JSON logging | `false`                |
| `RUST_LOG`            | Log level filter    | `info,honeytrap=debug` |

## Docker

```bash
# Build
docker build -t honeytrap-server .

# Run
docker run -p 8080:8080 -v $(pwd)/honeytrap.toml:/app/honeytrap.toml honeytrap-server
```

## Systemd Service

```ini
[Unit]
Description=HoneyTrap Server
After=network.target

[Service]
Type=simple
User=honeytrap
Group=honeytrap
WorkingDirectory=/opt/honeytrap
Environment=HONEYTRAP_CONFIG=/etc/honeytrap/config.toml
Environment=HONEYTRAP_JSON_LOGS=true
Environment=RUST_LOG=info,honeytrap=debug
ExecStart=/opt/honeytrap/bin/honeytrap-server
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

## Health Checks

The server logs startup progress and can be monitored via:

- Log messages for initialization steps
- Process exit codes (0 = success, 1 = error)
- Signal handling for graceful shutdown

## Security

- Runs with minimal privileges
- No root access required
- Config file permissions should be 0600
- Supports TLS/QUIC for encrypted communication

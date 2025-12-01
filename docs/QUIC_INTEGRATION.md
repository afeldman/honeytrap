# QUIC Integration mit Quinn

## ✅ Implementiert

HoneyTrap nutzt jetzt **Quinn 0.11** für vollständige QUIC-Unterstützung mit TLS 1.3.

### Features

- ✅ **Quinn 0.11** - Moderne QUIC-Implementation
- ✅ **Rustls 0.23** - TLS 1.3 Support
- ✅ **Self-signed Certificates** - Automatische Zertifikatsgenerierung
- ✅ **Bi-directional Streams** - Volle Duplex-Kommunikation
- ✅ **Uni-directional Streams** - One-way Datenströme
- ✅ **Client & Server** - Beide Modi unterstützt
- ✅ **Stream Utilities** - Helper-Funktionen für Stream-Handling
- ✅ **Feature Flags** - Optional aktivierbar

## Architektur

### Komponenten

```
honeytrap-protocol/
├── src/
│   ├── quic.rs              # QUIC Transport Layer
│   ├── stream.rs            # Stream Utilities
│   └── lib.rs               # Public API
└── examples/
    ├── quic_server.rs       # Test-Server
    └── quic_client.rs       # Test-Client
```

### SecureQuicTransport

Hauptklasse für QUIC-Kommunikation:

```rust
pub struct SecureQuicTransport {
    endpoint: Endpoint,
    bind_addr: SocketAddr,
}
```

## API-Übersicht

### Server erstellen

```rust
use honeytrap_protocol::SecureQuicTransport;

let server = SecureQuicTransport::new_server("0.0.0.0:8443".parse()?).await?;

loop {
    let (connection, peer_addr) = server.accept().await?;
    // Handle connection
}
```

### Client erstellen

```rust
let client = SecureQuicTransport::new_client().await?;
let connection = client.connect("127.0.0.1:8443".parse()?, "localhost").await?;
```

### Streams verwenden

#### Bi-directional Stream

```rust
// Server
let (mut send, mut recv) = connection.accept_bi().await?;

// Client
let (mut send, mut recv) = connection.open_bi().await?;

// Daten senden
send.write_all(b"Hello QUIC!").await?;
send.finish()?;

// Daten empfangen
let data = recv.read_to_end(1024).await?;
```

#### Uni-directional Stream

```rust
// Nur senden
let mut send = connection.open_uni().await?;
send.write_all(b"One-way message").await?;

// Nur empfangen
let mut recv = connection.accept_uni().await?;
let data = recv.read_to_end(1024).await?;
```

### Stream Utilities

```rust
use honeytrap_protocol::QuicStream;

let stream = QuicStream::new(send, recv);
stream.write_all(b"data").await?;
let n = stream.read(&mut buf).await?;
```

#### Line-based Reading

```rust
use honeytrap_protocol::QuicLineReader;

let mut reader = QuicLineReader::new(recv);
let line = reader.read_line().await?;  // Reads until \n
let chunk = reader.read_exact(100).await?;  // Reads exactly 100 bytes
```

## Connection-Struktur

Die `Connection`-Struktur wurde erweitert:

```rust
pub struct Connection {
    pub peer_addr: SocketAddr,
    pub quinn_connection: Option<Arc<quinn::Connection>>,
}

impl Connection {
    pub async fn open_bi(&self) -> Result<(SendStream, RecvStream), Error>;
    pub async fn open_uni(&self) -> Result<SendStream, Error>;
    pub async fn accept_bi(&self) -> Result<(SendStream, RecvStream), Error>;
    pub async fn accept_uni(&self) -> Result<RecvStream, Error>;
    pub async fn close(&self);
}
```

## Zertifikatsverwaltung

### Self-signed Certificates

HoneyTrap generiert automatisch selbst-signierte Zertifikate:

```rust
fn generate_self_signed_cert() -> Result<(CertificateDer, PrivatePkcs8KeyDer), Error>
```

Features:

- ✅ Automatische Generierung bei Server-Start
- ✅ RSA 2048-bit Keys
- ✅ Gültig für "localhost"
- ✅ Keine manuelle Konfiguration nötig

### Certificate Verification

Für Honeypot-Zwecke akzeptiert der Client alle Zertifikate:

```rust
struct SkipServerVerification;
```

⚠️ **Warnung**: Nur für Honeypots! In Production würde man echte Verifikation nutzen.

## Transport-Konfiguration

### Server

```rust
let mut server_config = ServerConfig::with_crypto(Arc::new(
    quinn::crypto::rustls::QuicServerConfig::try_from(crypto)?
));
```

Features:

- TLS 1.3
- Self-signed Cert
- No client auth required

### Client

```rust
let client_config = quinn::ClientConfig::new(Arc::new(
    quinn::crypto::rustls::QuicClientConfig::try_from(crypto)?
));
```

Features:

- TLS 1.3
- Skip certificate verification
- Suitable for testing

## Examples

### Test Server ausführen

```bash
cargo run --example quic_server
```

Output:

```
🚀 Starting QUIC test server on 127.0.0.1:8443
✅ Server ready, waiting for connections...
```

### Test Client ausführen

```bash
cargo run --example quic_client
```

Output:

```
🔌 Connecting to QUIC server at 127.0.0.1:8443
✅ Connected! Peer: 127.0.0.1:8443
📤 Sent message
📥 Received: Echo: Hello from QUIC client!
👋 Connection closed
```

### Makefile

```bash
# Build QUIC examples
make build

# Run tests
make test

# Run server
make run-server
```

## Integration in HoneyTrap

### Core Integration

```rust
// In honeytrap-core/src/lib.rs
pub struct HoneyTrap {
    pub transport: Arc<SecureQuicTransport>,
    // ...
}

impl HoneyTrap {
    pub async fn run(&self) -> Result<(), Error> {
        loop {
            let (connection, peer_addr) = self.transport.accept().await?;
            // Route to honeypots
            self.router.handle_connection(connection).await?;
        }
    }
}
```

### Honeypot Handler

```rust
#[async_trait]
impl Honeypot for SshHoneypot {
    async fn handle(
        &self,
        connection: Connection,
        session: Session,
    ) -> Result<(), Error> {
        // Open QUIC stream
        let (mut send, mut recv) = connection.open_bi().await?;

        // SSH protocol over QUIC
        // ...
    }
}
```

## Performance

### Benchmarks

QUIC bietet folgende Vorteile:

- **0-RTT Handshake**: Wiederverbindungen ohne Latenz
- **Multiplexing**: Mehrere Streams ohne Head-of-line Blocking
- **Connection Migration**: Nahtlose IP-Wechsel
- **Built-in TLS**: Verschlüsselt by default

### Messungen

```bash
# Benchmark ausführen
cargo bench
```

## Feature Flags

QUIC-Support ist über Feature Flags steuerbar:

```toml
[dependencies]
honeytrap-protocol = { path = "../honeytrap-protocol", default-features = false }

# Mit QUIC (default)
honeytrap-protocol = { path = "../honeytrap-protocol" }

# Ohne QUIC
honeytrap-protocol = { path = "../honeytrap-protocol", default-features = false }
```

## Error Handling

### Common Errors

```rust
// Connection errors
Err("Endpoint closed")           // Endpoint wurde geschlossen
Err("No QUIC connection")        // Connection hat keinen QUIC-Support

// Stream errors
Err(quinn::WriteError)           // Fehler beim Schreiben
Err(quinn::ReadError)            // Fehler beim Lesen
```

### Debugging

```bash
# Enable tracing
RUST_LOG=debug cargo run --example quic_server

# Detailed QUIC logs
RUST_LOG=quinn=trace cargo run
```

## Troubleshooting

### "Connection reset"

Ursache: Client oder Server hat Connection geschlossen

Lösung:

```rust
// Graceful shutdown
connection.close().await;
endpoint.close(0u32.into(), b"shutdown");
```

### "Certificate verification failed"

Ursache: Client versucht Zertifikat zu verifizieren

Lösung: `SkipServerVerification` verwenden (nur für Tests!)

### "Address already in use"

Ursache: Port bereits belegt

Lösung:

```bash
# Find process
lsof -i :8443

# Kill process
kill -9 <PID>
```

## Security

### Threat Model

QUIC in HoneyTrap ist für folgende Szenarien konzipiert:

✅ **Geschützt gegen:**

- Eavesdropping (TLS 1.3)
- Man-in-the-Middle (Certificate Pinning möglich)
- Replay Attacks (QUIC sequence numbers)

⚠️ **Nicht geschützt gegen:**

- DoS Attacks (rate limiting notwendig)
- Certificate Spoofing (SkipServerVerification!)

### Best Practices

1. **Production**: Echte Zertifikate verwenden (Let's Encrypt)
2. **Monitoring**: Connection metrics tracken
3. **Rate Limiting**: Connections pro IP begrenzen
4. **Logging**: Alle Verbindungen loggen

## Next Steps

### Geplante Erweiterungen

- [ ] Connection pooling
- [ ] Automatic reconnection
- [ ] Stream prioritization
- [ ] Congestion control tuning
- [ ] 0-RTT support
- [ ] HTTP/3 support
- [ ] Custom ALPN protocols

## Resources

- [Quinn Documentation](https://docs.rs/quinn/)
- [QUIC RFC 9000](https://www.rfc-editor.org/rfc/rfc9000.html)
- [Rustls Documentation](https://docs.rs/rustls/)
- [QUIC Working Group](https://quicwg.org/)

## Version History

### 0.1.0 (Current)

- ✅ Initial QUIC implementation
- ✅ Quinn 0.11 integration
- ✅ Self-signed certificates
- ✅ Bi/Uni streams
- ✅ Stream utilities
- ✅ Examples

### Planned (0.2.0)

- HTTP/3 support
- Connection migration
- 0-RTT handshakes
- Performance optimizations

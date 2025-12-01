use honeytrap_deception::Connection;
use quinn::{Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use std::net::SocketAddr;
use std::sync::Arc;

/// Secure QUIC Transport mit Quinn
pub struct SecureQuicTransport {
    endpoint: Endpoint,
    bind_addr: SocketAddr,
}

impl SecureQuicTransport {
    /// Neuer QUIC Server mit selbst-signiertem Zertifikat
    pub async fn new_server(bind_addr: SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        tracing::info!("🔐 Initializing QUIC server on {}", bind_addr);

        // Selbst-signiertes Zertifikat generieren
        let (cert, key) = generate_self_signed_cert()?;

        // Server-Konfiguration
        let server_config = configure_server(cert, key)?;

        // QUIC Endpoint erstellen
        let endpoint = Endpoint::server(server_config, bind_addr)?;

        tracing::info!("✅ QUIC endpoint ready on {}", bind_addr);

        Ok(Self {
            endpoint,
            bind_addr,
        })
    }

    /// Connection akzeptieren
    pub async fn accept(&self) -> Result<(Connection, SocketAddr), Box<dyn std::error::Error>> {
        // Warte auf eingehende QUIC-Verbindung
        let incoming = self.endpoint.accept().await.ok_or("Endpoint closed")?;

        let peer_addr = incoming.remote_address();
        tracing::debug!("📥 Accepting QUIC connection from {}", peer_addr);

        // Connection etablieren
        let quinn_connection = incoming.await?;

        tracing::info!("✅ QUIC connection established with {}", peer_addr);

        // In unsere Connection-Struktur konvertieren
        let connection = Connection {
            peer_addr,
            quinn_connection: Some(Arc::new(quinn_connection)),
        };

        Ok((connection, peer_addr))
    }

    /// Client-Endpoint erstellen (für ausgehende Verbindungen)
    pub async fn new_client() -> Result<Self, Box<dyn std::error::Error>> {
        tracing::info!("🔐 Initializing QUIC client");

        let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;

        // Client-Konfiguration mit unsicherer Zertifikatsprüfung (für Honeypot-Zwecke)
        let crypto = rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
            .with_no_client_auth();

        let client_config = quinn::ClientConfig::new(Arc::new(
            quinn::crypto::rustls::QuicClientConfig::try_from(crypto)?
        ));

        endpoint.set_default_client_config(client_config);

        Ok(Self {
            endpoint,
            bind_addr: "0.0.0.0:0".parse()?,
        })
    }

    /// Mit Remote-Server verbinden
    pub async fn connect(
        &self,
        addr: SocketAddr,
        server_name: &str,
    ) -> Result<Connection, Box<dyn std::error::Error>> {
        tracing::debug!("🔌 Connecting to {}@{}", server_name, addr);

        let quinn_connection = self.endpoint.connect(addr, server_name)?.await?;

        tracing::info!("✅ Connected to {}", addr);

        let connection = Connection {
            peer_addr: addr,
            quinn_connection: Some(Arc::new(quinn_connection)),
        };

        Ok(connection)
    }

    /// Lokale Adresse abrufen
    pub fn local_addr(&self) -> SocketAddr {
        self.bind_addr
    }

    /// Endpoint graceful shutdown
    pub async fn close(&self) {
        tracing::info!("🛑 Closing QUIC endpoint");
        self.endpoint.close(0u32.into(), b"shutdown");
        self.endpoint.wait_idle().await;
    }
}

/// Selbst-signiertes Zertifikat generieren
fn generate_self_signed_cert() -> Result<(CertificateDer<'static>, PrivatePkcs8KeyDer<'static>), Box<dyn std::error::Error>> {
    tracing::debug!("🔑 Generating self-signed certificate");

    let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])?;
    let key = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());
    let cert_der = cert.cert.into();

    tracing::debug!("✅ Certificate generated");

    Ok((cert_der, key))
}

/// Server-Konfiguration mit TLS
fn configure_server(
    cert: CertificateDer<'static>,
    key: PrivatePkcs8KeyDer<'static>,
) -> Result<ServerConfig, Box<dyn std::error::Error>> {
    tracing::debug!("⚙️  Configuring QUIC server");

    let crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key.into())?;

    let server_config = ServerConfig::with_crypto(Arc::new(
        quinn::crypto::rustls::QuicServerConfig::try_from(crypto)?
    ));

    tracing::debug!("✅ Server configuration ready");

    Ok(server_config)
}

/// Custom Certificate Verifier der alle Zertifikate akzeptiert
/// Nur für Honeypot-Zwecke! In Production würde man echte Verifikation nutzen.
#[derive(Debug)]
struct SkipServerVerification;

impl rustls::client::danger::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ED25519,
        ]
    }
}

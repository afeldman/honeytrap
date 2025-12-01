/// QUIC Test Client
///
/// Simple client to test QUIC connectivity

use honeytrap_protocol::SecureQuicTransport;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple logging
    tracing_subscriber::fmt::init();

    let server_addr: SocketAddr = "127.0.0.1:8443".parse()?;

    println!("🔌 Connecting to QUIC server at {}", server_addr);

    let client = SecureQuicTransport::new_client().await?;
    let connection = client.connect(server_addr, "localhost").await?;

    println!("✅ Connected! Peer: {}", connection.peer_addr);

    // Öffne bi-directional stream
    let (mut send, mut recv) = connection.open_bi().await?;

    // Sende Test-Nachricht
    send.write_all(b"Hello from QUIC client!\n").await?;
    send.finish()?;

    println!("📤 Sent message");

    // Empfange Antwort
    let response = recv.read_to_end(1024).await?;
    let response_str = String::from_utf8_lossy(&response);

    println!("📥 Received: {}", response_str);

    connection.close().await;

    println!("👋 Connection closed");

    Ok(())
}

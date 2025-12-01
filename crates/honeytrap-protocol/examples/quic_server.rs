/// QUIC Test Server
///
/// Simple server to test QUIC connectivity

use honeytrap_protocol::SecureQuicTransport;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple logging
    tracing_subscriber::fmt::init();

    let bind_addr: SocketAddr = "127.0.0.1:8443".parse()?;

    println!("🚀 Starting QUIC test server on {}", bind_addr);

    let server = SecureQuicTransport::new_server(bind_addr).await?;

    println!("✅ Server ready, waiting for connections...");

    loop {
        let (connection, peer_addr) = server.accept().await?;

        println!("📥 New connection from {}", peer_addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(connection).await {
                eprintln!("❌ Connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(
    connection: honeytrap_protocol::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Handling connection from {}", connection.peer_addr);

    // Akzeptiere bi-directional stream
    let (mut send, mut recv) = connection.accept_bi().await?;

    // Lese eingehende Nachricht
    let message = recv.read_to_end(1024).await?;
    let message_str = String::from_utf8_lossy(&message);

    println!("📥 Received: {}", message_str);

    // Sende Echo-Antwort
    let response = format!("Echo: {}", message_str);
    send.write_all(response.as_bytes()).await?;
    send.finish()?;

    println!("📤 Sent echo response");

    Ok(())
}

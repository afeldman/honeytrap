pub mod quic;
pub mod stream;

// Connection wird von honeytrap-deception bereitgestellt
pub use honeytrap_deception::Connection;
pub use quic::SecureQuicTransport;

#[cfg(feature = "quic")]
pub use stream::{QuicLineReader, QuicStream};

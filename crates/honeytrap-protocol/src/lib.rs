pub mod quic;

// Connection wird von honeytrap-deception bereitgestellt
pub use honeytrap_deception::Connection;
pub use quic::SecureQuicTransport;

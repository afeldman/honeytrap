//! Metrics Exporter
//!
//! HTTP endpoint for Prometheus scraping

use prometheus::{Encoder, TextEncoder};
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Metrics HTTP exporter
pub struct MetricsExporter {
    addr: SocketAddr,
}

impl MetricsExporter {
    /// Create new exporter
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    /// Start metrics HTTP server
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(self.addr).await?;
        tracing::info!("📊 Metrics server listening on http://{}/metrics", self.addr);

        loop {
            let (socket, addr) = listener.accept().await?;
            tracing::debug!("📊 Metrics request from {}", addr);

            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                if let Ok(n) = socket.try_read(&mut buffer) {
                    let request = String::from_utf8_lossy(&buffer[..n]);
                    
                    if request.contains("GET /metrics") {
                        if let Ok(response) = Self::generate_metrics_response() {
                            let _ = socket.try_write(response.as_bytes());
                        }
                    } else if request.contains("GET /") || request.contains("GET /health") {
                        let response = Self::health_response();
                        let _ = socket.try_write(response.as_bytes());
                    } else {
                        let response = Self::not_found_response();
                        let _ = socket.try_write(response.as_bytes());
                    }
                }
            });
        }
    }

    /// Generate Prometheus metrics response
    fn generate_metrics_response() -> Result<String, Box<dyn std::error::Error>> {
        let encoder = TextEncoder::new();
        let metric_families = crate::METRICS.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;

        let body = String::from_utf8(buffer)?;
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/plain; version=0.0.4\r\n\
             Content-Length: {}\r\n\
             \r\n\
             {}",
            body.len(),
            body
        );

        Ok(response)
    }

    /// Health check response
    fn health_response() -> String {
        "HTTP/1.1 200 OK\r\n\
         Content-Type: application/json\r\n\
         Content-Length: 15\r\n\
         \r\n\
         {\"status\":\"ok\"}"
            .to_string()
    }

    /// 404 response
    fn not_found_response() -> String {
        "HTTP/1.1 404 Not Found\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: 9\r\n\
         \r\n\
         Not Found"
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exporter_creation() {
        let addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();
        let exporter = MetricsExporter::new(addr);
        assert_eq!(exporter.addr, addr);
    }

    #[test]
    fn test_health_response() {
        let response = MetricsExporter::health_response();
        assert!(response.contains("200 OK"));
        assert!(response.contains("status"));
    }

    #[test]
    fn test_metrics_response() {
        let response = MetricsExporter::generate_metrics_response();
        assert!(response.is_ok());
    }
}

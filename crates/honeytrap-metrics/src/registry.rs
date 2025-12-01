//! Metrics Registry
//!
//! Centralized registry management

use prometheus::Registry;

/// Metrics registry manager
pub struct MetricsRegistry {
    registry: Registry,
}

impl MetricsRegistry {
    /// Create new registry
    pub fn new() -> Self {
        Self {
            registry: Registry::new(),
        }
    }

    /// Get registry reference
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Export metrics as text
    pub fn export_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = MetricsRegistry::new();
        assert!(registry.export_text().is_ok());
    }
}

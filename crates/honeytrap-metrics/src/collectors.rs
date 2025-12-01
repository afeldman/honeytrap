//! Prometheus Metrics Collectors
//!
//! Defines all metrics for HoneyTrap monitoring

use lazy_static::lazy_static;
use prometheus::{
    CounterVec, Gauge, GaugeVec, Histogram, HistogramOpts, HistogramVec, IntCounter,
    IntCounterVec, IntGauge, IntGaugeVec, Opts, Registry,
};

lazy_static! {
    /// Global metrics instance
    pub static ref METRICS: Metrics = Metrics::new();
}

/// All HoneyTrap metrics
pub struct Metrics {
    pub connections: ConnectionMetrics,
    pub honeypots: HoneypotMetrics,
    pub ml: MlMetrics,
    pub system: SystemMetrics,
    pub registry: Registry,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let connections = ConnectionMetrics::new(&registry);
        let honeypots = HoneypotMetrics::new(&registry);
        let ml = MlMetrics::new(&registry);
        let system = SystemMetrics::new(&registry);

        Self {
            connections,
            honeypots,
            ml,
            system,
            registry,
        }
    }

    /// Get Prometheus registry
    pub fn registry(&self) -> &Registry {
        &self.registry
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Connection-related metrics
pub struct ConnectionMetrics {
    /// Total connections received
    pub total: IntCounter,
    
    /// Active connections
    pub active: IntGauge,
    
    /// Connections by result (normal/anomaly/blocked)
    pub by_result: IntCounterVec,
    
    /// Connection duration histogram
    pub duration: HistogramVec,
    
    /// Bytes transferred
    pub bytes_total: CounterVec,
}

impl ConnectionMetrics {
    fn new(registry: &Registry) -> Self {
        let total = IntCounter::with_opts(Opts::new(
            "honeytrap_connections_total",
            "Total number of connections received",
        ))
        .unwrap();
        registry.register(Box::new(total.clone())).unwrap();

        let active = IntGauge::with_opts(Opts::new(
            "honeytrap_connections_active",
            "Number of currently active connections",
        ))
        .unwrap();
        registry.register(Box::new(active.clone())).unwrap();

        let by_result = IntCounterVec::new(
            Opts::new(
                "honeytrap_connections_by_result",
                "Connections by classification result",
            ),
            &["result"],
        )
        .unwrap();
        registry.register(Box::new(by_result.clone())).unwrap();

        let duration = HistogramVec::new(
            HistogramOpts::new(
                "honeytrap_connection_duration_seconds",
                "Connection duration in seconds",
            )
            .buckets(vec![0.1, 0.5, 1.0, 5.0, 10.0, 30.0, 60.0, 300.0]),
            &["result"],
        )
        .unwrap();
        registry.register(Box::new(duration.clone())).unwrap();

        let bytes_total = CounterVec::new(
            Opts::new("honeytrap_bytes_total", "Total bytes transferred"),
            &["direction"],
        )
        .unwrap();
        registry.register(Box::new(bytes_total.clone())).unwrap();

        Self {
            total,
            active,
            by_result,
            duration,
            bytes_total,
        }
    }
}

/// Honeypot-related metrics
pub struct HoneypotMetrics {
    /// Sessions by honeypot type
    pub sessions_by_type: IntCounterVec,
    
    /// Active honeypot sessions
    pub active_sessions: IntGaugeVec,
    
    /// Captured credentials
    pub credentials_captured: IntCounterVec,
    
    /// Commands executed
    pub commands_executed: IntCounterVec,
    
    /// Malicious commands detected
    pub malicious_commands: IntCounterVec,
    
    /// Session duration
    pub session_duration: HistogramVec,
}

impl HoneypotMetrics {
    fn new(registry: &Registry) -> Self {
        let sessions_by_type = IntCounterVec::new(
            Opts::new(
                "honeytrap_honeypot_sessions_total",
                "Total honeypot sessions by type",
            ),
            &["type"],
        )
        .unwrap();
        registry.register(Box::new(sessions_by_type.clone())).unwrap();

        let active_sessions = IntGaugeVec::new(
            Opts::new(
                "honeytrap_honeypot_sessions_active",
                "Active honeypot sessions by type",
            ),
            &["type"],
        )
        .unwrap();
        registry.register(Box::new(active_sessions.clone())).unwrap();

        let credentials_captured = IntCounterVec::new(
            Opts::new(
                "honeytrap_credentials_captured_total",
                "Total credentials captured by honeypot type",
            ),
            &["type"],
        )
        .unwrap();
        registry
            .register(Box::new(credentials_captured.clone()))
            .unwrap();

        let commands_executed = IntCounterVec::new(
            Opts::new(
                "honeytrap_commands_executed_total",
                "Total commands executed in honeypots",
            ),
            &["type"],
        )
        .unwrap();
        registry
            .register(Box::new(commands_executed.clone()))
            .unwrap();

        let malicious_commands = IntCounterVec::new(
            Opts::new(
                "honeytrap_malicious_commands_total",
                "Total malicious commands detected",
            ),
            &["type", "pattern"],
        )
        .unwrap();
        registry
            .register(Box::new(malicious_commands.clone()))
            .unwrap();

        let session_duration = HistogramVec::new(
            HistogramOpts::new(
                "honeytrap_honeypot_session_duration_seconds",
                "Honeypot session duration in seconds",
            )
            .buckets(vec![1.0, 10.0, 30.0, 60.0, 300.0, 600.0, 1800.0]),
            &["type"],
        )
        .unwrap();
        registry.register(Box::new(session_duration.clone())).unwrap();

        Self {
            sessions_by_type,
            active_sessions,
            credentials_captured,
            commands_executed,
            malicious_commands,
            session_duration,
        }
    }
}

/// Machine Learning metrics
pub struct MlMetrics {
    /// Anomaly detection predictions
    pub predictions: IntCounterVec,
    
    /// ML model inference time
    pub inference_duration: HistogramVec,
    
    /// Anomaly scores
    pub anomaly_scores: Histogram,
    
    /// RL agent actions
    pub rl_actions: IntCounterVec,
    
    /// RL agent Q-values
    pub rl_q_values: GaugeVec,
}

impl MlMetrics {
    fn new(registry: &Registry) -> Self {
        let predictions = IntCounterVec::new(
            Opts::new(
                "honeytrap_ml_predictions_total",
                "Total ML predictions by model and result",
            ),
            &["model", "result"],
        )
        .unwrap();
        registry.register(Box::new(predictions.clone())).unwrap();

        let inference_duration = HistogramVec::new(
            HistogramOpts::new(
                "honeytrap_ml_inference_duration_seconds",
                "ML model inference duration",
            )
            .buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0]),
            &["model"],
        )
        .unwrap();
        registry
            .register(Box::new(inference_duration.clone()))
            .unwrap();

        let anomaly_scores = Histogram::with_opts(
            HistogramOpts::new("honeytrap_anomaly_scores", "Distribution of anomaly scores")
                .buckets(vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]),
        )
        .unwrap();
        registry.register(Box::new(anomaly_scores.clone())).unwrap();

        let rl_actions = IntCounterVec::new(
            Opts::new(
                "honeytrap_rl_actions_total",
                "Total RL agent actions taken",
            ),
            &["action"],
        )
        .unwrap();
        registry.register(Box::new(rl_actions.clone())).unwrap();

        let rl_q_values = GaugeVec::new(
            Opts::new("honeytrap_rl_q_values", "RL agent Q-values by state-action"),
            &["state", "action"],
        )
        .unwrap();
        registry.register(Box::new(rl_q_values.clone())).unwrap();

        Self {
            predictions,
            inference_duration,
            anomaly_scores,
            rl_actions,
            rl_q_values,
        }
    }
}

/// System-level metrics
pub struct SystemMetrics {
    /// System uptime
    pub uptime_seconds: IntCounter,
    
    /// Memory usage
    pub memory_bytes: IntGauge,
    
    /// CPU usage percentage
    pub cpu_usage: Gauge,
    
    /// Active goroutines/tasks
    pub active_tasks: IntGauge,
}

impl SystemMetrics {
    fn new(registry: &Registry) -> Self {
        let uptime_seconds = IntCounter::with_opts(Opts::new(
            "honeytrap_uptime_seconds",
            "System uptime in seconds",
        ))
        .unwrap();
        registry.register(Box::new(uptime_seconds.clone())).unwrap();

        let memory_bytes = IntGauge::with_opts(Opts::new(
            "honeytrap_memory_bytes",
            "Memory usage in bytes",
        ))
        .unwrap();
        registry.register(Box::new(memory_bytes.clone())).unwrap();

        let cpu_usage = Gauge::with_opts(Opts::new(
            "honeytrap_cpu_usage_percent",
            "CPU usage percentage",
        ))
        .unwrap();
        registry.register(Box::new(cpu_usage.clone())).unwrap();

        let active_tasks = IntGauge::with_opts(Opts::new(
            "honeytrap_active_tasks",
            "Number of active async tasks",
        ))
        .unwrap();
        registry.register(Box::new(active_tasks.clone())).unwrap();

        Self {
            uptime_seconds,
            memory_bytes,
            cpu_usage,
            active_tasks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = Metrics::new();
        assert_eq!(metrics.connections.total.get(), 0);
        assert_eq!(metrics.connections.active.get(), 0);
    }

    #[test]
    fn test_connection_metrics() {
        let metrics = Metrics::new();
        
        metrics.connections.total.inc();
        assert_eq!(metrics.connections.total.get(), 1);
        
        metrics.connections.active.inc();
        assert_eq!(metrics.connections.active.get(), 1);
        
        metrics.connections.active.dec();
        assert_eq!(metrics.connections.active.get(), 0);
    }

    #[test]
    fn test_honeypot_metrics() {
        let metrics = Metrics::new();
        
        metrics.honeypots.sessions_by_type.with_label_values(&["ssh"]).inc();
        metrics.honeypots.sessions_by_type.with_label_values(&["http"]).inc();
        
        assert_eq!(
            metrics.honeypots.sessions_by_type.with_label_values(&["ssh"]).get(),
            1
        );
    }

    #[test]
    fn test_ml_metrics() {
        let metrics = Metrics::new();
        
        metrics.ml.predictions.with_label_values(&["randomforest", "anomaly"]).inc();
        metrics.ml.anomaly_scores.observe(0.85);
        
        assert_eq!(
            metrics.ml.predictions.with_label_values(&["randomforest", "anomaly"]).get(),
            1
        );
    }
}

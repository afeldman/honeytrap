//! Metrics Example
//!
//! Demonstrates Prometheus metrics collection and export

use honeytrap_metrics::{MetricsExporter, METRICS};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("📊 HoneyTrap Metrics Example\n");

    // Start metrics HTTP server
    let metrics_addr: SocketAddr = "127.0.0.1:9090".parse()?;
    let exporter = MetricsExporter::new(metrics_addr);
    
    println!("🚀 Starting metrics server on http://{}/metrics\n", metrics_addr);
    
    tokio::spawn(async move {
        if let Err(e) = exporter.start().await {
            eprintln!("Metrics server error: {}", e);
        }
    });

    // Wait for server to start
    sleep(Duration::from_millis(500)).await;

    println!("📈 Simulating HoneyTrap activity...\n");

    // Simulate connections
    println!("🔌 Simulating connections:");
    for i in 0..10 {
        METRICS.connections.total.inc();
        METRICS.connections.active.inc();
        
        let result = if i % 3 == 0 { "anomaly" } else { "normal" };
        METRICS.connections.by_result.with_label_values(&[result]).inc();
        
        let duration = (i as f64 + 1.0) * 0.5;
        METRICS.connections.duration.with_label_values(&[result]).observe(duration);
        
        METRICS.connections.bytes_total.with_label_values(&["sent"]).inc_by((i * 100) as f64);
        METRICS.connections.bytes_total.with_label_values(&["received"]).inc_by((i * 50) as f64);
        
        METRICS.connections.active.dec();
        
        println!("   Connection {}: {} ({:.1}s)", i + 1, result, duration);
        sleep(Duration::from_millis(200)).await;
    }

    // Simulate honeypot sessions
    println!("\n🍯 Simulating honeypot sessions:");
    let honeypot_types = vec!["ssh", "http", "mysql"];
    for (i, hp_type) in honeypot_types.iter().enumerate() {
        METRICS.honeypots.sessions_by_type.with_label_values(&[hp_type]).inc();
        METRICS.honeypots.active_sessions.with_label_values(&[hp_type]).inc();
        
        // Simulate commands
        for _ in 0..(i + 2) {
            METRICS.honeypots.commands_executed.with_label_values(&[hp_type]).inc();
        }
        
        // Simulate credential capture
        if i % 2 == 0 {
            METRICS.honeypots.credentials_captured.with_label_values(&[hp_type]).inc();
            println!("   {}: 🔑 Credentials captured", hp_type.to_uppercase());
        }
        
        // Simulate malicious command
        if i == 0 {
            METRICS.honeypots.malicious_commands.with_label_values(&[hp_type, "wget"]).inc();
            println!("   {}: 🚨 Malicious command detected (wget)", hp_type.to_uppercase());
        }
        
        let session_duration = (i + 1) as f64 * 30.0;
        METRICS.honeypots.session_duration.with_label_values(&[hp_type]).observe(session_duration);
        
        METRICS.honeypots.active_sessions.with_label_values(&[hp_type]).dec();
        
        println!("   {}: Session completed ({:.0}s)", hp_type.to_uppercase(), session_duration);
        sleep(Duration::from_millis(200)).await;
    }

    // Simulate ML predictions
    println!("\n🤖 Simulating ML predictions:");
    let models = vec!["randomforest", "rl_agent"];
    for model in &models {
        for _ in 0..5 {
            let result = if rand::random::<f64>() > 0.7 { "anomaly" } else { "normal" };
            METRICS.ml.predictions.with_label_values(&[model, result]).inc();
            
            let inference_time = rand::random::<f64>() * 0.05;
            METRICS.ml.inference_duration.with_label_values(&[model]).observe(inference_time);
        }
        
        let anomaly_score = rand::random::<f64>();
        METRICS.ml.anomaly_scores.observe(anomaly_score);
        
        println!("   {}: 5 predictions (avg score: {:.2})", model, anomaly_score);
    }

    // RL agent actions
    println!("\n🎯 Simulating RL agent actions:");
    let actions = vec!["ignore", "minimal", "standard", "deep", "block"];
    for action in &actions {
        let count = (rand::random::<u64>() % 5) + 1;
        for _ in 0..count {
            METRICS.ml.rl_actions.with_label_values(&[action]).inc();
        }
        println!("   {}: {} times", action, count);
    }

    // System metrics
    println!("\n💻 System metrics:");
    METRICS.system.uptime_seconds.inc_by(300);
    METRICS.system.memory_bytes.set(128 * 1024 * 1024); // 128 MB
    METRICS.system.cpu_usage.set(45.5);
    METRICS.system.active_tasks.set(12);
    
    println!("   Uptime: {}s", METRICS.system.uptime_seconds.get());
    println!("   Memory: {} MB", METRICS.system.memory_bytes.get() / (1024 * 1024));
    println!("   CPU: {:.1}%", METRICS.system.cpu_usage.get());
    println!("   Active tasks: {}", METRICS.system.active_tasks.get());

    println!("\n📊 Metrics Summary:");
    println!("   Total connections: {}", METRICS.connections.total.get());
    
    let total_honeypot_sessions: u64 = honeypot_types.iter()
        .map(|t| METRICS.honeypots.sessions_by_type.with_label_values(&[t]).get())
        .sum();
    println!("   Honeypot sessions: {}", total_honeypot_sessions);
    
    let mut total_ml_predictions = 0u64;
    for model in &models {
        for result in &["anomaly", "normal"] {
            total_ml_predictions += METRICS.ml.predictions.with_label_values(&[model, result]).get();
        }
    }
    println!("   ML predictions: {}", total_ml_predictions);

    println!("\n✅ Metrics available at: http://{}/metrics", metrics_addr);
    println!("   Health check at: http://{}/health", metrics_addr);
    println!("\n💡 Try: curl http://{}/metrics", metrics_addr);
    println!("   Or open in browser: http://{}/metrics\n", metrics_addr);

    println!("⏳ Server will run for 30 seconds...");
    sleep(Duration::from_secs(30)).await;

    println!("👋 Shutting down...");

    Ok(())
}

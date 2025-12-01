/// RandomForest Training Example
///
/// Zeigt wie man den AnomalyDetector mit RandomForest trainiert

use honeytrap_ai::AnomalyDetector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging
    tracing_subscriber::fmt::init();

    println!("🌲 RandomForest Training Example\n");

    // Detektor erstellen
    let mut detector = AnomalyDetector::new(100);

    println!("📊 Generating training dataset...");

    // Training-Daten generieren
    let training_data = generate_training_data();

    println!("   Generated {} samples", training_data.len());
    println!("   Features per sample: {}\n", training_data[0].0.len());

    // Model trainieren
    println!("🧠 Training RandomForest model...");
    let accuracy = detector.train(training_data).await?;

    println!("✅ Training completed!");
    println!("   Accuracy: {:.4}\n", accuracy);

    // Test-Vorhersagen
    println!("🔮 Testing predictions:\n");

    // Normal Traffic
    let normal_features = vec![
        443.0,   // source_port
        8443.0,  // dest_port
        10.0,    // duration
        0.1,     // inter_packet_time
        1000.0,  // bytes_sent
        2000.0,  // bytes_received
        10.0,    // packets_sent
        15.0,    // packets_received
        0.0,     // failed_logins
        1.0,     // command_freq
    ];

    let (is_anomaly, score) = detector.analyze(&normal_features).await?;
    println!("Normal traffic:");
    println!("   Anomaly: {} | Score: {:.4}\n", is_anomaly, score);

    // Suspicious Traffic
    let suspicious_features = vec![
        12345.0, // unusual source_port
        8443.0,
        0.5,      // sehr kurze duration
        0.001,    // sehr schnell
        100000.0, // viele bytes
        50.0,     // wenig bytes received
        1000.0,   // viele packets
        5.0,
        10.0,  // viele failed_logins
        100.0, // hohe command_freq
    ];

    let (is_anomaly, score) = detector.analyze(&suspicious_features).await?;
    println!("Suspicious traffic:");
    println!("   Anomaly: {} | Score: {:.4}\n", is_anomaly, score);

    // Attack Pattern
    let attack_features = vec![
        54321.0, // random port
        8443.0,
        0.1,
        0.0001,    // sehr schnell
        1000000.0, // sehr viele bytes
        0.0,       // keine response
        10000.0,   // flood
        0.0,
        50.0,  // viele failed_logins
        500.0, // sehr hohe command_freq
    ];

    let (is_anomaly, score) = detector.analyze(&attack_features).await?;
    println!("Attack pattern:");
    println!("   Anomaly: {} | Score: {:.4}\n", is_anomaly, score);

    // Model speichern
    println!("💾 Saving model...");
    detector.save_model("model.json").await?;
    println!("✅ Model saved to model.json\n");

    // Statistiken
    println!("📈 Statistics:");
    println!("   Anomalies detected: {}", detector.anomalies_detected());
    println!("   Model accuracy: {:.4}", detector.model_accuracy());
    println!("   ML trained: {}", detector.is_ml_trained());
    println!("   Feature count: {}", detector.feature_count());

    Ok(())
}

/// Training-Daten generieren
/// Mix aus normalen und anomalen Samples
fn generate_training_data() -> Vec<(Vec<f64>, bool)> {
    let mut data = Vec::new();

    // Normal Traffic (Label: false = 0)
    for i in 0..100 {
        let features = vec![
            (1000 + i * 10) as f64,          // source_port
            8443.0,                           // dest_port
            10.0 + (i as f64 * 0.5),          // duration
            0.1,                              // inter_packet_time
            1000.0 + (i as f64 * 100.0),      // bytes_sent
            2000.0 + (i as f64 * 150.0),      // bytes_received
            10.0 + (i as f64 * 0.5),          // packets_sent
            15.0 + (i as f64 * 0.5),          // packets_received
            0.0,                              // failed_logins
            1.0 + (i as f64 * 0.1),           // command_freq
        ];
        data.push((features, false)); // Normal
    }

    // Port Scanning (Label: true = 1)
    for i in 0..30 {
        let features = vec![
            (50000 + i * 100) as f64,    // random high ports
            8443.0,
            0.1,                         // sehr kurz
            0.001,                       // sehr schnell
            100.0,                       // wenig data
            50.0,
            5.0,
            2.0,
            0.0,
            50.0 + (i as f64 * 5.0),     // hohe command_freq
        ];
        data.push((features, true)); // Anomaly
    }

    // Brute Force (Label: true = 1)
    for i in 0..30 {
        let features = vec![
            (40000 + i * 50) as f64,
            8443.0,
            5.0 + (i as f64 * 0.2),
            0.1,
            5000.0,
            1000.0,
            50.0,
            10.0,
            20.0 + (i as f64 * 2.0),     // viele failed_logins
            10.0,
        ];
        data.push((features, true)); // Anomaly
    }

    // DDoS Pattern (Label: true = 1)
    for i in 0..30 {
        let features = vec![
            (30000 + i * 200) as f64,
            8443.0,
            0.5,
            0.0001,                      // extrem schnell
            100000.0 + (i as f64 * 10000.0), // flood
            0.0,                         // keine response
            1000.0 + (i as f64 * 100.0), // viele packets
            0.0,
            0.0,
            100.0 + (i as f64 * 50.0),   // hohe command_freq
        ];
        data.push((features, true)); // Anomaly
    }

    // Legitimate High-Volume Users (Label: false = 0)
    for i in 0..20 {
        let features = vec![
            (2000 + i * 20) as f64,
            8443.0,
            120.0 + (i as f64 * 10.0),   // längere sessions
            0.2,
            50000.0 + (i as f64 * 5000.0), // viel data aber normal
            45000.0 + (i as f64 * 4500.0),
            100.0 + (i as f64 * 5.0),
            95.0 + (i as f64 * 4.0),
            0.0,                         // keine failed logins
            5.0 + (i as f64 * 0.5),
        ];
        data.push((features, false)); // Normal
    }

    data
}

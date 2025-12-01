/// Rhai Scripting Example
///
/// Zeigt wie man Rhai für custom anomaly detection nutzt

use honeytrap_scripting::RhaiScriptEngine;
use rhai::Dynamic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Rhai Scripting Example\n");

    let mut engine = RhaiScriptEngine::new();

    // 1. Basic Math
    println!("📊 Basic Math:");
    let result = engine.execute("40 + 2")?;
    println!("   40 + 2 = {}\n", result);

    // 2. Variables
    println!("📊 Variables:");
    engine.set_variable("threshold", Dynamic::from(0.7));
    engine.set_variable("score", Dynamic::from(0.85));
    let result = engine.execute("score > threshold")?;
    println!("   score (0.85) > threshold (0.7): {}\n", result);

    // 3. Custom Anomaly Detection Script
    println!("🤖 Custom Anomaly Detection:");
    
    let script = r#"
        fn detect_anomaly(features) {
            let score = calculate_score(features);
            log_info("Calculated score: " + score);
            
            if score > 0.8 {
                log_info("⚠️  HIGH RISK detected!");
                return "anomaly";
            } else if score > 0.5 {
                log_info("⚡ Medium risk");
                return "suspicious";
            } else {
                log_info("✅ Normal traffic");
                return "normal";
            }
        }
        
        let test_features = [443.0, 8443.0, 10.0, 0.1, 1000.0];
        detect_anomaly(test_features)
    "#;

    let result = engine.execute(script)?;
    println!("   Detection result: {}\n", result);

    // 4. Custom Detector Function
    println!("🎯 Custom Detector Function:");
    
    engine.register_custom_detector("is_port_scan", |features: rhai::Array| {
        // Check if rapid port changes
        if features.len() >= 2 {
            if let (Ok(src_port), Ok(dest_port)) = 
                (features[0].as_float(), features[1].as_float()) {
                return src_port > 50000.0 && dest_port < 1024.0;
            }
        }
        false
    });

    let scan_script = r#"
        let suspicious_features = [54321.0, 22.0, 0.1, 0.001, 100.0];
        if is_port_scan(suspicious_features) {
            log_info("🔴 Port scan detected!");
            true
        } else {
            log_info("✅ No port scan");
            false
        }
    "#;

    engine.execute(scan_script)?;

    // 5. Complex Logic
    println!("\n💡 Complex Detection Logic:");
    
    let complex_script = r#"
        fn analyze_traffic(features) {
            let src_port = features[0];
            let dest_port = features[1];
            let duration = features[2];
            let bytes = features[4];
            
            let risk_score = 0.0;
            
            // High port = suspicious
            if src_port > 50000 {
                risk_score += 0.3;
                log_info("  + High source port: +0.3");
            }
            
            // Very short duration
            if duration < 1.0 {
                risk_score += 0.2;
                log_info("  + Short duration: +0.2");
            }
            
            // High data volume
            if bytes > 100000 {
                risk_score += 0.4;
                log_info("  + High bytes: +0.4");
            }
            
            log_info("Final risk score: " + risk_score);
            
            #{
                score: risk_score,
                is_anomaly: risk_score > 0.6,
                confidence: risk_score
            }
        }
        
        let attack_features = [54321.0, 8443.0, 0.5, 0.001, 150000.0];
        analyze_traffic(attack_features)
    "#;

    let result = engine.execute(complex_script)?;
    println!("   Analysis result: {}\n", result);

    println!("✅ Rhai scripting examples completed!");

    Ok(())
}

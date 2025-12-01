/// Python Scripting Example
///
/// Zeigt wie man Python für custom anomaly detection nutzt

use honeytrap_ai::PythonScriptEngine;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🐍 Python Scripting Example\n");

    let mut engine = PythonScriptEngine::new();

    // 1. Basic Math
    println!("📊 Basic Math:");
    let result = engine.execute("40 + 2")?;
    println!("   40 + 2 = {}\n", result);

    // 2. Variables
    println!("📊 Variables:");
    engine.set_variable("threshold", json!(0.7));
    engine.set_variable("score", json!(0.85));
    let result = engine.execute("score > threshold")?;
    println!("   score (0.85) > threshold (0.7): {}\n", result);

    // 3. List Operations
    println!("📊 List Operations:");
    let features = json!([100.0, 200.0, 50.0, 300.0, 150.0]);
    engine.set_variable("features", features);
    
    let result = engine.execute("sum(features)")?;
    println!("   sum(features): {}", result);
    
    let result = engine.execute("max(features)")?;
    println!("   max(features): {}", result);
    
    let result = engine.execute("len(features)")?;
    println!("   len(features): {}\n", result);

    // 4. Math Operations
    println!("🤖 Math Operations:");
    
    let result = engine.execute("[f * 2 for f in features]")?;
    println!("   Doubled features: {}\n", result);

    // 5. Statistical Operations  
    println!("📈 Statistical Operations:");
    
    let result = engine.execute("sum(features) / len(features)")?;
    println!("   Mean: {}", result);
    
    let result = engine.execute("sorted(features)[len(features)//2]")?;
    println!("   Median: {}", result);
    
    let result = engine.execute("max(features) - min(features)")?;
    println!("   Range: {}\n", result);

    // 6. Anomaly Score Calculation
    println!("💡 Anomaly Detection:");
    
    let attack_features = json!([54321.0, 22.0, 0.5, 0.001, 150000.0]);
    engine.set_variable("attack", attack_features);
    
    let result = engine.execute("sum(attack) / len(attack)")?;
    println!("   Attack average: {}", result);
    
    let result = engine.execute("sum(attack) > 10000")?;
    println!("   Is anomaly: {}\n", result);

    // 7. List Comprehensions
    println!("🧠 List Comprehensions:");
    
    let result = engine.execute("[x for x in features if x > 100]")?;
    println!("   Values > 100: {}\n", result);

    println!("✅ Python scripting examples completed!");

    Ok(())
}

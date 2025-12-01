# Machine Learning Guide

🌲 **RandomForest** ML Model für intelligente Anomalie-Erkennung in HoneyTrap

## 📖 Überblick

HoneyTrap verwendet einen **RandomForest Classifier** (smartcore) für supervised learning:

- **Algorithmus**: Random Forest (Ensemble von Decision Trees)
- **Typ**: Binary Classification (Normal vs. Anomaly)
- **Features**: 10 Network Features
- **Backend**: smartcore 0.3

## 🏗️ Architektur

### Komponenten

```
AnomalyDetector
    ├── RandomForestModel
    │   ├── RandomForestClassifier (smartcore)
    │   ├── Config (n_trees, max_depth, ...)
    │   └── Training Metrics
    ├── Heuristic Fallback
    └── LLM Integration (optional)
```

### Data Flow

```
Network Traffic
    ↓
Feature Extraction (10 features)
    ↓
ML Prediction → Anomaly Score
    ↓ (if anomaly)
LLM Analysis (optional)
    ↓
Decision: Normal / Honeypot
```

## 📊 Features

Das Model nutzt **10 Network Features**:

| Feature           | Typ | Beschreibung                   |
| ----------------- | --- | ------------------------------ |
| source_port       | f64 | Quell-Port der Verbindung      |
| dest_port         | f64 | Ziel-Port                      |
| duration          | f64 | Session-Dauer (Sekunden)       |
| inter_packet_time | f64 | Zeit zwischen Packets          |
| bytes_sent        | f64 | Gesendete Bytes                |
| bytes_received    | f64 | Empfangene Bytes               |
| packets_sent      | f64 | Anzahl gesendeter Packets      |
| packets_received  | f64 | Anzahl empfangener Packets     |
| failed_logins     | f64 | Anzahl fehlgeschlagener Logins |
| command_freq      | f64 | Befehls-Frequenz               |

## 🔧 Konfiguration

### Model Parameters

```rust
use honeytrap_ai::random_forest::RandomForestConfig;

let config = RandomForestConfig {
    n_trees: 100,           // Anzahl Decision Trees
    max_depth: 10,          // Maximale Tree-Tiefe
    min_samples_split: 2,   // Minimum Samples für Split
};
```

**Empfehlungen**:

- **n_trees**: 50-200 (mehr = besser, aber langsamer)
- **max_depth**: 5-15 (zu tief = Overfitting)
- **min_samples_split**: 2-10 (höher = weniger Overfitting)

## 🚀 Training

### 1. Daten sammeln

```rust
let training_data: Vec<(Vec<f64>, bool)> = vec![
    // Normal traffic (label: false)
    (vec![443.0, 8443.0, 10.0, 0.1, 1000.0, 2000.0, 10.0, 15.0, 0.0, 1.0], false),

    // Attack pattern (label: true)
    (vec![12345.0, 8443.0, 0.5, 0.001, 100000.0, 50.0, 1000.0, 5.0, 10.0, 100.0], true),
];
```

### 2. Model trainieren

```rust
use honeytrap_ai::AnomalyDetector;

let mut detector = AnomalyDetector::new(100);
let accuracy = detector.train(training_data).await?;

println!("Training accuracy: {:.4}", accuracy);
```

### 3. Example ausführen

```bash
cargo run --example train_model
```

**Output**:

```
🌲 RandomForest Training Example
📊 Generating training dataset...
   Generated 210 samples
🧠 Training RandomForest model...
✅ Training accuracy: 1.0000
```

## 🔮 Prediction

### Single Sample

```rust
let features = vec![443.0, 8443.0, 10.0, 0.1, 1000.0, 2000.0, 10.0, 15.0, 0.0, 1.0];

let (is_anomaly, score) = detector.analyze(&features).await?;

if is_anomaly {
    println!("⚠️ Anomaly detected: score={:.3}", score);
}
```

### Batch Prediction

```rust
let batch = vec![
    vec![...],  // Sample 1
    vec![...],  // Sample 2
];

if let Some(ref model) = detector.ml_model {
    let predictions = model.predict_batch(batch)?;
    // predictions: Vec<usize> (0 = normal, 1 = anomaly)
}
```

## 📈 Evaluation

### Metrics

```rust
let (x_test, y_test) = load_test_data();

let metrics = model.evaluate(x_test, y_test)?;

println!("Accuracy:  {:.4}", metrics.accuracy);
println!("Precision: {:.4}", metrics.precision);
println!("Recall:    {:.4}", metrics.recall);
println!("F1 Score:  {:.4}", metrics.f1_score);
```

### Interpretation

- **Accuracy**: Gesamtkorrektheit (TP+TN)/(TP+TN+FP+FN)
- **Precision**: TP/(TP+FP) - Wie viele erkannte Anomalien sind echt?
- **Recall**: TP/(TP+FN) - Wie viele echte Anomalien wurden erkannt?
- **F1 Score**: Harmonisches Mittel von Precision und Recall

## 💾 Model Persistence

### Speichern

```rust
detector.save_model("models/honeytrap.json").await?;
```

**Hinweis**: Nur Config wird gespeichert (smartcore unterstützt keine Serialization). Model muss neu trainiert werden.

### Laden

```rust
detector.load_model("models/honeytrap.json").await?;
// Model muss neu trainiert werden!
```

## 🎯 Best Practices

### Training Data

1. **Balanced Dataset**: Gleich viele Normal/Anomaly Samples
2. **Representative**: Samples aus echtem Traffic
3. **Diverse**: Viele verschiedene Angriffsmuster
4. **Clean**: Keine Duplikate oder Fehler

**Empfohlene Größe**:

- Minimum: 100 Samples
- Optimal: 1000+ Samples
- Production: 10k+ Samples

### Feature Engineering

```rust
// Normalisierung
let normalized_port = (port - 1024.0) / (65535.0 - 1024.0);

// Log-Transform für skewed features
let log_bytes = (bytes + 1.0).ln();

// Ratio features
let bytes_ratio = bytes_sent / (bytes_received + 1.0);
```

### Model Tuning

```rust
// Grid Search Example
for n_trees in [50, 100, 150, 200] {
    for max_depth in [5, 10, 15] {
        let config = RandomForestConfig {
            n_trees,
            max_depth,
            min_samples_split: 2,
        };

        let mut model = RandomForestModel::with_config(config);
        let accuracy = model.train(x_train.clone(), y_train.clone())?;

        println!("Trees={}, Depth={}: {:.4}", n_trees, max_depth, accuracy);
    }
}
```

### Production Deployment

1. **Training Pipeline**:

   - Collect real traffic data
   - Label samples (normal/anomaly)
   - Train model regularly (weekly/monthly)
   - Validate on test set
   - Deploy if metrics improve

2. **Monitoring**:

   - Track prediction accuracy
   - Monitor false positives/negatives
   - Retrain when drift detected

3. **Fallback Strategy**:
   - Heuristic backup if ML fails
   - LLM analysis for edge cases
   - Human review for critical decisions

## 🧪 Testing

### Unit Tests

```bash
cargo test --lib random_forest
```

### Integration Tests

```bash
cargo test --workspace
```

### Example Tests

```bash
# Test training
cargo run --example train_model

# Test with real traffic
cargo run --bin honeytrap-server
```

## 📚 References

- **smartcore**: https://smartcorelib.org/
- **RandomForest**: https://en.wikipedia.org/wiki/Random_forest
- **Feature Engineering**: https://www.kaggle.com/learn/feature-engineering

## 🤝 Contributing

Verbesserungen am ML Model:

1. Neue Features hinzufügen
2. Bessere Normalisierung
3. Advanced Algorithms (XGBoost, Neural Networks)
4. AutoML Integration
5. Online Learning

## 📝 License

MIT License - see LICENSE file.

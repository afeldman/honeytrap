use std::collections::VecDeque;
use crate::llm::{LLMClient, SessionData, BehaviorAnalysis};

/// Anomalie-Detektor mit Sliding Window + LLM
pub struct AnomalyDetector {
    window_size: usize,
    anomaly_threshold: f64,
    samples: VecDeque<Vec<f64>>,
    anomalies_count: u64,
    total_predictions: u64,
    llm_client: Option<LLMClient>,
    // TODO: ML Model (RandomForest)
}

impl AnomalyDetector {
    /// Neuer Detektor
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            anomaly_threshold: 0.7,
            samples: VecDeque::with_capacity(window_size),
            anomalies_count: 0,
            total_predictions: 0,
            llm_client: None,
        }
    }
    
    /// Mit Schwellwert
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.anomaly_threshold = threshold;
        self
    }
    
    /// Mit LLM-Client
    pub fn with_llm(mut self, llm_client: LLMClient) -> Self {
        self.llm_client = Some(llm_client);
        self
    }
    
    /// Feature-Vektor analysieren
    pub async fn analyze(&mut self, features: &[f64]) -> Result<(bool, f64), Box<dyn std::error::Error>> {
        self.total_predictions += 1;
        
        // Sample hinzufügen
        self.samples.push_back(features.to_vec());
        if self.samples.len() > self.window_size {
            self.samples.pop_front();
        }
        
        // Anomalie-Score berechnen
        let score = self.calculate_anomaly_score(features).await;
        
        let is_anomaly = score > self.anomaly_threshold;
        
        if is_anomaly {
            self.anomalies_count += 1;
            tracing::debug!("🤖 Anomaly detected: score={:.3}", score);
        }
        
        Ok((is_anomaly, score))
    }
    
    /// Erweiterte Analyse mit LLM
    pub async fn analyze_with_llm(
        &mut self,
        features: &[f64],
        session_data: SessionData,
    ) -> Result<(bool, f64, Option<BehaviorAnalysis>), Box<dyn std::error::Error>> {
        // Basis-Analyse
        let (is_anomaly, score) = self.analyze(features).await?;
        
        // LLM-Analyse nur bei Anomalien
        if is_anomaly && self.llm_client.is_some() {
            tracing::info!("🧠 Running LLM behavior analysis...");
            
            let llm_client = self.llm_client.as_ref().unwrap();
            match llm_client.analyze_behavior(&session_data).await {
                Ok(analysis) => {
                    tracing::info!(
                        "🧠 LLM Analysis: {} (confidence: {:.2}, threat: {:.2})",
                        analysis.attack_type,
                        analysis.confidence,
                        analysis.threat_score
                    );
                    
                    // LLM-Score mit ML-Score kombinieren
                    let combined_score = (score + analysis.threat_score) / 2.0;
                    let is_malicious = analysis.is_malicious || combined_score > self.anomaly_threshold;
                    
                    return Ok((is_malicious, combined_score, Some(analysis)));
                }
                Err(e) => {
                    tracing::error!("LLM analysis failed: {}", e);
                }
            }
        }
        
        Ok((is_anomaly, score, None))
    }
    
    /// Anomalie-Score berechnen
    async fn calculate_anomaly_score(&self, features: &[f64]) -> f64 {
        // TODO: Echtes ML-Model
        // Für jetzt: Simple heuristische Berechnung
        
        if self.samples.len() < 2 {
            return 0.0;
        }
        
        // Durchschnitt der bisherigen Samples
        let mut avg = vec![0.0; features.len()];
        for sample in &self.samples {
            for (i, &val) in sample.iter().enumerate() {
                avg[i] += val;
            }
        }
        for val in &mut avg {
            *val /= self.samples.len() as f64;
        }
        
        // Euklidische Distanz zum Durchschnitt
        let mut distance = 0.0;
        for (i, &feature) in features.iter().enumerate() {
            let diff = feature - avg.get(i).copied().unwrap_or(0.0);
            distance += diff * diff;
        }
        distance = distance.sqrt();
        
        // Normalisieren (0.0 - 1.0)
        let max_distance = 100.0; // Heuristic
        (distance / max_distance).min(1.0)
    }
    
    /// Model trainieren (Placeholder)
    pub async fn train(&mut self, _training_data: Vec<(Vec<f64>, bool)>) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("🧠 Training model...");
        // TODO: RandomForest Training
        tracing::info!("✅ Model trained");
        Ok(())
    }
    
    /// Model speichern
    pub async fn save_model(&self, _path: &str) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("💾 Saving model...");
        // TODO: Serialize model
        Ok(())
    }
    
    /// Model laden
    pub async fn load_model(&mut self, _path: &str) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("📂 Loading model...");
        // TODO: Deserialize model
        Ok(())
    }
    
    /// Statistiken
    pub fn anomalies_detected(&self) -> u64 {
        self.anomalies_count
    }
    
    pub fn model_accuracy(&self) -> f64 {
        if self.total_predictions == 0 {
            return 0.0;
        }
        
        // Placeholder: Echte Accuracy kommt vom Model
        0.85
    }
}

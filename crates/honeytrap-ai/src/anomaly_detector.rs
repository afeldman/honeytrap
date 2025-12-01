use crate::llm::{BehaviorAnalysis, LLMClient, SessionData};
use crate::random_forest::RandomForestModel;
use std::collections::VecDeque;

/// Anomalie-Detektor mit RandomForest ML + LLM
pub struct AnomalyDetector {
    window_size: usize,
    anomaly_threshold: f64,
    samples: VecDeque<Vec<f64>>,
    anomalies_count: u64,
    total_predictions: u64,
    llm_client: Option<LLMClient>,
    
    /// RandomForest ML Model
    ml_model: Option<RandomForestModel>,
    
    /// Verwende ML-Model für Predictions?
    use_ml_model: bool,
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
            ml_model: Some(RandomForestModel::new()),
            use_ml_model: false, // Erst nach Training aktivieren
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
    pub async fn analyze(
        &mut self,
        features: &[f64],
    ) -> Result<(bool, f64), Box<dyn std::error::Error>> {
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
                    let is_malicious =
                        analysis.is_malicious || combined_score > self.anomaly_threshold;

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
        // Wenn ML-Model trainiert ist, nutze es
        if self.use_ml_model {
            if let Some(ref model) = self.ml_model {
                match model.predict(features) {
                    Ok((prediction, probability)) => {
                        // prediction: 0 = normal, 1 = anomaly
                        if prediction == 1 {
                            return probability;
                        } else {
                            return 1.0 - probability;
                        }
                    }
                    Err(e) => {
                        tracing::warn!("ML prediction failed: {}, falling back to heuristic", e);
                    }
                }
            }
        }

        // Fallback: Heuristische Berechnung
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

    /// Model trainieren
    pub async fn train(
        &mut self,
        training_data: Vec<(Vec<f64>, bool)>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        if training_data.is_empty() {
            return Err("Training data is empty".into());
        }

        tracing::info!("🧠 Training RandomForest model with {} samples", training_data.len());

        // Daten für smartcore vorbereiten
        let mut x_train = Vec::new();
        let mut y_train = Vec::new();

        for (features, is_anomaly) in training_data {
            x_train.push(features);
            y_train.push(if is_anomaly { 1 } else { 0 });
        }

        // Model trainieren
        let model = self.ml_model.as_mut()
            .ok_or("ML model not initialized")?;
        
        let accuracy = model.train(x_train, y_train)?;
        
        self.use_ml_model = true;

        tracing::info!("✅ Model trained with accuracy: {:.4}", accuracy);

        Ok(accuracy)
    }

    /// Model speichern
    pub async fn save_model(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("💾 Saving model to {}", path);
        
        if let Some(ref model) = self.ml_model {
            model.save(path)?;
        } else {
            return Err("No model to save".into());
        }
        
        Ok(())
    }

    /// Model laden
    pub async fn load_model(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("📂 Loading model from {}", path);
        
        let loaded = RandomForestModel::load(path)?;
        self.ml_model = Some(loaded);
        
        Ok(())
    }

    /// Statistiken
    pub fn anomalies_detected(&self) -> u64 {
        self.anomalies_count
    }

    pub fn model_accuracy(&self) -> f64 {
        // Nutze echte Accuracy wenn ML-Model trainiert
        if self.use_ml_model {
            if let Some(ref model) = self.ml_model {
                return model.accuracy();
            }
        }

        // Fallback: Heuristic
        0.0
    }
    
    /// ML-Model Status
    pub fn is_ml_trained(&self) -> bool {
        self.use_ml_model
    }
    
    /// Feature-Anzahl
    pub fn feature_count(&self) -> usize {
        if let Some(ref model) = self.ml_model {
            if model.is_trained() {
                return 10; // NetworkFeatures hat 10 Features
            }
        }
        0
    }
}

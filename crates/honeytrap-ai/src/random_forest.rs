//! RandomForest ML Model für Anomalie-Erkennung
//!
//! Implementiert einen Random Forest Klassifikator mit smartcore

use serde::{Deserialize, Serialize};
use smartcore::ensemble::random_forest_classifier::RandomForestClassifier;
use smartcore::linalg::basic::matrix::DenseMatrix;
use std::error::Error;
use std::fs;

/// Configuration for RandomForest model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomForestConfig {
    pub n_trees: u16,
    pub max_depth: u16,
    pub min_samples_split: usize,
}

impl Default for RandomForestConfig {
    fn default() -> Self {
        Self {
            n_trees: 100,
            max_depth: 10,
            min_samples_split: 2,
        }
    }
}

/// RandomForest model for anomaly detection
#[derive(Debug, Serialize, Deserialize)]
pub struct RandomForestModel {
    #[serde(skip)]
    classifier: Option<RandomForestClassifier<f64, usize, DenseMatrix<f64>, Vec<usize>>>,
    config: RandomForestConfig,
    accuracy: f64,
    is_trained: bool,
}

impl RandomForestModel {
    /// Create new model with default configuration
    pub fn new() -> Self {
        Self {
            classifier: None,
            config: RandomForestConfig::default(),
            accuracy: 0.0,
            is_trained: false,
        }
    }

    /// Create new model with custom configuration
    pub fn with_config(config: RandomForestConfig) -> Self {
        Self {
            classifier: None,
            config,
            accuracy: 0.0,
            is_trained: false,
        }
    }

    /// Train the RandomForest model
    pub fn train(
        &mut self,
        x_train: Vec<Vec<f64>>,
        y_train: Vec<usize>,
    ) -> Result<f64, Box<dyn Error>> {
        tracing::info!(
            "🌲 Training RandomForest: {} samples, {} features",
            x_train.len(),
            x_train[0].len()
        );

        use smartcore::ensemble::random_forest_classifier::RandomForestClassifierParameters;

        // SmartCore RandomForestClassifier Parameter
        let params = RandomForestClassifierParameters::default()
            .with_n_trees(self.config.n_trees)
            .with_max_depth(self.config.max_depth)
            .with_min_samples_split(self.config.min_samples_split);

        // Convert to smartcore format
        let x_dense = DenseMatrix::from_2d_vec(&x_train);

        // Train model
        let trained_classifier = RandomForestClassifier::fit(&x_dense, &y_train, params)?;

        // Calculate accuracy on training data
        let predictions = trained_classifier.predict(&x_dense)?;
        let correct = predictions
            .iter()
            .zip(y_train.iter())
            .filter(|(pred, actual)| pred == actual)
            .count();

        self.accuracy = correct as f64 / y_train.len() as f64;
        self.classifier = Some(trained_classifier);
        self.is_trained = true;

        tracing::info!("✅ Training accuracy: {:.4}", self.accuracy);

        Ok(self.accuracy)
    }

    /// Make prediction for single sample
    pub fn predict(&self, features: &[f64]) -> Result<(usize, f64), Box<dyn Error>> {
        if !self.is_trained {
            return Err("Model not trained yet".into());
        }

        let classifier = self
            .classifier
            .as_ref()
            .ok_or("Model not initialized")?;

        // Convert to smartcore format (1 sample)
        let x_vec = vec![features.to_vec()];
        let x = DenseMatrix::from_2d_vec(&x_vec);

        // Predict
        let predictions = classifier.predict(&x)?;
        let prediction = predictions[0];

        // Get probability (simplified - smartcore doesn't expose predict_proba for all classifiers)
        let probability = if prediction == 1 { 0.8 } else { 0.2 };

        Ok((prediction, probability))
    }

    /// Make predictions for batch of samples
    pub fn predict_batch(&self, features: Vec<Vec<f64>>) -> Result<Vec<usize>, Box<dyn Error>> {
        if !self.is_trained {
            return Err("Model not trained yet".into());
        }

        let classifier = self
            .classifier
            .as_ref()
            .ok_or("Model not initialized")?;

        let x_dense = DenseMatrix::from_2d_vec(&features);
        let predictions = classifier.predict(&x_dense)?;

        Ok(predictions)
    }

    /// Evaluate model on test set
    pub fn evaluate(
        &self,
        x_test: Vec<Vec<f64>>,
        y_test: Vec<usize>,
    ) -> Result<ModelMetrics, Box<dyn Error>> {
        if !self.is_trained {
            return Err("Model not trained yet".into());
        }

        let classifier = self
            .classifier
            .as_ref()
            .ok_or("Model not initialized")?;

        let x_dense = DenseMatrix::from_2d_vec(&x_test);
        let predictions = classifier.predict(&x_dense)?;

        // Calculate metrics
        let mut tp = 0; // True Positives
        let mut fp = 0; // False Positives
        let mut tn = 0; // True Negatives
        let mut fn_ = 0; // False Negatives

        for (pred, actual) in predictions.iter().zip(y_test.iter()) {
            match (*pred, *actual) {
                (1, 1) => tp += 1,
                (1, 0) => fp += 1,
                (0, 0) => tn += 1,
                (0, 1) => fn_ += 1,
                _ => {}
            }
        }

        let accuracy = (tp + tn) as f64 / predictions.len() as f64;
        let precision = if tp + fp > 0 {
            tp as f64 / (tp + fp) as f64
        } else {
            0.0
        };
        let recall = if tp + fn_ > 0 {
            tp as f64 / (tp + fn_) as f64
        } else {
            0.0
        };
        let f1_score = if precision + recall > 0.0 {
            2.0 * precision * recall / (precision + recall)
        } else {
            0.0
        };

        Ok(ModelMetrics {
            accuracy,
            precision,
            recall,
            f1_score,
        })
    }

    /// Save model to file
    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        if !self.is_trained {
            return Err("Cannot save untrained model".into());
        }

        let data = ModelData {
            config: self.config.clone(),
            accuracy: self.accuracy,
            is_trained: self.is_trained,
        };

        let json = serde_json::to_string_pretty(&data)?;
        fs::write(path, json)?;

        Ok(())
    }

    /// Load model from file
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let json = fs::read_to_string(path)?;
        let data: ModelData = serde_json::from_str(&json)?;

        Ok(Self {
            classifier: None, // Cannot serialize RandomForest, needs retraining
            config: data.config,
            accuracy: data.accuracy,
            is_trained: false, // Mark as not trained since classifier is None
        })
    }

    /// Get model accuracy
    pub fn accuracy(&self) -> f64 {
        self.accuracy
    }

    /// Check if model is trained
    pub fn is_trained(&self) -> bool {
        self.is_trained
    }
}

impl Default for RandomForestModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Model data for serialization
#[derive(Debug, Serialize, Deserialize)]
struct ModelData {
    config: RandomForestConfig,
    accuracy: f64,
    is_trained: bool,
}

/// Model evaluation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_creation() {
        let model = RandomForestModel::new();
        assert!(!model.is_trained());
        assert_eq!(model.accuracy(), 0.0);
    }

    #[test]
    fn test_model_training() {
        let mut model = RandomForestModel::new();

        let x = vec![
            vec![1.0, 2.0],
            vec![2.0, 3.0],
            vec![3.0, 4.0],
            vec![4.0, 5.0],
        ];
        let y = vec![0, 0, 1, 1];

        let accuracy = model.train(x, y).unwrap();

        assert!(model.is_trained());
        assert!(accuracy > 0.0);
        assert_eq!(model.accuracy(), accuracy);
    }
}

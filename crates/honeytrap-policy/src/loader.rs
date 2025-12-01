use crate::model::PolicyFile;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("Failed to read policy file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
}

/// Policy Loader - loads policies from files
pub struct PolicyLoader;

impl PolicyLoader {
    /// Load policies from a file (YAML or JSON)
    pub fn load_from_file(path: &Path) -> Result<PolicyFile, LoaderError> {
        let content = std::fs::read_to_string(path)?;
        
        match path.extension().and_then(|e| e.to_str()) {
            Some("yaml") | Some("yml") => {
                Ok(serde_yaml::from_str(&content)?)
            }
            Some("json") => {
                Ok(serde_json::from_str(&content)?)
            }
            Some(ext) => Err(LoaderError::UnsupportedFormat(ext.to_string())),
            None => Err(LoaderError::UnsupportedFormat("no extension".to_string())),
        }
    }
    
    /// Load policies from multiple files
    pub fn load_from_files(paths: &[String]) -> Result<Vec<PolicyFile>, LoaderError> {
        let mut policy_files = Vec::new();
        
        for path_str in paths {
            let path = Path::new(path_str);
            if path.exists() {
                match Self::load_from_file(path) {
                    Ok(pf) => {
                        tracing::info!("Loaded policies from: {}", path_str);
                        policy_files.push(pf);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load policy file {}: {}", path_str, e);
                    }
                }
            } else {
                tracing::warn!("Policy file not found: {}", path_str);
            }
        }
        
        Ok(policy_files)
    }
}

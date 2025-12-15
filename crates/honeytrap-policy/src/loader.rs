//! Policy File Loader Module
//!
//! This module handles loading policy definitions from YAML and JSON files.
//!
//! # Supported Formats
//!
//! - YAML files (`.yaml`, `.yml`)
//! - JSON files (`.json`)
//!
//! # Example Policy File (YAML)
//!
//! ```yaml
//! version: v1
//! policies:
//!   - name: allow_trusted
//!     description: "Allow trusted internal services"
//!     priority: 10
//!     conditions:
//!       all:
//!         - mtls_verified: true
//!         - client_san_contains: "internal.service"
//!     action:
//!       type: ALLOW
//!       log: true
//! ```

use crate::model::PolicyFile;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur when loading policy files.
#[derive(Error, Debug)]
pub enum LoaderError {
    /// I/O error reading the file
    #[error("Failed to read policy file: {0}")]
    IoError(#[from] std::io::Error),
    
    /// YAML parsing error
    #[error("Failed to parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),
    
    /// JSON parsing error
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    
    /// Unsupported file format (not YAML or JSON)
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
}

/// Utility for loading policy files from disk.
///
/// PolicyLoader provides methods to load and parse policy files in YAML or JSON format.
/// Files are automatically parsed based on their extension.
///
/// # Examples
///
/// ```no_run
/// use honeytrap_policy::loader::PolicyLoader;
/// use std::path::Path;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Load a single file
/// let policy_file = PolicyLoader::load_from_file(Path::new("policies/base.yaml"))?;
/// println!("Loaded {} policies", policy_file.policies.len());
///
/// // Load multiple files
/// let paths = vec![
///     "policies/base.yaml".to_string(),
///     "policies/custom.yaml".to_string(),
/// ];
/// let policy_files = PolicyLoader::load_from_files(&paths)?;
/// println!("Loaded {} policy files", policy_files.len());
/// # Ok(())
/// # }
/// ```
pub struct PolicyLoader;

impl PolicyLoader {
    /// Loads and parses a single policy file.
    ///
    /// The file format (YAML or JSON) is determined by the file extension.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the policy file
    ///
    /// # Returns
    ///
    /// * `Ok(PolicyFile)` - Successfully parsed policy file
    /// * `Err(LoaderError)` - Failed to read or parse the file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use honeytrap_policy::loader::PolicyLoader;
    /// use std::path::Path;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Load YAML file
    /// let yaml_policies = PolicyLoader::load_from_file(
    ///     Path::new("policies/base-policies.yaml")
    /// )?;
    ///
    /// // Load JSON file
    /// let json_policies = PolicyLoader::load_from_file(
    ///     Path::new("policies/custom.json")
    /// )?;
    ///
    /// for policy in &yaml_policies.policies {
    ///     println!("Policy: {} (priority: {})", policy.name, policy.priority);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// - `LoaderError::IoError`: File doesn't exist or can't be read
    /// - `LoaderError::YamlError`: Invalid YAML syntax
    /// - `LoaderError::JsonError`: Invalid JSON syntax
    /// - `LoaderError::UnsupportedFormat`: File extension is not .yaml, .yml, or .json
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
    
    /// Loads and parses multiple policy files.
    ///
    /// This method attempts to load all specified files. Files that don't exist or
    /// have parse errors are logged as warnings but don't cause the entire operation
    /// to fail. At least one successfully loaded file will result in Ok.
    ///
    /// # Arguments
    ///
    /// * `paths` - Slice of file path strings
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<PolicyFile>)` - Vector of successfully loaded policy files (may be empty)
    /// * `Err(LoaderError)` - Currently never returns Err, but signature allows for future validation
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use honeytrap_policy::loader::PolicyLoader;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let paths = vec![
    ///     "policies/base-policies.yaml".to_string(),
    ///     "policies/high-risk.yaml".to_string(),
    ///     "policies/custom.json".to_string(),
    /// ];
    ///
    /// let policy_files = PolicyLoader::load_from_files(&paths)?;
    ///
    /// let total_policies: usize = policy_files.iter()
    ///     .map(|pf| pf.policies.len())
    ///     .sum();
    ///
    /// println!("Loaded {} policies from {} files", total_policies, policy_files.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Behavior
    ///
    /// - Non-existent files: Logged as warning, continue processing
    /// - Parse errors: Logged as warning, continue processing
    /// - Returns all successfully loaded files
    /// - Empty result if no files could be loaded
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

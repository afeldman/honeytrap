/// Scripting interfaces for ML model interaction
///
/// Supports Python (PyO3) and Rhai scripting

pub mod python;
pub mod rhai_engine;

pub use python::PythonScriptEngine;
pub use rhai_engine::RhaiScriptEngine;

use std::error::Error;

/// Script engine trait for different scripting languages
pub trait ScriptEngine {
    /// Execute a script and return result
    fn execute(&mut self, script: &str) -> Result<String, Box<dyn Error>>;
    
    /// Load script from file
    fn load_file(&mut self, path: &str) -> Result<(), Box<dyn Error>>;
    
    /// Set variable in script context
    fn set_variable(&mut self, name: &str, value: serde_json::Value) -> Result<(), Box<dyn Error>>;
    
    /// Get variable from script context
    fn get_variable(&self, name: &str) -> Result<serde_json::Value, Box<dyn Error>>;
}

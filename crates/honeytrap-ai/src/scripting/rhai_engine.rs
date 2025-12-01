/// Rhai scripting engine for ML model interaction
///
/// Allows dynamic scripting of anomaly detection logic

use rhai::{Dynamic, Engine, EvalAltResult, Scope};
use serde_json::Value;
use std::error::Error;
use super::ScriptEngine;

pub struct RhaiScriptEngine {
    engine: Engine,
    scope: Scope<'static>,
}

impl RhaiScriptEngine {
    /// Create new Rhai engine with ML functions
    pub fn new() -> Self {
        let mut engine = Engine::new();
        
        // Register custom functions for ML
        engine.register_fn("log_info", |msg: &str| {
            tracing::info!("🔧 Rhai: {}", msg);
        });
        
        engine.register_fn("calculate_score", |features: rhai::Array| -> f64 {
            // Simple heuristic scoring
            let mut score: f64 = 0.0;
            for (i, feature) in features.iter().enumerate() {
                if let Ok(val) = feature.as_float() {
                    score += val * (i as f64 + 1.0) * 0.01;
                }
            }
            score.min(1.0)
        });
        
        Self {
            engine,
            scope: Scope::new(),
        }
    }
    
    /// Execute Rhai script with current scope
    pub fn execute(&mut self, script: &str) -> Result<Dynamic, Box<EvalAltResult>> {
        self.engine.eval_with_scope(&mut self.scope, script)
    }
    
    /// Execute script from file
    pub fn execute_file(&mut self, path: &str) -> Result<Dynamic, Box<dyn Error>> {
        let script = std::fs::read_to_string(path)?;
        Ok(self.execute(&script)?)
    }
    
    /// Set variable in scope
    pub fn set_variable(&mut self, name: &str, value: Dynamic) {
        self.scope.push(name, value);
    }
    
    /// Get variable from scope
    pub fn get_variable(&self, name: &str) -> Option<Dynamic> {
        self.scope.get_value(name)
    }
    
    /// Register custom anomaly detection function
    pub fn register_custom_detector<F>(&mut self, name: &str, func: F)
    where
        F: Fn(rhai::Array) -> bool + Send + Sync + 'static,
    {
        self.engine.register_fn(name, func);
    }
}

impl Default for RhaiScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ScriptEngine for RhaiScriptEngine {
    fn execute(&mut self, script: &str) -> Result<String, Box<dyn Error>> {
        let result = self.execute(script)?;
        Ok(result.to_string())
    }
    
    fn load_file(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let _ = self.execute_file(path)?;
        Ok(())
    }
    
    fn set_variable(&mut self, name: &str, value: Value) -> Result<(), Box<dyn Error>> {
        let dynamic = json_to_dynamic(value)?;
        self.set_variable(name, dynamic);
        Ok(())
    }
    
    fn get_variable(&self, name: &str) -> Result<Value, Box<dyn Error>> {
        let dynamic = self.get_variable(name)
            .ok_or("Variable not found")?;
        dynamic_to_json(dynamic)
    }
}

/// Convert JSON to Rhai Dynamic
fn json_to_dynamic(value: Value) -> Result<Dynamic, Box<dyn Error>> {
    match value {
        Value::Null => Ok(Dynamic::UNIT),
        Value::Bool(b) => Ok(Dynamic::from(b)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Dynamic::from(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Dynamic::from(f))
            } else {
                Err("Invalid number".into())
            }
        }
        Value::String(s) => Ok(Dynamic::from(s)),
        Value::Array(arr) => {
            let mut rhai_arr = rhai::Array::new();
            for item in arr {
                rhai_arr.push(json_to_dynamic(item)?);
            }
            Ok(Dynamic::from(rhai_arr))
        }
        Value::Object(obj) => {
            let mut rhai_map = rhai::Map::new();
            for (k, v) in obj {
                rhai_map.insert(k.into(), json_to_dynamic(v)?);
            }
            Ok(Dynamic::from(rhai_map))
        }
    }
}

/// Convert Rhai Dynamic to JSON
fn dynamic_to_json(value: Dynamic) -> Result<Value, Box<dyn Error>> {
    if value.is_unit() {
        Ok(Value::Null)
    } else if value.is::<bool>() {
        Ok(Value::Bool(value.cast::<bool>()))
    } else if value.is::<i64>() {
        Ok(Value::Number(value.cast::<i64>().into()))
    } else if value.is::<f64>() {
        let f = value.cast::<f64>();
        Ok(serde_json::Number::from_f64(f)
            .map(Value::Number)
            .unwrap_or(Value::Null))
    } else if value.is::<String>() {
        Ok(Value::String(value.cast::<String>()))
    } else if value.is::<rhai::Array>() {
        let arr = value.cast::<rhai::Array>();
        let json_arr: Result<Vec<_>, _> = arr.into_iter()
            .map(dynamic_to_json)
            .collect();
        Ok(Value::Array(json_arr?))
    } else {
        Ok(Value::String(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rhai_basic() {
        let mut engine = RhaiScriptEngine::new();
        let result = engine.execute("40 + 2").unwrap();
        assert_eq!(result.to_string(), "42");
    }
    
    #[test]
    fn test_rhai_variables() {
        let mut engine = RhaiScriptEngine::new();
        engine.set_variable("threshold", Dynamic::from(0.7));
        let result = engine.execute("threshold * 2").unwrap();
        assert_eq!(result.as_float().unwrap(), 1.4);
    }
    
    #[test]
    fn test_rhai_custom_function() {
        let mut engine = RhaiScriptEngine::new();
        let features = vec![
            Dynamic::from(100.0),
            Dynamic::from(200.0),
            Dynamic::from(50.0),
        ];
        engine.set_variable("features", Dynamic::from(features));
        let result = engine.execute("calculate_score(features)").unwrap();
        assert!(result.as_float().unwrap() > 0.0);
    }
}

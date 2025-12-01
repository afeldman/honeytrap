/// Python scripting engine for ML model interaction
///
/// Allows using Python for custom anomaly detection logic

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct PythonScriptEngine {
    variables: HashMap<String, Value>,
}

impl PythonScriptEngine {
    /// Create new Python engine
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    
    /// Execute Python code with current context
    pub fn execute(&mut self, code: &str) -> Result<String, Box<dyn Error>> {
        Python::with_gil(|py| {
            // Create global namespace
            let globals = PyDict::new_bound(py);
            
            // Add variables to namespace
            for (name, value) in &self.variables {
                let py_value = json_to_python(py, value)?;
                globals.set_item(name, py_value)?;
            }
            
            // Execute code
            let result = py.eval_bound(code, Some(&globals), None)?;
            
            // Extract result
            Ok(result.to_string())
        })
    }
    
    /// Execute Python script from file
    pub fn execute_file(&mut self, path: &str) -> Result<String, Box<dyn Error>> {
        let code = std::fs::read_to_string(path)?;
        self.execute(&code)
    }
    
    /// Set variable in context
    pub fn set_variable(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }
    
    /// Get variable from context
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    /// Call Python function with arguments
    pub fn call_function(&mut self, func_name: &str, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        Python::with_gil(|py| {
            let globals = PyDict::new_bound(py);
            
            // Add variables
            for (name, value) in &self.variables {
                let py_value = json_to_python(py, value)?;
                globals.set_item(name, py_value)?;
            }
            
            // Convert args to Python
            let py_args: Result<Vec<_>, _> = args.iter()
                .map(|v| json_to_python(py, v))
                .collect();
            let py_args = py_args?;
            
            // Get function and call it
            let func = globals.get_item(func_name)?
                .ok_or("Function not found")?;
            let result = func.call1(PyTuple::new_bound(py, py_args))?;
            
            // Convert result back
            python_to_json(&result)
        })
    }
}

impl Default for PythonScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl super::ScriptEngine for PythonScriptEngine {
    fn execute(&mut self, script: &str) -> Result<String, Box<dyn Error>> {
        self.execute(script)
    }
    
    fn load_file(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        self.execute_file(path)?;
        Ok(())
    }
    
    fn set_variable(&mut self, name: &str, value: Value) -> Result<(), Box<dyn Error>> {
        self.set_variable(name, value);
        Ok(())
    }
    
    fn get_variable(&self, name: &str) -> Result<Value, Box<dyn Error>> {
        self.get_variable(name)
            .cloned()
            .ok_or_else(|| "Variable not found".into())
    }
}

/// Convert JSON to Python object
fn json_to_python(py: Python, value: &Value) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.to_object(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.to_object(py))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid number"))
            }
        }
        Value::String(s) => Ok(s.to_object(py)),
        Value::Array(arr) => {
            let py_list = PyList::empty_bound(py);
            for item in arr {
                py_list.append(json_to_python(py, item)?)?;
            }
            Ok(py_list.to_object(py))
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new_bound(py);
            for (k, v) in obj {
                py_dict.set_item(k, json_to_python(py, v)?)?;
            }
            Ok(py_dict.to_object(py))
        }
    }
}

/// Convert Python object to JSON
fn python_to_json(obj: &Bound<PyAny>) -> Result<Value, Box<dyn Error>> {
    if obj.is_none() {
        Ok(Value::Null)
    } else if let Ok(b) = obj.extract::<bool>() {
        Ok(Value::Bool(b))
    } else if let Ok(i) = obj.extract::<i64>() {
        Ok(Value::Number(i.into()))
    } else if let Ok(f) = obj.extract::<f64>() {
        Ok(serde_json::Number::from_f64(f)
            .map(Value::Number)
            .unwrap_or(Value::Null))
    } else if let Ok(s) = obj.extract::<String>() {
        Ok(Value::String(s))
    } else if let Ok(list) = obj.downcast::<PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            arr.push(python_to_json(&item)?);
        }
        Ok(Value::Array(arr))
    } else if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (k, v) in dict.iter() {
            let key = k.extract::<String>()?;
            map.insert(key, python_to_json(&v)?);
        }
        Ok(Value::Object(map))
    } else {
        Ok(Value::String(obj.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_python_basic() {
        let mut engine = PythonScriptEngine::new();
        let result = engine.execute("40 + 2").unwrap();
        assert_eq!(result, "42");
    }
    
    #[test]
    fn test_python_variables() {
        let mut engine = PythonScriptEngine::new();
        engine.set_variable("threshold", Value::from(0.7));
        let result = engine.execute("threshold * 2").unwrap();
        assert_eq!(result, "1.4");
    }
    
    #[test]
    fn test_python_list() {
        let mut engine = PythonScriptEngine::new();
        let features = Value::Array(vec![
            Value::from(100.0),
            Value::from(200.0),
            Value::from(50.0),
        ]);
        engine.set_variable("features", features);
        let result = engine.execute("sum(features)").unwrap();
        assert_eq!(result, "350.0");
    }
}

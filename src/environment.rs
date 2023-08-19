use std::collections::HashMap;

use crate::{error::LoxRuntimeError, value::Value};

pub struct Environment {
    map: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: HashMap::new(),
        }
    }

    pub fn define(&mut self, k: String, v: Option<Value>) {
        self.map.insert(k, v.unwrap_or(Value::Nil));
    }

    pub fn get(&self, k: &str) -> Result<Value, LoxRuntimeError> {
        let v = self.map.get(k).ok_or(LoxRuntimeError {
            message: format!("Undefined variable {}", k),
        })?;
        Ok(v.to_owned())
    }
}

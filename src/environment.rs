use std::collections::HashMap;

use crate::{error::LoxRuntimeError, value::Value};

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    map: HashMap<String, Value>,
}

impl Environment {
    pub fn new_global() -> Environment {
        Environment {
            map: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_local(&self) -> Environment {
        Environment {
            map: HashMap::new(),
            enclosing: Some(Box::new(self.clone())),
        }
    }

    pub fn define(&mut self, k: String, v: Option<Value>) {
        self.map.insert(k, v.unwrap_or(Value::Nil));
    }

    pub fn assign(&mut self, k: String, v: Value) -> Result<(), LoxRuntimeError> {
        if self.map.contains_key(&k) {
            self.map.insert(k, v);
            Ok(())
        } else if let Some(enclosing_environment) = &mut self.enclosing {
            enclosing_environment.as_mut().assign(k, v)
        } else {
            Err(LoxRuntimeError {
                message: format!("Undefined variable {}", k),
            })
        }
    }

    pub fn get(&self, k: &str) -> Result<Value, LoxRuntimeError> {
        let v = self.map.get(k);

        if let Some(v) = v {
            Ok(v.to_owned())
        } else if let (None, Some(enclosing_environment)) = (v, &self.enclosing) {
            enclosing_environment.get(k)
        } else {
            Err(LoxRuntimeError {
                message: format!("Undefined variable {}", k),
            })
        }
    }
}

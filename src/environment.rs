use std::collections::HashMap;

use crate::{error::LoxRuntimeError, value::LoxValue};

pub struct Environment {
    map: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: HashMap::new(),
        }
    }

    pub fn define(&mut self, k: String, v: Option<LoxValue>) {
        self.map.insert(k, v.unwrap_or(LoxValue::LoxNil));
    }

    pub fn get(&self, k: &str) -> Result<LoxValue, LoxRuntimeError> {
        let v = self.map.get(k).ok_or(LoxRuntimeError {
            message: format!("Undefined variable {}", k),
        })?;
        Ok(v.to_owned())
    }
}

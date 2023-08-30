use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{cell::RefCell, collections::HashMap};

use crate::{error::LoxRuntimeError, value::Value};

pub struct Environment<'a> {
    enclosing: Option<&'a Environment<'a>>,
    map: RefCell<HashMap<String, Value>>,
}

impl<'a> Environment<'a> {
    pub fn new_global() -> Environment<'static> {
        let global = Environment {
            map: RefCell::new(HashMap::new()),
            enclosing: None,
        };

        global.define(
            "clock".to_owned(),
            // TODO: Move definition to a standard lib
            Some(Value::Callable(
                "clock".to_owned(),
                0,
                Rc::new(|| {
                    let start = SystemTime::now();
                    let since_the_epoch = start
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs();

                    Value::Number(since_the_epoch as f64)
                }),
            )),
        );

        global
    }

    pub fn new_local(&'a self) -> Environment<'a> {
        Environment {
            map: RefCell::new(HashMap::new()),
            enclosing: Some(self),
        }
    }

    pub fn define(&self, k: String, v: Option<Value>) {
        self.map.borrow_mut().insert(k, v.unwrap_or(Value::Nil));
    }

    pub fn assign(&self, k: String, v: Value) -> Result<(), LoxRuntimeError> {
        if self.map.borrow().contains_key(&k) {
            self.map.borrow_mut().insert(k, v);
            Ok(())
        } else if let Some(enclosing_environment) = &self.enclosing {
            enclosing_environment.assign(k, v)
        } else {
            Err(LoxRuntimeError {
                message: format!("Undefined variable {}", k),
            })
        }
    }

    pub fn get(&self, k: &str) -> Result<Value, LoxRuntimeError> {
        let map = self.map.borrow();
        let v = map.get(k);

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

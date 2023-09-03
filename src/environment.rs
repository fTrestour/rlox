use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::standard::clock;
use crate::{error::LoxRuntimeException, value::Value};

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    locals: Rc<RefCell<HashMap<String, Value>>>,
}

impl Environment {
    pub fn new_global() -> Environment {
        let global = Environment {
            locals: Rc::new(RefCell::new(HashMap::new())),
            enclosing: None,
        };

        global.define(
            "clock".to_owned(),
            Some(Value::NativeCallable("clock".to_owned(), 0, clock)),
        );

        global
    }

    pub fn new_local(&self) -> Environment {
        Environment {
            locals: Rc::new(RefCell::new(HashMap::new())),
            enclosing: Some(Box::new(self.clone())),
        }
    }

    pub fn define(&self, k: String, v: Option<Value>) {
        self.locals.borrow_mut().insert(k, v.unwrap_or(Value::Nil));
    }

    pub fn assign(&self, k: String, v: Value) -> Result<(), LoxRuntimeException> {
        if self.locals.borrow().contains_key(&k) {
            self.locals.borrow_mut().insert(k, v);
            Ok(())
        } else if let Some(enclosing_environment) = &self.enclosing {
            enclosing_environment.assign(k, v)
        } else {
            Err(LoxRuntimeException::Error(format!(
                "Undefined variable {}",
                k
            )))
        }
    }

    pub fn get(&self, k: &str) -> Result<Value, LoxRuntimeException> {
        let map = self.locals.borrow();
        let v = map.get(k);

        if let Some(v) = v {
            Ok(v.to_owned())
        } else if let (None, Some(enclosing_environment)) = (v, &self.enclosing) {
            enclosing_environment.get(k)
        } else {
            Err(LoxRuntimeException::Error(format!(
                "Undefined variable {}",
                k
            )))
        }
    }
}

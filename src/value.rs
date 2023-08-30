use std::{fmt::Display, rc::Rc};

use crate::{environment::Environment, error::LoxRuntimeError};
#[derive(Clone)]
pub enum Value {
    Nil,
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(String, usize, Rc<dyn Fn() -> Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Number(n) if n.fract() == 0. => write!(f, "{}", n),
            Value::Number(n) => write!(f, "{:.2}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Callable(name, _, _) => write!(f, "{}()", name),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil | Value::Boolean(false) => false,
            _ => true,
        }
    }

    pub fn as_number(&self) -> Result<f64, LoxRuntimeError> {
        match *self {
            Value::Number(n) => Ok(n),
            _ => Err(LoxRuntimeError {
                message: format!("{} is not a number", self),
            }),
        }
    }

    pub fn call(&self, _: &Environment, args: Vec<Value>) -> Result<Value, LoxRuntimeError> {
        if let Value::Callable(_, arity, call) = self {
            if args.len() != *arity {
                Err(LoxRuntimeError {
                    message: format!("Expected {} arguments but got {}.", arity, args.len()),
                })
            } else {
                Ok(call())
            }
        } else {
            Err(LoxRuntimeError {
                message: "Can only call functions and classes".to_owned(),
            })
        }
    }

    pub fn is_equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
            _ => false,
        }
    }
}

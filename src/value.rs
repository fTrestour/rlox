use std::fmt::Display;

use crate::{
    environment::Environment, error::LoxRuntimeException, grammar::Declaration,
    interpreter::interpret,
};
#[derive(Clone)]
pub enum Value {
    Nil,
    Number(f64),
    String(String),
    Boolean(bool),
    NativeCallable(String, usize, fn(Vec<Value>) -> Value),
    Callable(String, Environment, Vec<String>, Declaration),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Number(n) if n.fract() == 0. => write!(f, "{}", n),
            Value::Number(n) => write!(f, "{:.2}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Callable(name, _, _, _) => write!(f, "<fn {}>", name),
            Value::NativeCallable(name, _, _) => write!(f, "<fn {}>", name),
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

    pub fn as_number(&self) -> Result<f64, LoxRuntimeException> {
        match *self {
            Value::Number(n) => Ok(n),
            _ => Err(LoxRuntimeException::Error(format!(
                "{} is not a number",
                self
            ))),
        }
    }

    pub fn call(&self, args: Vec<Value>) -> Result<Value, LoxRuntimeException> {
        match self {
            Value::NativeCallable(name, arity, f) => {
                check_callable_arity(&args, *arity, name)?;

                Ok(f(args))
            }
            Value::Callable(name, closure, parameters, body) => {
                check_callable_arity(&args, parameters.len(), name)?;

                let local_environment = closure.new_local();
                for (name, value) in parameters.iter().zip(args.iter()) {
                    local_environment.define(name.clone(), Some(value.clone()));
                }

                match interpret(body.clone(), &local_environment) {
                    Ok(_) => Ok(Value::Nil),
                    Err(LoxRuntimeException::Return(value)) => Ok(value),
                    Err(e) => Err(e),
                }
            }
            _ => Err(LoxRuntimeException::Error(
                "Can only call functions and classes".to_owned(),
            )),
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

fn check_callable_arity(
    args: &Vec<Value>,
    arity: usize,
    name: &String,
) -> Result<(), LoxRuntimeException> {
    if args.len() != arity {
        Err(LoxRuntimeException::Error(format!(
            "Function {} expected {} arguments but got {}.",
            name,
            arity,
            args.len()
        )))
    } else {
        Ok(())
    }
}

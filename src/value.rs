use std::fmt::Display;

use crate::error::LoxRuntimeError;

pub enum LoxValue {
    LoxNil,
    LoxNumber(f64),
    LoxString(String),
    LoxBoolean(bool),
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::LoxNil => write!(f, "nil"),
            LoxValue::LoxNumber(n) if n.fract() == 0. => write!(f, "{}", n),
            LoxValue::LoxNumber(n) => write!(f, "{:.2}", n),
            LoxValue::LoxString(s) => write!(f, "\"{}\"", s),
            LoxValue::LoxBoolean(b) => write!(f, "{}", b),
        }
    }
}

impl LoxValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            LoxValue::LoxNil | LoxValue::LoxBoolean(false) => false,
            _ => true,
        }
    }

    pub fn as_number(&self) -> Result<f64, LoxRuntimeError> {
        match *self {
            LoxValue::LoxNumber(n) => Ok(n),
            _ => Err(LoxRuntimeError {
                message: format!("{} is not a number", self),
            }),
        }
    }

    pub fn is_equal(&self, other: &LoxValue) -> bool {
        match (self, other) {
            (LoxValue::LoxNil, LoxValue::LoxNil) => true,
            (LoxValue::LoxNumber(n1), LoxValue::LoxNumber(n2)) => n1 == n2,
            (LoxValue::LoxString(s1), LoxValue::LoxString(s2)) => s1 == s2,
            (LoxValue::LoxBoolean(b1), LoxValue::LoxBoolean(b2)) => b1 == b2,
            _ => false,
        }
    }
}

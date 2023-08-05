use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct LoxError {
    pub line: usize,
    pub message: String,
}

impl Error for LoxError {}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Line {}] Error: {}", self.line, self.message)
    }
}

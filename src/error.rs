use std::{error::Error, fmt::Display};

use crate::types::Line;

#[derive(Debug)]
pub struct LoxError {
    pub line: Line,
    pub message: String,
}

impl Error for LoxError {}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Line {}] Error: {}", self.line, self.message)
    }
}

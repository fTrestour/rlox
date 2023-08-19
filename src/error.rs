use std::{error::Error, fmt::Display};

use crate::types::Line;

#[derive(Debug, PartialEq)]
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

pub struct Report {
    errors: Vec<LoxError>,
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            writeln!(f, "{}", *error)?;
        }

        Ok(())
    }
}

impl Report {
    pub fn new() -> Report {
        Report { errors: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn push(&mut self, error: LoxError) {
        self.errors.push(error);
    }
}

pub struct LoxRuntimeError {
    pub message: String,
}
// TODO: Add line handling

impl Display for LoxRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime error: {}", self.message)
    }
}

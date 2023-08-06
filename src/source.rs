use std::{iter, str};

use crate::types::Line;

pub struct Source<'a> {
    iterable: iter::Peekable<str::Chars<'a>>,
    pending_lexeme: String,
    current_line: Line,
}

impl Source<'_> {
    pub fn new<'a>(s: &'a str) -> Source<'a> {
        Source {
            iterable: s.chars().peekable(),
            pending_lexeme: String::new(),
            current_line: 1,
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        let next = self.iterable.next();

        if let Some(c) = next {
            if c == '\n' {
                self.current_line += 1;
            };
            self.pending_lexeme.push(c);
        }

        next
    }

    pub fn maybe_next_char(&mut self, expected: char) -> Option<char> {
        let result = match self.iterable.peek() {
            Some(c) if *c == expected => Some(*c),
            _ => None,
        };

        if let Some(_) = result {
            self.next_char();
        };

        result
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.iterable.peek()
    }

    pub fn consume_until(&mut self, expected: char) {
        while self.peek_char() != Some(&expected) && self.peek_char() != None {
            self.next_char();
        }
    }

    pub fn consume_digits(&mut self) {
        while self.peek_char().map(|c| c.is_digit(10)) == Some(true) {
            self.next_char();
        }
    }

    pub fn consume_alphanumeric(&mut self) {
        while self.peek_char().map(|c| c.is_alphanumeric()) == Some(true) {
            self.next_char();
        }
    }

    pub fn flush_lexeme(&mut self) -> String {
        let lexeme = self.pending_lexeme.clone();
        self.pending_lexeme = String::new();

        lexeme
    }

    pub fn get_current_line(&self) -> Line {
        self.current_line
    }
}

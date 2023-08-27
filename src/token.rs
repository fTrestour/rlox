use std::{fmt, iter::Peekable, vec::IntoIter};

use crate::{error::LoxError, types::Line};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier(String),
    String(String),
    Number(f64),
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: Line,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} l.{}", self.token_type, self.lexeme, self.line)
    }
}

pub struct Tokens {
    peekable: Peekable<IntoIter<Token>>,
}

impl Tokens {
    pub fn new<'a>(v: Vec<Token>) -> Tokens {
        Tokens {
            peekable: v.into_iter().peekable(),
        }
    }

    pub fn peek(&mut self) -> Token {
        self.peekable
            .peek()
            .expect("Tokens should not be read after EOF")
            .clone()
    }

    pub fn peek_type(&mut self) -> TokenType {
        self.peek().token_type
    }

    pub fn next(&mut self) -> Token {
        self.peekable
            .next()
            .expect("Tokens should not be read after EOF")
    }

    pub fn consume(&mut self, token_type: TokenType) -> Result<(), LoxError> {
        if self.peek_type() == token_type {
            self.next();
            Ok(())
        } else {
            let token = self.peek();
            Err(LoxError {
                line: token.line,
                message: format!("Expected '{}', got '{}' instead", token_type, token.lexeme),
            })
        }
    }

    pub fn consume_until_semicolon(&mut self) -> Option<()> {
        self.peekable
            .by_ref()
            .skip_while(|token| token.token_type != TokenType::Semicolon)
            .next()
            .map(|_| ())
    }
}

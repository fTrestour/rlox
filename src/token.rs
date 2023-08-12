use std::{fmt, iter::Peekable, vec::IntoIter};

use crate::types::Line;

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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: Line,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} l.{}", self.token_type, self.lexeme, self.line)
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

    pub fn peek(&mut self) -> Option<Token> {
        let test = self.peekable.peek()?;
        let test = test.clone();
        Some(test)
    }

    pub fn peek_type(&mut self) -> Option<TokenType> {
        let test = self.peek()?;
        Some(test.token_type)
    }

    pub fn next(&mut self) -> Option<Token> {
        self.peekable.next()
    }
}

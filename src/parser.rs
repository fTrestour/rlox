use std::{iter::Peekable, slice::Iter};

use crate::{
    error::Report,
    grammar::Expression,
    token::{Token, TokenType},
};

pub fn parse(tokens: &mut Peekable<Iter<'_, Token>>, report: &mut Report) -> Option<Expression> {
    parse_expression(tokens)
}

fn parse_expression(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<Expression> {
    parse_equality(tokens)
}

fn parse_equality(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<Expression> {
    let left = parse_comparison(tokens)?;

    match tokens.peek()?.token_type {
        TokenType::BangEqual => {
            tokens.next();
            Some(Expression::NotEqual(
                Box::new(left),
                Box::new(parse_equality(tokens)?),
            ))
        }
        TokenType::EqualEqual => {
            tokens.next();
            let right = parse_equality(tokens)?;
            Some(Expression::Equal(Box::new(left), Box::new(right)))
        }
        _ => Some(left),
    }
}

fn parse_comparison(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<Expression> {
    let left = parse_term(tokens)?;

    match tokens.peek()?.token_type {
        TokenType::LessEqual => {
            tokens.next();
            Some(Expression::LessEqual(
                Box::new(left),
                Box::new(parse_comparison(tokens)?),
            ))
        }
        TokenType::Less => {
            tokens.next();
            Some(Expression::Less(
                Box::new(left),
                Box::new(parse_comparison(tokens)?),
            ))
        }
        TokenType::GreaterEqual => {
            tokens.next();
            Some(Expression::GreaterEqual(
                Box::new(left),
                Box::new(parse_comparison(tokens)?),
            ))
        }
        TokenType::Greater => {
            tokens.next();
            Some(Expression::Greater(
                Box::new(left),
                Box::new(parse_comparison(tokens)?),
            ))
        }
        _ => Some(left),
    }
}

fn parse_term(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<Expression> {
    let left = parse_factor(tokens)?;

    match tokens.peek()?.token_type {
        TokenType::Minus => {
            tokens.next();
            Some(Expression::Minus(
                Box::new(left),
                Box::new(parse_term(tokens)?),
            ))
        }
        TokenType::Plus => {
            tokens.next();
            Some(Expression::Plus(
                Box::new(left),
                Box::new(parse_term(tokens)?),
            ))
        }
        _ => Some(left),
    }
}

fn parse_factor(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<Expression> {
    let left = parse_unary(tokens)?;

    match tokens.peek()?.token_type {
        TokenType::Slash => {
            tokens.next();
            Some(Expression::Divide(
                Box::new(left),
                Box::new(parse_factor(tokens)?),
            ))
        }
        TokenType::Star => {
            tokens.next();
            Some(Expression::Multiply(
                Box::new(left),
                Box::new(parse_factor(tokens)?),
            ))
        }
        _ => Some(left),
    }
}

fn parse_unary(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<Expression> {
    match tokens.peek()?.token_type {
        TokenType::Bang => {
            tokens.next();
            Some(Expression::Not(Box::new(parse_unary(tokens)?)))
        }
        TokenType::Minus => {
            tokens.next();
            Some(Expression::Minus(
                Box::new(Expression::Number(0.into())),
                Box::new(parse_unary(tokens)?),
            ))
        }
        _ => parse_primary(tokens),
    }
}

fn parse_primary(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<Expression> {
    match &(tokens.peek()?).token_type {
        TokenType::Number(n) => {
            tokens.next();
            Some(Expression::Number(*n))
        }
        TokenType::String(s) => {
            tokens.next();
            Some(Expression::String(s.clone()))
        }
        TokenType::True => {
            tokens.next();
            Some(Expression::True)
        }
        TokenType::False => {
            tokens.next();
            Some(Expression::False)
        }
        TokenType::Nil => {
            tokens.next();
            Some(Expression::Nil)
        }
        TokenType::LeftParen => {
            tokens.next();
            let result = Expression::Paren(Box::new(parse_expression(tokens)?));
            if let TokenType::RightParen = tokens.next()?.token_type {
                Some(result)
            } else {
                None
            }
        }
        _ => None,
    }
}

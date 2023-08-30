use crate::{
    error::LoxError,
    grammar::Expression,
    token::{TokenType, Tokens},
};

pub fn parse_expression(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    parse_assignment(tokens)
}

fn parse_assignment(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    let left = parse_or(tokens)?;

    if tokens.consume(TokenType::Equal).is_ok() {
        if let Expression::Variable(name) = left {
            let assignment = parse_assignment(tokens)?;

            Ok(Expression::Assignment(name, Box::new(assignment)))
        } else {
            let token = tokens.peek();
            Err(LoxError {
                line: token.line,
                message: format!("Invalid assignment target."),
            })
        }
    } else {
        Ok(left)
    }
}

fn parse_or(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    let left = parse_and(tokens)?;

    if tokens.consume(TokenType::Or).is_ok() {
        let right = parse_or(tokens)?;

        Ok(Expression::Or(Box::new(left), Box::new(right)))
    } else {
        Ok(left)
    }
}

fn parse_and(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    let left = parse_equality(tokens)?;

    if tokens.consume(TokenType::And).is_ok() {
        let right = parse_and(tokens)?;

        Ok(Expression::And(Box::new(left), Box::new(right)))
    } else {
        Ok(left)
    }
}

fn parse_equality(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    let left = parse_comparison(tokens)?;

    match tokens.peek_type() {
        TokenType::BangEqual => {
            tokens.next();

            let right = parse_equality(tokens)?;
            Ok(Expression::NotEqual(Box::new(left), Box::new(right)))
        }
        TokenType::EqualEqual => {
            tokens.next();

            let right = parse_equality(tokens)?;
            Ok(Expression::Equal(Box::new(left), Box::new(right)))
        }
        _ => Ok(left),
    }
}

fn parse_comparison(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    let left = parse_term(tokens)?;

    match tokens.peek_type() {
        TokenType::LessEqual => {
            tokens.next();

            let right = parse_comparison(tokens)?;
            Ok(Expression::LessEqual(Box::new(left), Box::new(right)))
        }
        TokenType::Less => {
            tokens.next();

            let right = parse_comparison(tokens)?;
            Ok(Expression::Less(Box::new(left), Box::new(right)))
        }
        TokenType::GreaterEqual => {
            tokens.next();

            let right = parse_comparison(tokens)?;
            Ok(Expression::GreaterEqual(Box::new(left), Box::new(right)))
        }
        TokenType::Greater => {
            tokens.next();

            let right = parse_comparison(tokens)?;
            Ok(Expression::Greater(Box::new(left), Box::new(right)))
        }
        _ => Ok(left),
    }
}

fn parse_term(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    let left = parse_factor(tokens)?;

    match tokens.peek_type() {
        TokenType::Minus => {
            tokens.next();

            let right = parse_term(tokens)?;
            Ok(Expression::Minus(Box::new(left), Box::new(right)))
        }
        TokenType::Plus => {
            tokens.next();

            let right = parse_term(tokens)?;
            Ok(Expression::Plus(Box::new(left), Box::new(right)))
        }
        _ => Ok(left),
    }
}

fn parse_factor(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    let left = parse_unary(tokens)?;

    match tokens.peek_type() {
        TokenType::Slash => {
            tokens.next();

            let right = parse_factor(tokens)?;
            Ok(Expression::Divide(Box::new(left), Box::new(right)))
        }
        TokenType::Star => {
            tokens.next();

            let right = parse_factor(tokens)?;
            Ok(Expression::Multiply(Box::new(left), Box::new(right)))
        }
        _ => Ok(left),
    }
}

fn parse_unary(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    match tokens.peek_type() {
        TokenType::Bang => {
            tokens.next();

            let expression = parse_unary(tokens)?;
            Ok(Expression::Not(Box::new(expression)))
        }
        TokenType::Minus => {
            tokens.next();

            let expression = parse_unary(tokens)?;
            let zero = Expression::Number(0.);
            Ok(Expression::Minus(Box::new(zero), Box::new(expression)))
        }
        _ => parse_call(tokens),
    }
}

fn parse_call(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    let mut expression = parse_primary(tokens)?;

    while tokens.consume(TokenType::LeftParen).is_ok() {
        let arguments = match tokens.consume(TokenType::RightParen) {
            Ok(_) => Ok(vec![]),
            Err(_) => {
                let args = parse_args(tokens)?;
                tokens.consume(TokenType::RightParen)?;
                Ok(args)
            }
        }?;

        expression = Expression::Call(Box::new(expression), arguments);
    }

    Ok(expression)
}

fn parse_args(tokens: &mut Tokens) -> Result<Vec<Expression>, LoxError> {
    let expression = parse_expression(tokens)?;

    let mut args = vec![expression];
    while tokens.consume(TokenType::Comma).is_ok() {
        let expression = parse_expression(tokens)?;
        args.push(expression);
    }

    if args.len() >= 255 {
        let token = tokens.peek();
        // FIXME: This doesn't handle the error well, we should return the args anyway
        // We want to report the error, not throw it
        Err(LoxError {
            line: token.line,
            message: format!("Can't have more than 255 arguments."),
        })
    } else {
        Ok(args)
    }
}

fn parse_primary(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    match tokens.peek_type() {
        TokenType::Number(n) => {
            tokens.next();
            Ok(Expression::Number(n))
        }
        TokenType::String(s) => {
            tokens.next();
            Ok(Expression::String(s.clone()))
        }
        TokenType::True => {
            tokens.next();
            Ok(Expression::True)
        }
        TokenType::False => {
            tokens.next();
            Ok(Expression::False)
        }
        TokenType::Nil => {
            tokens.next();
            Ok(Expression::Nil)
        }
        TokenType::LeftParen => {
            tokens.next();

            let expression = parse_expression(tokens)?;
            tokens.consume(TokenType::RightParen)?;

            let expression = Expression::Paren(Box::new(expression));
            Ok(expression)
        }
        TokenType::Identifier(id) => {
            tokens.next();
            Ok(Expression::Variable(id))
        }
        _ => {
            let token = tokens.peek();
            Err(LoxError {
                line: token.line,
                message: format!("Expected expression, got {}", token.lexeme),
            })
        }
    }
}

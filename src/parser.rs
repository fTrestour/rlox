use crate::{
    error::{LoxError, Report},
    grammar::{Expression, Statement},
    token::{TokenType, Tokens},
};

pub fn parse(mut tokens: Tokens) -> Result<Vec<Statement>, Report> {
    let mut statements: Vec<Statement> = vec![];
    let mut report = Report::new();

    while let Some(statement) = parse_statement(&mut tokens, &mut report) {
        statements.push(statement);
    }

    if report.is_empty() {
        Ok(statements)
    } else {
        Err(report)
    }
}

fn parse_statement(tokens: &mut Tokens, report: &mut Report) -> Option<Statement> {
    let token_type = tokens.peek_type().expect("Invalid tokens list"); // TODO: Improve
    let statement = match token_type {
        TokenType::Eof => {
            tokens.next();
            None
        }
        TokenType::Print => {
            tokens.next();
            let expression = parse_expression(tokens, report)?;
            Some(Statement::Print(expression))
        }
        _ => {
            let expression = parse_expression(tokens, report)?;
            Some(Statement::Expression(expression))
        }
    }?;

    let next_token_type = tokens.peek_type()?; // TODO: Improve
    if let TokenType::Semicolon = next_token_type {
        tokens.next();
        Some(statement)
    } else {
        report.push(LoxError {
            line: 1000,
            message: "Expected ';'".to_owned(),
        }); // TODO: improve line handling
        None
    }
}

fn parse_expression(tokens: &mut Tokens, report: &mut Report) -> Option<Expression> {
    parse_equality(tokens, report)
}

fn parse_equality(tokens: &mut Tokens, report: &mut Report) -> Option<Expression> {
    let left = parse_comparison(tokens, report)?;

    match tokens.peek()?.token_type {
        TokenType::BangEqual => {
            tokens.next();
            Some(Expression::NotEqual(
                Box::new(left),
                Box::new(parse_equality(tokens, report)?),
            ))
        }
        TokenType::EqualEqual => {
            tokens.next();
            let right = parse_equality(tokens, report)?;
            Some(Expression::Equal(Box::new(left), Box::new(right)))
        }
        _ => Some(left),
    }
}

fn parse_comparison(tokens: &mut Tokens, report: &mut Report) -> Option<Expression> {
    let left = parse_term(tokens, report)?;

    match tokens.peek()?.token_type {
        TokenType::LessEqual => {
            tokens.next();
            Some(Expression::LessEqual(
                Box::new(left),
                Box::new(parse_comparison(tokens, report)?),
            ))
        }
        TokenType::Less => {
            tokens.next();
            Some(Expression::Less(
                Box::new(left),
                Box::new(parse_comparison(tokens, report)?),
            ))
        }
        TokenType::GreaterEqual => {
            tokens.next();
            Some(Expression::GreaterEqual(
                Box::new(left),
                Box::new(parse_comparison(tokens, report)?),
            ))
        }
        TokenType::Greater => {
            tokens.next();
            Some(Expression::Greater(
                Box::new(left),
                Box::new(parse_comparison(tokens, report)?),
            ))
        }
        _ => Some(left),
    }
}

fn parse_term(tokens: &mut Tokens, report: &mut Report) -> Option<Expression> {
    let left = parse_factor(tokens, report)?;

    match tokens.peek()?.token_type {
        TokenType::Minus => {
            tokens.next();
            Some(Expression::Minus(
                Box::new(left),
                Box::new(parse_term(tokens, report)?),
            ))
        }
        TokenType::Plus => {
            tokens.next();
            Some(Expression::Plus(
                Box::new(left),
                Box::new(parse_term(tokens, report)?),
            ))
        }
        _ => Some(left),
    }
}

fn parse_factor(tokens: &mut Tokens, report: &mut Report) -> Option<Expression> {
    let left = parse_unary(tokens, report)?;

    match tokens.peek_type()? {
        TokenType::Slash => {
            tokens.next();
            Some(Expression::Divide(
                Box::new(left),
                Box::new(parse_factor(tokens, report)?),
            ))
        }
        TokenType::Star => {
            tokens.next();
            Some(Expression::Multiply(
                Box::new(left),
                Box::new(parse_factor(tokens, report)?),
            ))
        }
        _ => Some(left),
    }
}

fn parse_unary(tokens: &mut Tokens, report: &mut Report) -> Option<Expression> {
    match tokens.peek_type()? {
        TokenType::Bang => {
            tokens.next();
            Some(Expression::Not(Box::new(parse_unary(tokens, report)?)))
        }
        TokenType::Minus => {
            tokens.next();
            Some(Expression::Minus(
                Box::new(Expression::Number(0.into())),
                Box::new(parse_unary(tokens, report)?),
            ))
        }
        _ => parse_primary(tokens, report),
    }
}

fn parse_primary(tokens: &mut Tokens, report: &mut Report) -> Option<Expression> {
    match tokens.peek_type()? {
        TokenType::Number(n) => {
            tokens.next();
            Some(Expression::Number(n))
        }
        TokenType::String(s) => {
            let s = s.clone();
            tokens.next();
            Some(Expression::String(s))
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
            tokens.next()?;
            let result = Expression::Paren(Box::new(parse_expression(tokens, report)?));

            let token = tokens.next()?;
            if let TokenType::RightParen = token.token_type {
                Some(result)
            } else {
                report.push(LoxError {
                    line: token.line,
                    message: format!("Expected ')' got '{}' instead", token.lexeme),
                });
                None
            }
        }
        _ => {
            let token = tokens.next()?;
            report.push(LoxError {
                line: token.line,
                message: "Expected expression".to_owned(),
            });
            None
        }
    }
}

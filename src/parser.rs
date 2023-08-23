use crate::{
    error::{LoxError, Report},
    grammar::{Declaration, Expression},
    token::{TokenType, Tokens},
};

pub fn parse(mut tokens: Tokens) -> Result<Vec<Declaration>, Report> {
    let mut declarations: Vec<Declaration> = vec![];
    let mut report = Report::new();

    let mut declaration = parse_declaration(&mut tokens);
    while declaration != Ok(None) {
        if let Err(error) = declaration {
            report.push(error);

            if tokens.consume_until_semicolon() == None {
                break;
            }
        } else {
            let declaration = declaration.expect("Err should be caught before the current case");
            let declaration = declaration.expect("None should not get in this loop");

            declarations.push(declaration);
        }

        declaration = parse_declaration(&mut tokens);
    }

    if report.is_empty() {
        Ok(declarations)
    } else {
        Err(report)
    }
}

fn parse_declaration(tokens: &mut Tokens) -> Result<Option<Declaration>, LoxError> {
    match tokens.peek_type() {
        TokenType::Eof => {
            tokens.next();

            Ok(None)
        }
        TokenType::Var => {
            tokens.next();

            match tokens.peek_type() {
                TokenType::Identifier(id) => {
                    tokens.next();

                    let declaration = if tokens.peek_type() == TokenType::Equal {
                        tokens.next();

                        let expression = parse_expression(tokens)?;
                        Declaration::Var(id, Some(expression))
                    } else {
                        Declaration::Var(id, None)
                    };
                    tokens.consume_semicolon()?;

                    Ok(Some(declaration))
                }
                _ => {
                    let token = tokens.peek();
                    Err(LoxError {
                        line: token.line,
                        message: format!("Expected identifier, got {} instead.", token.lexeme),
                    })
                }
            }
        }
        _ => parse_statement(tokens).map(|statement| Some(statement)),
    }
}

fn parse_statement(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    let statement = match tokens.peek_type() {
        TokenType::Print => {
            tokens.next();

            let expression = parse_expression(tokens)?;
            tokens.consume_semicolon()?;
            Ok(Declaration::Print(expression))
        }
        TokenType::LeftBrace => {
            tokens.next();

            let mut declarations = vec![];
            while tokens.peek_type() != TokenType::RightBrace
                && tokens.peek_type() != TokenType::Eof
            {
                let declaration = parse_declaration(tokens)?; // FIXME: group errors together and parse all statements
                declaration.map(|declaration| declarations.push(declaration));
            }

            if tokens.peek_type() == TokenType::RightBrace {
                tokens.next();

                Ok(Declaration::Block(declarations))
            } else {
                let token = tokens.peek();
                Err(LoxError {
                    line: token.line,
                    message: "Expect '}' after block.".to_owned(),
                })
            }
        }
        _ => {
            let expression = parse_expression(tokens)?;
            tokens.consume_semicolon()?;
            Ok(Declaration::Expression(expression))
        }
    }?;

    Ok(statement)
}

fn parse_expression(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    parse_assignment(tokens)
}

fn parse_assignment(tokens: &mut Tokens) -> Result<Expression, LoxError> {
    let left = parse_equality(tokens)?;

    if tokens.peek_type() == TokenType::Equal {
        tokens.next();

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
        _ => parse_primary(tokens),
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

            if tokens.peek_type() == TokenType::RightParen {
                tokens.next();

                let expression = Expression::Paren(Box::new(expression));
                Ok(expression)
            } else {
                let token = tokens.peek();
                Err(LoxError {
                    line: token.line,
                    message: format!("Expected ')' got '{}' instead", token.lexeme),
                })
            }
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

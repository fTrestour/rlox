use crate::grammar::Expression;
use crate::parser::expression::parse_expression;

use crate::{
    error::LoxError,
    grammar::Declaration,
    token::{TokenType, Tokens},
};

pub fn parse_declaration(tokens: &mut Tokens) -> Result<Option<Declaration>, LoxError> {
    match tokens.peek_type() {
        TokenType::Eof => {
            tokens.next();

            Ok(None)
        }
        TokenType::Var => Some(parse_var_declaration(tokens)).transpose(),
        TokenType::Fun => Some(parse_fun_declaration(tokens)).transpose(),
        _ => Some(parse_statement(tokens)).transpose(),
    }
}

fn parse_fun_declaration(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    tokens.consume(TokenType::Fun)?;
    parse_function(tokens)
}

fn parse_function(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    if let TokenType::Identifier(function_name) = tokens.peek_type() {
        tokens.next();

        tokens.consume(TokenType::LeftParen)?;
        let parameters = parse_parameters(tokens).or(Ok(vec![]))?;
        tokens.consume(TokenType::RightParen)?;
        let body = parse_block(tokens)?;

        Ok(Declaration::Function(
            function_name,
            parameters,
            Box::new(body),
        ))
    } else {
        let token = tokens.peek();
        Err(LoxError {
            line: token.line,
            message: format!("Expected identifier, got {}.", token.lexeme),
        })
    }
}

fn parse_parameters(tokens: &mut Tokens) -> Result<Vec<String>, LoxError> {
    if let TokenType::Identifier(parameter) = tokens.peek_type() {
        tokens.next();

        let mut parameters = vec![parameter];
        while tokens.peek_type() != TokenType::RightParen && tokens.peek_type() != TokenType::Eof {
            tokens.consume(TokenType::Comma)?;

            if let TokenType::Identifier(parameter) = tokens.peek_type() {
                tokens.next();
                parameters.push(parameter);
            } else {
                let token = tokens.peek();
                return Err(LoxError {
                    line: token.line,
                    message: format!("Expected identifier, got {}.", token.lexeme),
                });
            }
        }

        Ok(parameters)
    } else {
        let token = tokens.peek();
        Err(LoxError {
            line: token.line,
            message: format!("Expected identifier, got {}.", token.lexeme),
        })
    }
}

fn parse_var_declaration(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    tokens.consume(TokenType::Var)?;

    if let TokenType::Identifier(id) = tokens.peek_type() {
        tokens.next();

        let declaration = if tokens.consume(TokenType::Equal).is_ok() {
            let expression = parse_expression(tokens)?;
            Declaration::Var(id, Some(expression))
        } else {
            Declaration::Var(id, None)
        };

        tokens.consume(TokenType::Semicolon)?;

        Ok(declaration)
    } else {
        let token = tokens.peek();
        Err(LoxError {
            line: token.line,
            message: format!("Expected identifier, got {} instead.", token.lexeme),
        })
    }
}

fn parse_statement(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    match tokens.peek_type() {
        TokenType::Print => parse_print(tokens),
        TokenType::LeftBrace => parse_block(tokens),
        TokenType::If => parse_if(tokens),
        TokenType::While => parse_while(tokens),
        TokenType::For => parse_for(tokens),
        TokenType::Return => parse_return(tokens),
        _ => parse_expression_statement(tokens),
    }
}

fn parse_return(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    tokens.consume(TokenType::Return)?;

    let expression = match tokens.consume(TokenType::Semicolon) {
        Ok(_) => Expression::Nil,
        Err(_) => {
            let expression = parse_expression(tokens)?;
            tokens.consume(TokenType::Semicolon)?;

            expression
        }
    };

    Ok(Declaration::Return(expression))
}

fn parse_for(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    tokens.consume(TokenType::For)?;

    tokens.consume(TokenType::LeftParen)?;

    let initializer = match tokens.peek_type() {
        TokenType::Semicolon => {
            tokens.next();
            None
        }
        TokenType::Var => Some(parse_var_declaration(tokens)?),
        _ => Some(parse_expression_statement(tokens)?),
    };

    let condition = if tokens.consume(TokenType::Semicolon).is_ok() {
        None
    } else {
        let expression = parse_expression(tokens)?;
        tokens.consume(TokenType::Semicolon)?;

        Some(expression)
    };

    let increment = if tokens.consume(TokenType::Semicolon).is_ok() {
        None
    } else {
        Some(parse_expression(tokens)?)
    };
    tokens.consume(TokenType::RightParen)?;

    let mut body = parse_statement(tokens)?;

    if let Some(increment) = increment {
        body = Declaration::Block(vec![body, Declaration::Expression(increment)]);
    }

    if let Some(condition) = condition {
        body = Declaration::While(condition, Box::new(body));
    } else {
        body = Declaration::While(Expression::True, Box::new(body));
    }

    if let Some(initializer) = initializer {
        body = Declaration::Block(vec![initializer, body])
    }

    Ok(body)
}

fn parse_expression_statement(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    let expression = parse_expression(tokens)?;
    tokens.consume(TokenType::Semicolon)?;

    Ok(Declaration::Expression(expression))
}

fn parse_while(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    tokens.consume(TokenType::While)?;

    tokens.consume(TokenType::LeftParen)?;
    let condition = parse_expression(tokens)?;
    tokens.consume(TokenType::RightParen)?;

    let while_statement = parse_statement(tokens)?;

    Ok(Declaration::While(condition, Box::new(while_statement)))
}

fn parse_if(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    tokens.consume(TokenType::If)?;

    tokens.consume(TokenType::LeftParen)?;
    let condition = parse_expression(tokens)?;
    tokens.consume(TokenType::RightParen)?;

    let if_statement = parse_statement(tokens)?;
    if tokens.consume(TokenType::Else).is_ok() {
        let else_statement = parse_statement(tokens)?;

        Ok(Declaration::If(
            condition,
            Box::new(if_statement),
            Some(Box::new(else_statement)),
        ))
    } else {
        Ok(Declaration::If(condition, Box::new(if_statement), None))
    }
}

fn parse_print(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    tokens.consume(TokenType::Print)?;
    let expression = parse_expression(tokens)?;
    tokens.consume(TokenType::Semicolon)?;

    Ok(Declaration::Print(expression))
}

fn parse_block(tokens: &mut Tokens) -> Result<Declaration, LoxError> {
    tokens.consume(TokenType::LeftBrace)?;

    let mut declarations = vec![];
    while tokens.peek_type() != TokenType::RightBrace && tokens.peek_type() != TokenType::Eof {
        let declaration = parse_declaration(tokens)?; // FIXME: group errors together and parse all statements
        declaration.map(|declaration| declarations.push(declaration));
    }
    tokens.consume(TokenType::RightBrace)?;

    Ok(Declaration::Block(declarations))
}

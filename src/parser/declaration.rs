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
        _ => Some(parse_statement(tokens)).transpose(),
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
        _ => parse_expression_statement(tokens),
    }
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

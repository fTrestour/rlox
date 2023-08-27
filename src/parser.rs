mod declaration;
mod expression;

use self::declaration::parse_declaration;
use crate::{error::Report, grammar::Declaration, token::Tokens};

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

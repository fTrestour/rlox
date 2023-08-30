mod environment;
mod error;
mod grammar;
mod interpreter;
mod parser;
mod scanner;
mod source;
mod token;
mod types;
mod value;

use anyhow::{Context, Result};
use environment::Environment;
use interpreter::interpret;
use parser::parse;
use scanner::scan;
use source::Source;
use std::{
    fs,
    io::{self, Write},
};
use token::Tokens;

pub fn run_file(filename: &str) -> Result<()> {
    let file =
        fs::read_to_string(&filename).context(format!("Failed reading file {}", filename))?;
    let mut environment = Environment::new_global();
    run(&file, &environment).context("Failed running lox code")
}

pub fn run_prompt() -> Result<()> {
    let mut environment = Environment::new_global();

    loop {
        let line = invite()?;

        if !line.is_empty() {
            run(&line, &environment);
        }
    }
}

fn invite() -> Result<String> {
    print!("> ");
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?; // FIXME: Pasting many lines breaks this

    Ok(line.trim().to_owned())
}

// TODO: return a result to differentiate static vs runtime errors
fn run(source: &str, environment: &Environment) -> Option<()> {
    let source = Source::new(source);
    let tokens = scan(source);

    let tokens = tokens.map(Tokens::new);
    let statements = tokens.and_then(parse);

    match statements {
        Ok(statements) => {
            for statement in statements {
                if let Err(error) = interpret(statement, environment) {
                    println!("{}", error);
                    return None;
                }
            }
            Some(())
        }
        Err(report) => {
            print!("{}", report);
            None
        }
    }
}

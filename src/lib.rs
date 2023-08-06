mod error;
mod scanner;
mod token;
mod types;

use anyhow::Result;
use error::LoxError;
use scanner::scan_tokens;
use std::{fs, io};

pub fn run_file(filename: &str) -> Result<()> {
    let file = fs::read_to_string(&filename)?;
    Ok(run(&file)?)
}

pub fn run_prompt() -> Result<()> {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        let line = line.trim();

        if !line.is_empty() {
            run(line);
        }
    }
}

fn run(source: &str) -> Result<(), LoxError> {
    let result = scan_tokens(source);

    match result {
        Ok(tokens) => {
            println!("{:?}", tokens);
            Ok(())
        }
        Err(error) => {
            println!("{}", error);
            Err(error)
        }
    }
}

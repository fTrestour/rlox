mod error;
mod scanner;
mod token;
mod types;

use anyhow::{Context, Result};
use scanner::scan_tokens;
use std::{
    fs,
    io::{self},
};

pub fn run_file(filename: &str) -> Result<()> {
    let file =
        fs::read_to_string(&filename).context(format!("Failed reading file {}", filename))?;
    run(&file).context("Failed running lox code")
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

fn run(source: &str) -> Option<()> {
    let result = scan_tokens(source);

    match result {
        Ok(tokens) => {
            println!("{:?}", tokens);
            Some(())
        }
        Err(report) => {
            println!("{}", report);
            None
        }
    }
}

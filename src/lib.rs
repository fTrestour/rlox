use anyhow::Result;
use std::{fs, io};

pub fn run_file(filename: &str) -> Result<()> {
    let file = fs::read_to_string(&filename)?;
    run(&file)
}

pub fn run_prompt() -> Result<()> {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        let line = line.trim();

        if !line.is_empty() {
            run(line)?;
        }
    }
}

fn run(code: &str) -> Result<()> {
    print!("Running code: {}", code);
    todo!()
}

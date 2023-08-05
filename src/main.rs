use anyhow::Result;
use std::{env, process};

use rlox::{run_file, run_prompt};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        run_file(&args[1]).or_else(|_| process::exit(65))
    } else if args.len() == 1 {
        run_prompt()
    } else {
        println!("Usage: rlox [script]");
        process::exit(64);
    }
}

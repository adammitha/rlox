use std::{env, io, process};

use rlox::Lox;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        lox.run_file(&args[1])?
    } else {
        lox.run_prompt()?;
    }
    Ok(())
}
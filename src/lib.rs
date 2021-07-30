mod error;
mod scanner;
use error::Error;
use scanner::Scanner;
use std::{
    cell::RefCell,
    fs,
    io::{self, Write},
    process,
    rc::Rc,
};

pub struct Lox {
    error_handler: Rc<RefCell<Error>>,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            error_handler: Rc::new(RefCell::new(Error { had_error: false })),
        }
    }
    pub fn run_file(&self, path: &str) -> io::Result<()> {
        let source = fs::read_to_string(path)?;
        self.run(&source);
        if self.error_handler.borrow().had_error {
            process::exit(65);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> io::Result<usize> {
        println!("Running rlox prompt");
        let mut buf = String::new();
        loop {
            print!("> ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut buf)?;
            self.run(&buf);
            buf.clear();
            self.error_handler.borrow_mut().had_error = false;
        }
    }

    pub fn run(&self, source: &str) {
        let mut scanner = Scanner::new(source, self.error_handler.clone());
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }
}

mod error;
pub mod interpreter;
mod parser;
mod scanner;
use error::SimpleErrorHandler;
use interpreter::environment::Environment;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use std::{
    cell::RefCell,
    fs,
    io::{self, Write},
    process,
    rc::Rc,
};

pub struct Lox {
    error_handler: SimpleErrorHandler,
    environment: Rc<RefCell<Environment>>,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            error_handler: SimpleErrorHandler::new(),
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }
    pub fn run_file(&mut self, path: &str) -> io::Result<()> {
        let source = fs::read_to_string(path)?;
        self.run(&source);
        if self.error_handler.had_error {
            process::exit(65);
        }
        if self.error_handler.had_runtime_error {
            process::exit(70);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        println!("Running rlox prompt");
        let mut buf = String::new();
        loop {
            print!("> ");
            io::stdout().flush()?;
            let n = io::stdin().read_line(&mut buf)?;
            if n == 0 {
                process::exit(64);
            }
            self.run(&buf);
            buf.clear();
            self.error_handler.had_error = false;
        }
    }

    pub fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source, &mut self.error_handler);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens, &mut self.error_handler);
        let statements = parser.parse();
        if self.error_handler.had_error {
            return;
        };
        let mut interpreter = Interpreter::new(&mut self.error_handler, self.environment.clone());
        interpreter.interpret(statements);
    }
}

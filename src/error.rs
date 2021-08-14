use crate::interpreter::error::InterpreterError;
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

pub struct SimpleErrorHandler {
    pub had_error: bool,
    pub had_runtime_error: bool,
}

impl SimpleErrorHandler {
    pub fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    pub fn parser_error(&mut self, token: &Token, msg: &str) {
        match token.token_type {
            TokenType::EOF => self.report(token.line, " at end", msg),
            _ => self.report(token.line, &format!(" at '{}'", token.lexeme), msg),
        }
    }

    pub fn runtime_error(&mut self, error: InterpreterError) {
        eprintln!("{}", error.message);
        eprintln!("[line {}]", error.token.line);
        self.had_runtime_error = true;
    }

    fn report(&mut self, line: u32, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }
}

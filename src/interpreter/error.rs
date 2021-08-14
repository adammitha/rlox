use crate::scanner::token::Token;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message}")]
pub struct InterpreterError {
    token: Token,
    message: String,
}

impl InterpreterError {
    pub fn new(token: Token, message: &str) -> Self {
        Self {
            token,
            message: String::from(message),
        }
    }
}

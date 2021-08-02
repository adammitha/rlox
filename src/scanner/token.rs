use std::fmt::{Debug, Display};

use super::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    literal: Option<Literal>,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("token_type", &self.token_type)
            .field("lexeme", &self.lexeme)
            .field(
                "literal",
                &match &self.literal {
                    Some(literal) => literal.to_string(),
                    None => String::from(""),
                },
            )
            .field("line", &self.line)
            .finish()
    }
}

#[derive(Clone)]
pub enum Literal {
    Number(f64),
    String(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

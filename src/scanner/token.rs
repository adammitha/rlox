use std::fmt::Debug;
use std::fmt::Display;

use super::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    pub lexeme: String,
    literal: Option<Box<dyn Display>>,
    line: u32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Box<dyn Display>>,
        line: u32,
    ) -> Self {
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

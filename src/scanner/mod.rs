pub mod token;
pub mod token_type;
use super::SimpleErrorHandler;
use std::mem;
use token::Literal;
use token::Token;
use token_type::TokenType;

pub struct Scanner<'a> {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
    error_handler: &'a mut SimpleErrorHandler,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str, error_handler: &'a mut SimpleErrorHandler) -> Self {
        Self {
            source: String::from(source).chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_handler,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            None,
            self.line,
        ));
        return mem::take(&mut self.tokens);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        let token_type = match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            '!' => {
                if self.match_char(&'=') {
                    Some(TokenType::BangEqual)
                } else {
                    Some(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char(&'=') {
                    Some(TokenType::EqualEqual)
                } else {
                    Some(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char(&'=') {
                    Some(TokenType::LessEqual)
                } else {
                    Some(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char(&'=') {
                    Some(TokenType::GreaterEqual)
                } else {
                    Some(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_char(&'/') {
                    while self.peek() != &'\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None
                } else {
                    Some(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            '"' => {
                // String literal - returns None as it uses the add_token_with_literal method
                while self.peek() != &'"' && !self.is_at_end() {
                    if self.peek() == &'\n' {
                        self.line += 1
                    };
                    self.advance();
                }

                if self.is_at_end() {
                    self.error_handler.error(self.line, "Unterminated string.");
                } else {
                    self.advance();
                    let value: String = self.source
                        [(self.start + 1) as usize..(self.current - 1) as usize]
                        .into_iter()
                        .collect();
                    self.add_token_with_literal(TokenType::String, Some(Literal::String(value)));
                };
                None
            }
            c if c.is_digit(10) => {
                // Numeric literal
                while self.peek().is_digit(10) {
                    self.advance();
                }

                // Look for a fractional part
                if self.peek() == &'.' && self.peek_next().is_digit(10) {
                    self.advance();
                    while self.peek().is_digit(10) {
                        self.advance();
                    }
                };
                let num_str: String = self.source[self.start as usize..self.current as usize]
                    .into_iter()
                    .collect();
                let num: f64 = num_str.parse().unwrap();
                self.add_token_with_literal(TokenType::Number, Some(Literal::Number(num)));
                None
            }
            c if c.is_alphabetic() || c == &'_' => {
                while self.peek().is_alphabetic() || self.peek().is_digit(10) {
                    self.advance();
                }
                let text: String = self.source[self.start as usize..self.current as usize]
                    .into_iter()
                    .collect();
                match Scanner::keywords(&text) {
                    Some(t) => Some(t),
                    None => Some(TokenType::Identifier),
                }
            }
            _ => {
                self.error_handler.error(self.line, "Unexpected character");
                None
            }
        };
        match token_type {
            Some(t) => self.add_token(t),
            None => (),
        };
    }

    fn peek(&self) -> &char {
        if self.is_at_end() {
            return &'\0';
        };
        &self.source[self.current as usize]
    }

    fn peek_next(&self) -> &char {
        if self.current + 1 >= self.source.len() as u32 {
            return &'\0';
        };
        return &self.source[(self.current + 1) as usize];
    }

    fn match_char(&mut self, expected: &char) -> bool {
        if self.is_at_end() {
            return false;
        };
        if &self.source[self.current as usize] != expected {
            return false;
        };

        self.current += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = self.source[self.start as usize..self.current as usize]
            .to_owned()
            .into_iter()
            .collect();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn advance(&mut self) -> &char {
        let c = &self.source[self.current as usize];
        self.current += 1;
        c
    }

    fn keywords(kw: &str) -> Option<TokenType> {
        match kw {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}

pub mod ast_printer;
pub mod expr;
pub mod stmt;
use crate::error::SimpleErrorHandler;

use super::scanner::token_type::TokenType;
use thiserror::Error;

use super::scanner::token::Literal;
use super::scanner::token::Token;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: u32,
    error_handler: &'a mut SimpleErrorHandler,
}

#[derive(Error, Debug)]
#[error("Parser error!")]
struct ParserError {}

type Result<T> = std::result::Result<T, ParserError>;

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, error_handler: &'a mut SimpleErrorHandler) -> Self {
        Self {
            tokens,
            current: 0,
            error_handler,
        }
    }

    pub fn parse(&mut self) -> Option<Box<expr::Expr>> {
        match self.expression() {
            Ok(expr) => Some(Box::new(expr)),
            Err(_) => None,
        }
    }

    fn expression(&mut self) -> Result<expr::Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<expr::Expr> {
        let mut expr = self.comparison()?;
        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = expr::Expr::Binary(expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<expr::Expr> {
        let mut expr = self.term()?;
        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = expr::Expr::Binary(expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<expr::Expr> {
        let mut expr = self.factor()?;

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = expr::Expr::Binary(expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<expr::Expr> {
        let mut expr = self.unary()?;

        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = expr::Expr::Binary(expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<expr::Expr> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(expr::Expr::Unary(expr::Unary {
                operator,
                right: Box::new(right),
            }));
        };
        self.primary()
    }

    fn primary(&mut self) -> Result<expr::Expr> {
        if self.match_token(vec![TokenType::False]) {
            return Ok(expr::Expr::Literal(expr::Literal {
                value: Box::new(Literal::False),
            }));
        } else if self.match_token(vec![TokenType::True]) {
            return Ok(expr::Expr::Literal(expr::Literal {
                value: Box::new(Literal::True),
            }));
        } else if self.match_token(vec![TokenType::Nil]) {
            return Ok(expr::Expr::Literal(expr::Literal {
                value: Box::new(Literal::Nil),
            }));
        } else if self.match_token(vec![TokenType::Number, TokenType::String]) {
            return Ok(expr::Expr::Literal(expr::Literal {
                value: self.previous().literal.unwrap(),
            }));
        } else if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(expr::Expr::Grouping(expr::Grouping {
                expression: Box::new(expr),
            }));
        }
        let token = self.peek().clone();
        Err(self.error(&token, "Expect expression"))
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        };
        let token = self.peek().clone();
        Err(self.error(&token, msg))
    }

    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        };
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current as usize]
    }

    fn previous(&mut self) -> Token {
        self.tokens[(self.current - 1) as usize].clone()
    }

    fn error(&mut self, token: &Token, msg: &str) -> ParserError {
        self.error_handler.parser_error(token, msg);
        ParserError {}
    }

    fn _synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            };
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }
    }
}

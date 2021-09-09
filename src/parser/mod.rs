pub mod expr;
pub mod stmt;
use crate::error::SimpleErrorHandler;
use crate::scanner::token;

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

    pub fn parse(&mut self) -> Vec<stmt::Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            let statement = match self.declaration() {
                Ok(stmt) => stmt,
                Err(_) => {
                    self.synchronize();
                    continue;
                }
            };
            statements.push(statement);
        }
        statements
    }

    fn declaration(&mut self) -> Result<stmt::Stmt> {
        if self.match_token(vec![TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Result<stmt::Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let mut initializer = expr::Expr::Literal(expr::Literal {
            value: Box::new(Literal::Nil),
        });
        if self.match_token(vec![TokenType::Equal]) {
            initializer = self.expression()?;
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration",
        )?;
        Ok(stmt::Stmt::Var(stmt::Var {
            name,
            initializer: Box::new(initializer),
        }))
    }

    fn statement(&mut self) -> Result<stmt::Stmt> {
        if self.match_token(vec![TokenType::For]) {
            return self.for_statement();
        } else if self.match_token(vec![TokenType::If]) {
            return self.if_statement();
        } else if self.match_token(vec![TokenType::Print]) {
            return self.print_statement();
        } else if self.match_token(vec![TokenType::While]) {
            return self.while_statement();
        } else if self.match_token(vec![TokenType::LeftBrace]) {
            return Ok(stmt::Stmt::Block(stmt::Block {
                statements: self.block()?,
            }));
        };
        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<stmt::Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer;
        if self.match_token(vec![TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_token(vec![TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = stmt::Stmt::Block(stmt::Block {
                statements: vec![
                    body,
                    stmt::Stmt::Expression(stmt::Expression {
                        expression: Box::new(inc),
                    }),
                ],
            })
        }

        if let None = condition {
            condition = Some(expr::Expr::Literal(expr::Literal {
                value: Box::new(token::Literal::True),
            }));
        }
        body = stmt::Stmt::While(stmt::While {
            condition: Box::new(condition.unwrap()),
            body: Box::new(body),
        });

        if let Some(init) = initializer {
            body = stmt::Stmt::Block(stmt::Block {
                statements: vec![init, body],
            })
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<stmt::Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch = None;
        if self.match_token(vec![TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(stmt::Stmt::If(stmt::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        }))
    }

    fn print_statement(&mut self) -> Result<stmt::Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(stmt::Stmt::Print(stmt::Print {
            expression: Box::new(value),
        }))
    }

    fn while_statement(&mut self) -> Result<stmt::Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;

        Ok(stmt::Stmt::While(stmt::While {
            condition: Box::new(condition),
            body: Box::new(body),
        }))
    }

    fn expression_statement(&mut self) -> Result<stmt::Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(stmt::Stmt::Expression(stmt::Expression {
            expression: Box::new(expr),
        }))
    }

    fn block(&mut self) -> Result<Vec<stmt::Stmt>> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<expr::Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<expr::Expr> {
        let expr = self.or()?;

        if self.match_token(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            return match expr {
                expr::Expr::Variable(variable) => {
                    let name = variable.name;
                    Ok(expr::Expr::Assign(expr::Assign {
                        name,
                        value: Box::new(value),
                    }))
                }
                _ => Err(self.error(&equals, "Invalid assignment target.")),
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<expr::Expr> {
        let mut expr = self.and()?;

        while self.match_token(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = expr::Expr::Logical(expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<expr::Expr> {
        let mut expr = self.equality()?;

        while self.match_token(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = expr::Expr::Logical(expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
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
        } else if self.match_token(vec![TokenType::Identifier]) {
            return Ok(expr::Expr::Variable(expr::Variable {
                name: self.previous(),
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

    fn synchronize(&mut self) {
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

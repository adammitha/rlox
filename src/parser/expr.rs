use super::super::scanner::token;
use super::super::scanner::token::Token;
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Literal {
    pub value: token::Literal,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

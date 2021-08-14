use crate::{parser::expr::Expr, scanner::token::Token};
pub struct Expression {
    pub expression: Box<Expr>,
}

pub struct Print {
    pub expression: Box<Expr>,
}

pub struct Var {
    pub name: Token,
    pub initializer: Box<Expr>,
}

pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
}

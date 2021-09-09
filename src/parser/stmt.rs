use crate::{parser::expr::Expr, scanner::token::Token};

pub struct Block {
    pub statements: Vec<Stmt>,
}

pub struct Expression {
    pub expression: Box<Expr>,
}

pub struct If {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct Print {
    pub expression: Box<Expr>,
}

pub struct Var {
    pub name: Token,
    pub initializer: Box<Expr>,
}

pub struct While {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

pub enum Stmt {
    Block(Block),
    Expression(Expression),
    If(If),
    Print(Print),
    Var(Var),
    While(While),
}

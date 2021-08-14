use crate::parser::expr::Expr;
pub struct Expression {
    pub expression: Expr,
}

pub struct Print {
    pub expression: Expr,
}

pub enum Stmt {
    Expression(Expression),
    Print(Print),
}

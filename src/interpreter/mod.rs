use crate::parser::expr;
use crate::scanner::token::Literal;
use crate::scanner::token_type::TokenType;
mod error;
use error::InterpreterError;

pub struct Interpreter {}

#[derive(PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Interpreter {
    pub fn visit_literal_expr(&self, expr: &expr::Literal) -> Result<Value> {
        Ok(match &*expr.value {
            Literal::Number(num) => Value::Number(*num),
            Literal::String(string) => Value::String(string.clone()),
            Literal::True => Value::Boolean(true),
            Literal::False => Value::Boolean(false),
            Literal::Nil => Value::Nil,
        })
    }

    pub fn visit_grouping_expr(&self, expr: &expr::Grouping) -> Result<Value> {
        self.evaluate(&expr.expression)
    }

    pub fn visit_unary_expr(&self, expr: &expr::Unary) -> Result<Value> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Bang => Ok(Value::Boolean(!self.is_truthy(right))),
            TokenType::Minus => {
                if let Value::Number(num) = right {
                    Ok(Value::Number(-num))
                } else {
                    Err(InterpreterError::new(
                        expr.operator.clone(),
                        "Operand must be a number",
                    ))
                }
            }
            _ => Err(InterpreterError::new(
                expr.operator.clone(),
                "Operator in a unary expression must be a '!' or '-'",
            )),
        }
    }

    fn is_truthy(&self, val: Value) -> bool {
        match val {
            Value::Nil => false,
            Value::Boolean(boolean) => boolean,
            _ => true,
        }
    }

    fn visit_binary_expr(&self, expr: &expr::Binary) -> Result<Value> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        if let (Value::Number(left_num), Value::Number(right_num)) = (&left, &right) {
            match expr.operator.token_type {
                TokenType::Greater => Ok(Value::Boolean(left_num > right_num)),
                TokenType::GreaterEqual => Ok(Value::Boolean(left_num >= right_num)),
                TokenType::Less => Ok(Value::Boolean(left_num < right_num)),
                TokenType::LessEqual => Ok(Value::Boolean(left_num <= right_num)),
                TokenType::Minus => Ok(Value::Number(left_num - right_num)),
                TokenType::Plus => Ok(Value::Number(left_num + right_num)),
                TokenType::Slash => Ok(Value::Number(left_num / right_num)),
                TokenType::Star => Ok(Value::Number(left_num * right_num)),
                _ => Err(InterpreterError::new(
                    expr.operator.clone(),
                    "Invalid infix operator for two numbers",
                )),
            }
        } else if let (Value::String(left_str), Value::String(right_str)) = (&left, &right) {
            match expr.operator.token_type {
                TokenType::Plus => Ok(Value::String(format!("{}{}", &left_str, &right_str))),
                _ => Err(InterpreterError::new(
                    expr.operator.clone(),
                    "Invalid infix operator for two strings",
                )),
            }
        } else {
            match expr.operator.token_type {
                TokenType::BangEqual => Ok(Value::Boolean(!(left == right))),
                TokenType::EqualEqual => Ok(Value::Boolean(left == right)),
                _ => Err(InterpreterError::new(
                    expr.operator.clone(),
                    "Invalid binary operation",
                )),
            }
        }
    }

    fn evaluate(&self, expr: &expr::Expr) -> Result<Value> {
        match expr {
            expr::Expr::Binary(binary) => self.visit_binary_expr(binary),
            expr::Expr::Grouping(grouping) => self.visit_grouping_expr(grouping),
            expr::Expr::Literal(literal) => self.visit_literal_expr(literal),
            expr::Expr::Unary(unary) => self.visit_unary_expr(unary),
        }
    }
}

type Result<T> = std::result::Result<T, InterpreterError>;

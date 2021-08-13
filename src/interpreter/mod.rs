use crate::parser::expr;
use crate::scanner::token::Literal;
use crate::scanner::token_type::TokenType;
pub struct Interpreter {}

#[derive(PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Interpreter {
    pub fn visit_literal_expr(&self, expr: &expr::Literal) -> Value {
        match &*expr.value {
            Literal::Number(num) => Value::Number(*num),
            Literal::String(string) => Value::String(string.clone()),
            Literal::True => Value::Boolean(true),
            Literal::False => Value::Boolean(false),
            Literal::Nil => Value::Nil,
        }
    }

    pub fn visit_grouping_expr(&self, expr: &expr::Grouping) -> Value {
        self.evaluate(&expr.expression)
    }

    pub fn visit_unary_expr(&self, expr: &expr::Unary) -> Value {
        let right = self.evaluate(&expr.right);

        match expr.operator.token_type {
            TokenType::Bang => Value::Boolean(!self.is_truthy(right)),
            TokenType::Minus => {
                if let Value::Number(num) = right {
                    Value::Number(-num)
                } else {
                    todo!("Can only apply negation to a number")
                }
            }
            _ => todo!(),
        }
    }

    fn is_truthy(&self, val: Value) -> bool {
        match val {
            Value::Nil => false,
            Value::Boolean(boolean) => boolean,
            _ => true,
        }
    }

    fn visit_binary_expr(&self, expr: &expr::Binary) -> Value {
        let left = self.evaluate(&expr.left);
        let right = self.evaluate(&expr.right);

        if let (Value::Number(left_num), Value::Number(right_num)) = (&left, &right) {
            match expr.operator.token_type {
                TokenType::Greater => Value::Boolean(left_num > right_num),
                TokenType::GreaterEqual => Value::Boolean(left_num >= right_num),
                TokenType::Less => Value::Boolean(left_num < right_num),
                TokenType::LessEqual => Value::Boolean(left_num <= right_num),
                TokenType::Minus => Value::Number(left_num - right_num),
                TokenType::Plus => Value::Number(left_num + right_num),
                TokenType::Slash => Value::Number(left_num / right_num),
                TokenType::Star => Value::Number(left_num * right_num),
                _ => todo!(),
            }
        } else if let (Value::String(left_str), Value::String(right_str)) = (&left, &right) {
            match expr.operator.token_type {
                TokenType::Plus => Value::String(format!("{}{}", &left_str, &right_str)),
                _ => todo!(),
            }
        } else {
            match expr.operator.token_type {
                TokenType::BangEqual => Value::Boolean(!(left == right)),
                TokenType::EqualEqual => Value::Boolean(left == right),
                _ => todo!(),
            }
        }
    }

    fn evaluate(&self, expr: &expr::Expr) -> Value {
        match expr {
            expr::Expr::Binary(binary) => self.visit_binary_expr(binary),
            expr::Expr::Grouping(grouping) => self.visit_grouping_expr(grouping),
            expr::Expr::Literal(literal) => self.visit_literal_expr(literal),
            expr::Expr::Unary(_) => todo!(),
        }
    }
}

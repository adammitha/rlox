use std::cell::RefCell;
use std::rc::Rc;

use crate::error::SimpleErrorHandler;
use crate::parser::{expr, stmt};
use crate::scanner::token::Literal;
use crate::scanner::token_type::TokenType;
pub mod environment;
pub mod error;
mod value;
use environment::Environment;
use error::InterpreterError;
use value::Value;

pub struct Interpreter<'a> {
    error_handler: &'a mut SimpleErrorHandler,
    environment: Rc<RefCell<Environment>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(
        error_handler: &'a mut SimpleErrorHandler,
        environment: Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            error_handler,
            environment,
        }
    }

    fn visit_literal_expr(&self, expr: &expr::Literal) -> Result<Value> {
        Ok(match &*expr.value {
            Literal::Number(num) => Value::Number(*num),
            Literal::String(string) => Value::String(string.clone()),
            Literal::True => Value::Boolean(true),
            Literal::False => Value::Boolean(false),
            Literal::Nil => Value::Nil,
        })
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping) -> Result<Value> {
        self.evaluate(&expr.expression)
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> Result<Value> {
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

    fn visit_variable_expr(&self, expr: &expr::Variable) -> Result<Value> {
        match self.environment.borrow().get(&expr.name) {
            Some(val) => Ok(val.clone()),
            None => Err(InterpreterError::new(
                expr.name.clone(),
                &format!("Undefined variable '{}'.", expr.name.lexeme),
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

    fn visit_binary_expr(&mut self, expr: &expr::Binary) -> Result<Value> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        let equality = match expr.operator.token_type {
            TokenType::BangEqual => Some(Value::Boolean(!(left == right))),
            TokenType::EqualEqual => Some(Value::Boolean(left == right)),
            _ => None,
        };

        match equality {
            Some(val) => return Ok(val),
            None => (),
        };

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
            Err(InterpreterError::new(
                expr.operator.clone(),
                "Invalid binary operation",
            ))
        }
    }

    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Result<Value> {
        let value = self.evaluate(&expr.value)?;
        match self
            .environment
            .borrow_mut()
            .assign(&expr.name, value.clone())
        {
            Some(_) => Ok(value), // Discard old value - we want to return updated value
            None => Err(InterpreterError::new(
                expr.name.clone(),
                &format!("Undefined variable '{}'.", expr.name.lexeme),
            )),
        }
    }

    fn evaluate(&mut self, expr: &expr::Expr) -> Result<Value> {
        match expr {
            expr::Expr::Assign(assign) => self.visit_assign_expr(assign),
            expr::Expr::Binary(binary) => self.visit_binary_expr(binary),
            expr::Expr::Grouping(grouping) => self.visit_grouping_expr(grouping),
            expr::Expr::Literal(literal) => self.visit_literal_expr(literal),
            expr::Expr::Unary(unary) => self.visit_unary_expr(unary),
            expr::Expr::Variable(variable) => self.visit_variable_expr(variable),
        }
    }

    fn visit_expression_statement(&mut self, stmt: stmt::Expression) {
        match self.evaluate(&stmt.expression) {
            Ok(_) => (),
            Err(err) => self.error_handler.runtime_error(err),
        };
    }

    fn visit_print_stmt(&mut self, stmt: stmt::Print) {
        match self.evaluate(&stmt.expression) {
            Ok(val) => println!("{}", val),
            Err(err) => self.error_handler.runtime_error(err),
        }
    }

    fn visit_var_stmt(&mut self, stmt: stmt::Var) {
        let value = match self.evaluate(&stmt.initializer) {
            Ok(val) => val,
            Err(err) => {
                self.error_handler.runtime_error(err);
                return;
            }
        };
        self.environment
            .borrow_mut()
            .define(&stmt.name.lexeme, value);
    }

    fn visit_block_stmt(&mut self, stmt: stmt::Block) {
        self.execute_block(
            stmt.statements,
            Rc::new(RefCell::new(Environment::with_enclosing(
                self.environment.clone(),
            ))),
        );
    }

    fn execute_block(
        &mut self,
        statements: Vec<stmt::Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) {
        let previous = self.environment.clone();
        self.environment = environment;

        for statement in statements {
            self.execute(statement)
        }

        self.environment = previous;
    }

    pub fn interpret(&mut self, statements: Vec<stmt::Stmt>) {
        for statement in statements {
            self.execute(statement)
        }
    }

    fn execute(&mut self, statement: stmt::Stmt) {
        match statement {
            stmt::Stmt::Block(block_statement) => self.visit_block_stmt(block_statement),
            stmt::Stmt::Expression(expression_statement) => {
                self.visit_expression_statement(expression_statement)
            }
            stmt::Stmt::Print(print_statement) => self.visit_print_stmt(print_statement),
            stmt::Stmt::Var(var_statement) => self.visit_var_stmt(var_statement),
        }
    }
}

type Result<T> = std::result::Result<T, InterpreterError>;

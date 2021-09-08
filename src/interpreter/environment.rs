use crate::scanner::token::Token;

use super::value::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(String::from(name), value);
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Option<Value> {
        match self
            .values
            .insert(String::from(&name.lexeme), value.clone())
        {
            Some(value) => Some(value),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow_mut().assign(name, value),
                None => None,
            },
        }
    }

    pub fn get(&self, name: &Token) -> Option<Value> {
        match self.values.get(&name.lexeme) {
            Some(val) => Some(val.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow().get(name),
                None => None,
            },
        }
    }
}

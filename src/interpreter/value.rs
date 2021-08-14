use std::fmt::Display;

#[derive(PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(num) => {
                let num_rounded = (num * 100_000_000.0).round() / 100_000_000.0;
                write!(f, "{}", num_rounded)
            }
            Value::String(string) => write!(f, "{}", string),
            Value::Boolean(boolean) => write!(f, "{}", boolean),
            Value::Nil => write!(f, "nil"),
        }
    }
}

use super::expr::Expr;

pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Binary(expr) => parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right]),
        Expr::Grouping(expr) => parenthesize("group", &[&expr.expression]),
        Expr::Literal(expr) => match &expr.value {
            Some(value) => value.to_string(),
            None => String::from("nil"),
        },
        Expr::Unary(expr) => parenthesize(&expr.operator.lexeme, &[&expr.right]),
    }
}

fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
    let mut out = String::new();
    out.push('(');
    out.push_str(name);
    for &expr in exprs {
        out.push(' ');
        out.push_str(&print(expr));
    }
    out.push(')');
    out
}

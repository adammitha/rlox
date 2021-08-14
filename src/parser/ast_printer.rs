use super::expr::Expr;

#[allow(dead_code)]
pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Binary(expr) => parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right]),
        Expr::Grouping(expr) => parenthesize("group", &[&expr.expression]),
        Expr::Literal(expr) => expr.value.to_string(),
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

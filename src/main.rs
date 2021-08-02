use std::{env, io, process};

use rlox::Lox;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        lox.run_file(&args[1])?
    } else {
        lox.run_prompt()?;
    }
    Ok(())
}

// use rlox::parser::ast_printer;
// use rlox::parser::expr;
// use rlox::scanner::token::Literal;
// fn main() {
//     let my_expr = produce_expr();
//     println!("{}", ast_printer::print(&my_expr));
// }

// fn produce_expr() -> expr::Expr {
//     let string = Literal::String(String::from("This is a string literal"));
//     expr::Expr::Literal(expr::Literal {
//         value: Box::new(string),
//     })
// }

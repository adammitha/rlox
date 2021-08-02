// use std::{env, io, process};

// use rlox::Lox;

// fn main() -> io::Result<()> {
//     let args: Vec<String> = env::args().collect();
//     let mut lox = Lox::new();
//     if args.len() > 2 {
//         println!("Usage: rlox [script]");
//         process::exit(64);
//     } else if args.len() == 2 {
//         lox.run_file(&args[1])?
//     } else {
//         lox.run_prompt()?;
//     }
//     Ok(())
// }

use rlox::parser::ast_printer;
use rlox::parser::expr;
use rlox::scanner::token::Token;
use rlox::scanner::token_type::TokenType;

fn main() {
    let expression = expr::Expr::Binary(expr::Binary {
        left: Box::new(expr::Expr::Unary(expr::Unary {
            operator: Token::new(TokenType::Minus, String::from("-"), None, 1),
            right: Box::new(expr::Expr::Literal(expr::Literal {
                value: Some(Box::new(123)),
            })),
        })),
        operator: Token::new(TokenType::Star, String::from("*"), None, 1),
        right: Box::new(expr::Expr::Grouping(expr::Grouping {
            expression: Box::new(expr::Expr::Literal(expr::Literal {
                value: Some(Box::new(45.67)),
            })),
        })),
    });
    println!("{}", ast_printer::print(&expression));
}

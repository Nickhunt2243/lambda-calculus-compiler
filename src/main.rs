extern crate core;

pub mod lexer;
pub mod ast;
mod helpers;
mod type_inferencer;


use lexer::{Lexer};
use ast::{parse};

fn main() {
    let mut lexer = Lexer::new("let x = 5 in x");
    let tokens = lexer.tokenize().unwrap();

    let ast = parse(tokens);

    println!("{:?}", ast);
}

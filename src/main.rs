extern crate core;

pub mod lexer;
pub mod ast;
mod type_inferencer;


use lexer::{Lexer};
use ast::{parse};
use type_inferencer::{type_inference};

fn main() {
    let mut lexer = Lexer::new("let x = fn y => y + 3 in x");
    let tokens = lexer.tokenize().unwrap();
    let ast = parse(tokens).unwrap();

    let return_type = type_inference(&ast);
    println!("Result: {:?}", return_type);
}

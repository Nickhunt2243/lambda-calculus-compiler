pub mod types;
use types::{Expr};
use crate::lexer::types::{Token};

pub fn parse(tokens: &[Token]) -> Result<Expr, String> {
    let (expr, _) = Expr::new(tokens, 0)?;
    Ok(expr)
}
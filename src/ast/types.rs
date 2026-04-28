use crate::lexer::types::{Token, BooleanOps, AdditiveOps, Keyword, MultiplicativeOps};

// const INVALID_EXPR_ERROR: &str = "
// Invalid expression. Expected expression syntax:
// <expr>     ::= fn <identifier> => <expr>
//             | let rec <identifier> = <expr> in <expr>
//             | let <identifier> = <expr> in <expr>
//             | if <expr> then <expr> else <expr>
//             | <comp_expr>
// ";
// const INVALID_FUNC_DECL_EXPR_ERROR: &str = "Incomplete func declaration. Syntax expected: fn <identifier> => <expr>";
// const INVALID_LET_EXPR_ERROR: &str = "Incomplete Let statement. Syntax expected: let <identifier> = <expr> in <expr>";
// const INVALID_IF_EXPR_ERROR: &str = "Incomplete if statement. Syntax expected: if <expr> then <expr> else <expr>";
type ParseResult<T> = Result<(T, usize), String>;

#[derive(Debug)]
pub struct LetExpr {
    pub identifier: String,
    pub value: Box<Expr>,
    pub body_expr: Box<Expr>,
}

fn parse_let_expr(tokens: &[Token], idx: usize) -> ParseResult<(String, Box<Expr>, Box<Expr>)> {
    let identifier = match tokens.get(idx + 1) {
        Some(Token::Identifier(identifier)) => identifier,
        Some(other) => return Err(format!("Expected =, received: {:?}.", other)),
        None => return Err("Unexpected end of input, expected =.".to_string()),
    };
    match tokens.get(idx + 2) {
        Some(Token::EqualSign) => {},
        Some(other) => return Err(format!("Expected =, received: {:?}.", other)),
        None => return Err("Unexpected end of input, expected =.".to_string()),
    }

    let (value, next_idx) = Expr::new(tokens, idx + 3)?;
    match tokens.get(next_idx) {
        Some(Token::Keyword(Keyword::In)) => {},
        Some(other) => return Err(format!("Expected in, received: {:?}.", other)),
        None => return Err("Unexpected end of input, expected =.".to_string()),
    }

    let (body_expr, next_idx) = Expr::new(tokens, next_idx + 1)?;

    Ok(
        (
            (identifier.clone(), Box::new(value), Box::new(body_expr)), next_idx
        )
    )
}

impl LetExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Self> {
        let ((identifier, value, body_expr), next_idx) = parse_let_expr(tokens, idx)?;
        Ok(
            (Self {
                identifier,
                value,
                body_expr,
            }, next_idx)
        )
    }
}

#[derive(Debug)]
pub struct LetRecExpr {
    pub identifier: String,
    pub value: Box<Expr>,
    pub body_expr: Box<Expr>,
}

impl LetRecExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Self> {
        let ((identifier, value, body_expr), next_idx) = parse_let_expr(tokens, idx)?;
        Ok(
            (Self {
                identifier,
                value,
                body_expr,
            }, next_idx)
        )
    }
}

#[derive(Debug)]
pub struct IfExpr {
    pub bool_expr: Box<Expr>,
    pub then_expr: Box<Expr>,
    pub else_expr: Box<Expr>,
}

impl IfExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Self> {
        let (bool_expr, next_idx) = Expr::new(tokens, idx + 1)?;
        match tokens.get(next_idx) {
            Some(Token::Keyword(Keyword::Then)) => {},
            Some(other) => return Err(format!("Expected then, received: {:?}.", other)),
            None => return Err("Unexpected end of input, expected =.".to_string()),
        }
        let (then_expr, next_idx) = Expr::new(tokens, next_idx + 1)?;
        match tokens.get(next_idx) {
            Some(Token::Keyword(Keyword::Else)) => {},
            Some(other) => return Err(format!("Expected else, received: {:?}.", other)),
            None => return Err("Unexpected end of input, expected =.".to_string()),
        }
        let (else_expr, next_idx) = Expr::new(tokens, next_idx + 1)?;

        Ok(
            (Self {
                bool_expr: Box::new(bool_expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr)
            }, next_idx)
        )
    }
}

#[derive(Debug)]
pub struct FunctionDeclExpr {
    pub param: String,
    pub body_expr: Box<Expr>,
}

impl FunctionDeclExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Self> {
        let identifier = match tokens.get(idx + 1) {
            Some(Token::Identifier(identifier)) => identifier,
            Some(other) => return Err(format!("Expected identifier, received: {:?}.", other)),
            None => return Err("Unexpected end of input, expected =.".to_string()),
        };
        match tokens.get(idx + 2) {
            Some(Token::Arrow) => {},
            Some(other) => return Err(format!("Expected =>, received: {:?}.", other)),
            None => return Err("Unexpected end of input, expected =.".to_string()),
        }
        let (body_expr, next_idx) = Expr::new(tokens, idx + 3)?;

        Ok(
            (Self {
                param: identifier.clone(),
                body_expr: Box::new(body_expr)
            }, next_idx)
        )
    }
}

#[derive(Debug)]
pub struct CompExpr {
    pub add_expr: AddExpr,
    pub chained_expr: Option<ChainedCompExpr>
}

impl CompExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Self> {
        let (add_expr, next_idx) = AddExpr::new(tokens, idx)?;
        let (chained_expr, next_idx) = ChainedCompExpr::new(tokens, next_idx)?;

        Ok(
            (Self {
                add_expr,
                chained_expr
            }, next_idx)
        )
    }
}

#[derive(Debug)]
pub struct ChainedCompExpr {
    pub add_expr: AddExpr,
    pub comp_op: BooleanOps
}

impl ChainedCompExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Option<Self>> {
        let comp_op = match tokens.get(idx) {
            Some(Token::BooleanOps(ops)) => {ops.clone()}
            Some(_) | None => return Ok((None, idx)),
        };

        let (add_expr, next_idx) = AddExpr::new(tokens, idx + 1)?;

        Ok(
            (
                Some(Self {
                    add_expr,
                    comp_op
                }),
                next_idx
            )
        )
    }
}

#[derive(Debug)]
pub struct AddExpr {
    pub mul_expr: Box<MulExpr>,
    pub chained_expr: Option<Box<ChainedAddExpr>>
}

impl AddExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Self> {
        let (mul_expr, next_idx) = MulExpr::new(tokens, idx)?;
        let (chained_expr, next_idx) = ChainedAddExpr::new(tokens, next_idx)?;

        Ok(
            (Self {
                mul_expr: Box::new(mul_expr),
                chained_expr
            }, next_idx)
        )
    }
}

#[derive(Debug)]
pub struct ChainedAddExpr {
    pub mul_expr: MulExpr,
    pub additive_ops: AdditiveOps,
    pub chained_expr: Option<Box<ChainedAddExpr>>,
}

impl ChainedAddExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Option<Box<Self>>> {
        let additive_ops = match tokens.get(idx) {
            Some(Token::AdditiveOps(ops)) => {ops.clone()}
            Some(_) | None => return Ok((None, idx)),
        };

        let (mul_expr, next_idx) = MulExpr::new(tokens, idx + 1)?;
        let (chained_expr, next_idx) = ChainedAddExpr::new(tokens, next_idx)?;

        Ok(
            (
                Some(Box::new(Self {
                    mul_expr,
                    additive_ops,
                    chained_expr
                })),
                next_idx
            )
        )
    }
}

#[derive(Debug)]
pub struct MulExpr {
    pub app_expr: AppExpr,
    pub chained_expr: Option<Box<ChainedMulExpr>>
}

impl MulExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Self> {
        let (app_expr, next_idx) = AppExpr::new(tokens, idx)?;
        let (chained_expr, next_idx) = ChainedMulExpr::new(tokens, next_idx)?;

        Ok((Self {app_expr, chained_expr}, next_idx))
    }
}

#[derive(Debug)]
pub struct ChainedMulExpr {
    pub app_expr: AppExpr,
    pub multiplicative_ops: MultiplicativeOps,
    pub chained_expr: Option<Box<ChainedMulExpr>>
}

impl ChainedMulExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Option<Box<Self>>> {
        let multiplicative_ops = match tokens.get(idx) {
            Some(Token::MultiplicativeOps(ops)) => {ops.clone()}
            Some(_) | None => return Ok((None, idx)),
        };

        let (app_expr, next_idx) = AppExpr::new(tokens, idx + 1)?;
        let (chained_expr, next_idx) = ChainedMulExpr::new(tokens, next_idx)?;

        Ok(
            (
                Some(Box::new(Self {
                    app_expr,
                    multiplicative_ops,
                    chained_expr
                })),
                next_idx
            )
        )
    }
}

#[derive(Debug)]
pub struct AppExpr {
    pub atom: Atom,
    pub chained_expr: Option<Box<ChainedAppExpr>>,
}

impl AppExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Self> {
        let (atom, next_idx) = Atom::new(tokens, idx)?;
        let (chained_expr, next_idx) = ChainedAppExpr::new(tokens, next_idx)?;

        Ok((Self {atom, chained_expr}, next_idx))
    }
}

#[derive(Debug)]
pub struct ChainedAppExpr {
    pub atom: Atom,
    pub chained_expr: Option<Box<ChainedAppExpr>>,
}

impl ChainedAppExpr {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Option<Box<Self>>> {
        match tokens.get(idx) {
            Some(
                Token::Identifier(_)
                | Token::IntegerLiteral(_)
                | Token::BooleanLiteral(_)
                | Token::LParen
            ) => {},
            Some(_) | None => return Ok((None, idx))
        }

        let (atom, next_idx) = Atom::new(tokens, idx)?;
        let (chained_expr, next_idx) = ChainedAppExpr::new(tokens, next_idx)?;

        Ok(
            (
                Some(Box::new(Self {atom, chained_expr})),
                next_idx
            )
        )
    }
}

#[derive(Debug)]
pub enum Atom {
    Identifier(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    PrioritizedExpr(Box<Expr>),
}

impl Atom {
    pub fn new(tokens: &[Token], idx: usize) -> ParseResult<Self> {
        match tokens.get(idx) {
            Some(Token::Identifier(identifier)) => Ok((Atom::Identifier(identifier.clone()), idx + 1)),
            Some(Token::IntegerLiteral(integer_literal)) => Ok((Atom::IntegerLiteral(integer_literal.clone()), idx + 1)),
            Some(Token::BooleanLiteral(boolean_literal)) => Ok((Atom::BooleanLiteral(boolean_literal.clone()), idx + 1)),
            Some(Token::LParen) => {
                let (expr, next_idx) = Expr::new(tokens, idx + 1)?;
                match tokens.get(next_idx) {
                    Some(Token::RParen) => Ok((Atom::PrioritizedExpr(Box::new(expr)), next_idx + 1)),
                    Some(other) => Err(format!("Expected ), received: {:?}", other)),
                    None => Err("Unexpected end of input, expected )".to_string()),
                }
            },
            Some(other) => return Err(format!("Failed to build atom, received: {:?}", other)),
            None => Err("Unexpected end of input, expected =.".to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    LetExpr(LetExpr),
    LetRecExpr(LetRecExpr),
    IfExpr(IfExpr),
    FunctionDeclExpr(FunctionDeclExpr),
    CompExpr(CompExpr),
}

impl Expr {
    pub fn new(tokens: &[Token], idx: usize) -> Result<(Expr, usize), String> {
        let token = tokens.get(idx).expect("Unexpected end of token stream.");
        match token {
            Token::Keyword(Keyword::Fn) => {
                let (func_decl_expr, next_idx) = FunctionDeclExpr::new(tokens, idx)?;
                Ok((Expr::FunctionDeclExpr(func_decl_expr), next_idx))
            },
            Token::Keyword(Keyword::Let) => {
                let (let_expr, next_idx) = LetExpr::new(tokens, idx)?;
                Ok((Expr::LetExpr(let_expr), next_idx))
            },
            Token::Keyword(Keyword::LetRec) => {
                let (let_rec_expr, next_idx) = LetRecExpr::new(tokens, idx)?;
                Ok((Expr::LetRecExpr(let_rec_expr), next_idx))
            },
            Token::Keyword(Keyword::If) => {
                let (if_expr, next_idx) = IfExpr::new(tokens, idx)?;
                Ok((Expr::IfExpr(if_expr), next_idx))
            },
            Token::Identifier(_)
            | Token::IntegerLiteral(_)
            | Token::BooleanLiteral(_)
            | Token::LParen => {
                let (comp_expr, next_idx) = CompExpr::new(tokens, idx)?;
                Ok((Expr::CompExpr(comp_expr), next_idx))
            },
            _ => Err("Failed to parse expression".to_string()),
        }
    }
}
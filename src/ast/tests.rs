use super::types::*;
use crate::lexer::types::{Token, Keyword, AdditiveOps, MultiplicativeOps, BooleanOps};

// ── helpers ───────────────────────────────────────────────────────────────────

fn parse(tokens: Vec<Token>) -> Expr {
    let (expr, _) = Expr::new(&tokens, 0).unwrap();
    expr
}

// ── integer literal ───────────────────────────────────────────────────────────

#[test]
fn test_parse_integer_literal() {
    let tokens = vec![Token::IntegerLiteral(42)];
    let expr = parse(tokens);
    assert!(matches!(expr, Expr::CompExpr(_)));
}

#[test]
fn test_parse_boolean_literal_true() {
    let tokens = vec![Token::BooleanLiteral(true)];
    let expr = parse(tokens);
    assert!(matches!(expr, Expr::CompExpr(_)));
}

#[test]
fn test_parse_boolean_literal_false() {
    let tokens = vec![Token::BooleanLiteral(false)];
    let expr = parse(tokens);
    assert!(matches!(expr, Expr::CompExpr(_)));
}

// ── identifier ────────────────────────────────────────────────────────────────

#[test]
fn test_parse_identifier() {
    let tokens = vec![Token::Identifier("x".to_string())];
    let expr = parse(tokens);
    assert!(matches!(expr, Expr::CompExpr(_)));
}

// ── let expression ────────────────────────────────────────────────────────────

#[test]
fn test_parse_let_expr() {
    // let x = 5 in x
    let tokens = vec![
        Token::Keyword(Keyword::Let),
        Token::Identifier("x".to_string()),
        Token::EqualSign,
        Token::IntegerLiteral(5),
        Token::Keyword(Keyword::In),
        Token::Identifier("x".to_string()),
    ];
    let expr = parse(tokens);
    match expr {
        Expr::LetExpr(let_expr) => {
            assert_eq!(let_expr.identifier, "x");
            assert!(matches!(*let_expr.value, Expr::CompExpr(_)));
            assert!(matches!(*let_expr.body_expr, Expr::CompExpr(_)));
        }
        _ => panic!("Expected LetExpr"),
    }
}

#[test]
fn test_parse_let_missing_equal_sign_errors() {
    // let x 5 in x — missing =
    let tokens = vec![
        Token::Keyword(Keyword::Let),
        Token::Identifier("x".to_string()),
        Token::IntegerLiteral(5),
        Token::Keyword(Keyword::In),
        Token::Identifier("x".to_string()),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

#[test]
fn test_parse_let_missing_in_errors() {
    // let x = 5 x — missing in
    let tokens = vec![
        Token::Keyword(Keyword::Let),
        Token::Identifier("x".to_string()),
        Token::EqualSign,
        Token::IntegerLiteral(5),
        Token::Identifier("x".to_string()),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

// ── letrec expression ─────────────────────────────────────────────────────────

#[test]
fn test_parse_letrec_expr() {
    // letrec f = fn x => x in f 5
    let tokens = vec![
        Token::Keyword(Keyword::LetRec),
        Token::Identifier("f".to_string()),
        Token::EqualSign,
        Token::Keyword(Keyword::Fn),
        Token::Identifier("x".to_string()),
        Token::Arrow,
        Token::Identifier("x".to_string()),
        Token::Keyword(Keyword::In),
        Token::Identifier("f".to_string()),
        Token::IntegerLiteral(5),
    ];
    let expr = parse(tokens);
    match expr {
        Expr::LetRecExpr(let_rec) => {
            assert_eq!(let_rec.identifier, "f");
        }
        _ => panic!("Expected LetRecExpr"),
    }
}

// ── fn expression ─────────────────────────────────────────────────────────────

#[test]
fn test_parse_fn_expr() {
    // fn x => x
    let tokens = vec![
        Token::Keyword(Keyword::Fn),
        Token::Identifier("x".to_string()),
        Token::Arrow,
        Token::Identifier("x".to_string()),
    ];
    let expr = parse(tokens);
    match expr {
        Expr::FunctionDeclExpr(func) => {
            assert_eq!(func.param, "x");
        }
        _ => panic!("Expected FunctionDeclExpr"),
    }
}

#[test]
fn test_parse_fn_missing_arrow_errors() {
    // fn x x — missing =>
    let tokens = vec![
        Token::Keyword(Keyword::Fn),
        Token::Identifier("x".to_string()),
        Token::Identifier("x".to_string()),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

#[test]
fn test_parse_curried_fn() {
    // fn x => fn y => x
    let tokens = vec![
        Token::Keyword(Keyword::Fn),
        Token::Identifier("x".to_string()),
        Token::Arrow,
        Token::Keyword(Keyword::Fn),
        Token::Identifier("y".to_string()),
        Token::Arrow,
        Token::Identifier("x".to_string()),
    ];
    let expr = parse(tokens);
    match expr {
        Expr::FunctionDeclExpr(outer) => {
            assert_eq!(outer.param, "x");
            assert!(matches!(*outer.body_expr, Expr::FunctionDeclExpr(_)));
        }
        _ => panic!("Expected nested FunctionDeclExpr"),
    }
}

// ── if expression ─────────────────────────────────────────────────────────────

#[test]
fn test_parse_if_expr() {
    // if true then 1 else 0
    let tokens = vec![
        Token::Keyword(Keyword::If),
        Token::BooleanLiteral(true),
        Token::Keyword(Keyword::Then),
        Token::IntegerLiteral(1),
        Token::Keyword(Keyword::Else),
        Token::IntegerLiteral(0),
    ];
    let expr = parse(tokens);
    assert!(matches!(expr, Expr::IfExpr(_)));
}

#[test]
fn test_parse_if_missing_then_errors() {
    let tokens = vec![
        Token::Keyword(Keyword::If),
        Token::BooleanLiteral(true),
        Token::IntegerLiteral(1),
        Token::Keyword(Keyword::Else),
        Token::IntegerLiteral(0),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

#[test]
fn test_parse_if_missing_else_errors() {
    let tokens = vec![
        Token::Keyword(Keyword::If),
        Token::BooleanLiteral(true),
        Token::Keyword(Keyword::Then),
        Token::IntegerLiteral(1),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

// ── arithmetic ────────────────────────────────────────────────────────────────

#[test]
fn test_parse_addition() {
    // 1 + 2
    let tokens = vec![
        Token::IntegerLiteral(1),
        Token::AdditiveOps(AdditiveOps::Add),
        Token::IntegerLiteral(2),
    ];
    let expr = parse(tokens);
    assert!(matches!(expr, Expr::CompExpr(_)));
}

#[test]
fn test_parse_multiplication() {
    // 3 * 4
    let tokens = vec![
        Token::IntegerLiteral(3),
        Token::MultiplicativeOps(MultiplicativeOps::Mul),
        Token::IntegerLiteral(4),
    ];
    let expr = parse(tokens);
    assert!(matches!(expr, Expr::CompExpr(_)));
}

#[test]
fn test_parse_comparison_less_than() {
    // x < 5
    let tokens = vec![
        Token::Identifier("x".to_string()),
        Token::BooleanOps(BooleanOps::LessThan),
        Token::IntegerLiteral(5),
    ];
    let expr = parse(tokens);
    match expr {
        Expr::CompExpr(comp) => {
            assert!(comp.chained_expr.is_some());
            assert!(matches!(
                comp.chained_expr.unwrap().comp_op,
                BooleanOps::LessThan
            ));
        }
        _ => panic!("Expected CompExpr"),
    }
}

// ── function application ──────────────────────────────────────────────────────

#[test]
fn test_parse_function_application() {
    // f x
    let tokens = vec![
        Token::Identifier("f".to_string()),
        Token::Identifier("x".to_string()),
    ];
    let expr = parse(tokens);
    match expr {
        Expr::CompExpr(comp) => {
            let app_expr = &comp.add_expr.mul_expr.app_expr;
            assert!(app_expr.chained_expr.is_some());
        }
        _ => panic!("Expected CompExpr with application"),
    }
}

// ── parenthesized expressions ─────────────────────────────────────────────────

#[test]
fn test_parse_parenthesized_expr() {
    // (5)
    let tokens = vec![
        Token::LParen,
        Token::IntegerLiteral(5),
        Token::RParen,
    ];
    let expr = parse(tokens);
    assert!(matches!(expr, Expr::CompExpr(_)));
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn test_parse_unexpected_token_errors() {
    // = is not a valid expression start
    let tokens = vec![Token::EqualSign];
    assert!(Expr::new(&tokens, 0).is_err());
}
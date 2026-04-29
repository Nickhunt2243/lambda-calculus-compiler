use super::types::*;
use crate::lexer::types::{Token, Keyword, AdditiveOps, MultiplicativeOps, BooleanOps};

fn parse(tokens: Vec<Token>) -> Expr {
    Expr::new(&tokens, 0).unwrap().0
}

// ── FunctionDeclExpr ──────────────────────────────────────────────────────────

#[test]
fn test_fn_decl_structure() {
    // fn x => x: param is x, body is identifier x
    let tokens = vec![
        Token::Keyword(Keyword::Fn),
        Token::Identifier("x".to_string()),
        Token::Arrow,
        Token::Identifier("x".to_string()),
    ];
    match parse(tokens) {
        Expr::FunctionDeclExpr(f) => {
            assert_eq!(f.param, "x");
            assert!(matches!(*f.body_expr, Expr::CompExpr(_)));
        }
        _ => panic!("Expected FunctionDeclExpr"),
    }
}

#[test]
fn test_fn_decl_curried_nests_correctly() {
    // fn x => fn y => x: outer param x, inner param y, body x
    let tokens = vec![
        Token::Keyword(Keyword::Fn), Token::Identifier("x".to_string()), Token::Arrow,
        Token::Keyword(Keyword::Fn), Token::Identifier("y".to_string()), Token::Arrow,
        Token::Identifier("x".to_string()),
    ];
    match parse(tokens) {
        Expr::FunctionDeclExpr(outer) => {
            assert_eq!(outer.param, "x");
            match *outer.body_expr {
                Expr::FunctionDeclExpr(inner) => {
                    assert_eq!(inner.param, "y");
                    assert!(matches!(*inner.body_expr, Expr::CompExpr(_)));
                }
                _ => panic!("Expected inner FunctionDeclExpr"),
            }
        }
        _ => panic!("Expected FunctionDeclExpr"),
    }
}

#[test]
fn test_fn_decl_body_can_be_any_expr() {
    // fn x => if x then 1 else 0: body is IfExpr
    let tokens = vec![
        Token::Keyword(Keyword::Fn), Token::Identifier("x".to_string()), Token::Arrow,
        Token::Keyword(Keyword::If), Token::Identifier("x".to_string()),
        Token::Keyword(Keyword::Then), Token::IntegerLiteral(1),
        Token::Keyword(Keyword::Else), Token::IntegerLiteral(0),
    ];
    match parse(tokens) {
        Expr::FunctionDeclExpr(f) => assert!(matches!(*f.body_expr, Expr::IfExpr(_))),
        _ => panic!("Expected FunctionDeclExpr"),
    }
}

#[test]
fn test_fn_decl_missing_arrow_errors() {
    let tokens = vec![
        Token::Keyword(Keyword::Fn),
        Token::Identifier("x".to_string()),
        Token::Identifier("x".to_string()),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

#[test]
fn test_fn_decl_missing_identifier_errors() {
    let tokens = vec![
        Token::Keyword(Keyword::Fn),
        Token::Arrow,
        Token::Identifier("x".to_string()),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

// ── LetExpr ───────────────────────────────────────────────────────────────────

#[test]
fn test_let_expr_structure() {
    // let x = 5 in x + 1: identifier, value, body all correct
    let tokens = vec![
        Token::Keyword(Keyword::Let), Token::Identifier("x".to_string()), Token::EqualSign,
        Token::IntegerLiteral(5), Token::Keyword(Keyword::In),
        Token::Identifier("x".to_string()), Token::AdditiveOps(AdditiveOps::Add), Token::IntegerLiteral(1),
    ];
    match parse(tokens) {
        Expr::LetExpr(l) => {
            assert_eq!(l.identifier, "x");
            assert!(matches!(*l.value, Expr::CompExpr(_)));
            assert!(matches!(*l.body_expr, Expr::CompExpr(_)));
        }
        _ => panic!("Expected LetExpr"),
    }
}

#[test]
fn test_let_value_can_be_fn() {
    // let f = fn x => x in f: value is FunctionDeclExpr
    let tokens = vec![
        Token::Keyword(Keyword::Let), Token::Identifier("f".to_string()), Token::EqualSign,
        Token::Keyword(Keyword::Fn), Token::Identifier("x".to_string()), Token::Arrow,
        Token::Identifier("x".to_string()), Token::Keyword(Keyword::In),
        Token::Identifier("f".to_string()),
    ];
    match parse(tokens) {
        Expr::LetExpr(l) => {
            assert_eq!(l.identifier, "f");
            assert!(matches!(*l.value, Expr::FunctionDeclExpr(_)));
        }
        _ => panic!("Expected LetExpr"),
    }
}

#[test]
fn test_let_body_can_be_let() {
    // let x = 1 in let y = 2 in x: nested let
    let tokens = vec![
        Token::Keyword(Keyword::Let), Token::Identifier("x".to_string()), Token::EqualSign,
        Token::IntegerLiteral(1), Token::Keyword(Keyword::In),
        Token::Keyword(Keyword::Let), Token::Identifier("y".to_string()), Token::EqualSign,
        Token::IntegerLiteral(2), Token::Keyword(Keyword::In),
        Token::Identifier("x".to_string()),
    ];
    match parse(tokens) {
        Expr::LetExpr(outer) => {
            assert!(matches!(*outer.body_expr, Expr::LetExpr(_)));
        }
        _ => panic!("Expected nested LetExpr"),
    }
}

#[test]
fn test_let_missing_equal_errors() {
    let tokens = vec![
        Token::Keyword(Keyword::Let), Token::Identifier("x".to_string()),
        Token::IntegerLiteral(5), Token::Keyword(Keyword::In), Token::Identifier("x".to_string()),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

#[test]
fn test_let_missing_in_errors() {
    let tokens = vec![
        Token::Keyword(Keyword::Let), Token::Identifier("x".to_string()), Token::EqualSign,
        Token::IntegerLiteral(5), Token::Identifier("x".to_string()),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

// ── LetRecExpr ────────────────────────────────────────────────────────────────

#[test]
fn test_letrec_expr_structure() {
    // letrec f = fn x => f x in f: recursive self-reference in value
    let tokens = vec![
        Token::Keyword(Keyword::LetRec), Token::Identifier("f".to_string()), Token::EqualSign,
        Token::Keyword(Keyword::Fn), Token::Identifier("x".to_string()), Token::Arrow,
        Token::Identifier("f".to_string()), Token::Identifier("x".to_string()),
        Token::Keyword(Keyword::In), Token::Identifier("f".to_string()),
    ];
    match parse(tokens) {
        Expr::LetRecExpr(l) => {
            assert_eq!(l.identifier, "f");
            assert!(matches!(*l.value, Expr::FunctionDeclExpr(_)));
            assert!(matches!(*l.body_expr, Expr::CompExpr(_)));
        }
        _ => panic!("Expected LetRecExpr"),
    }
}

#[test]
fn test_letrec_missing_equal_errors() {
    let tokens = vec![
        Token::Keyword(Keyword::LetRec), Token::Identifier("f".to_string()),
        Token::Keyword(Keyword::In), Token::Identifier("f".to_string()),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

// ── IfExpr ────────────────────────────────────────────────────────────────────

#[test]
fn test_if_expr_structure() {
    // if true then 1 else 0: all three branches correctly captured
    let tokens = vec![
        Token::Keyword(Keyword::If), Token::BooleanLiteral(true),
        Token::Keyword(Keyword::Then), Token::IntegerLiteral(1),
        Token::Keyword(Keyword::Else), Token::IntegerLiteral(0),
    ];
    match parse(tokens) {
        Expr::IfExpr(i) => {
            assert!(matches!(*i.bool_expr, Expr::CompExpr(_)));
            assert!(matches!(*i.then_expr, Expr::CompExpr(_)));
            assert!(matches!(*i.else_expr, Expr::CompExpr(_)));
        }
        _ => panic!("Expected IfExpr"),
    }
}

#[test]
fn test_nested_if_else_binds_to_inner_if() {
    // if true then if false then 1 else 2 else 3
    // inner else must bind to inner if, not outer
    let tokens = vec![
        Token::Keyword(Keyword::If), Token::BooleanLiteral(true),
        Token::Keyword(Keyword::Then),
        Token::Keyword(Keyword::If), Token::BooleanLiteral(false),
        Token::Keyword(Keyword::Then), Token::IntegerLiteral(1),
        Token::Keyword(Keyword::Else), Token::IntegerLiteral(2),
        Token::Keyword(Keyword::Else), Token::IntegerLiteral(3),
    ];
    match parse(tokens) {
        Expr::IfExpr(outer) => {
            // then branch is a nested if
            assert!(matches!(*outer.then_expr, Expr::IfExpr(_)));
            // outer else is just 3, not the inner if's else
            assert!(matches!(*outer.else_expr, Expr::CompExpr(_)));
        }
        _ => panic!("Expected IfExpr"),
    }
}

#[test]
fn test_if_missing_then_errors() {
    let tokens = vec![
        Token::Keyword(Keyword::If), Token::BooleanLiteral(true),
        Token::IntegerLiteral(1), Token::Keyword(Keyword::Else), Token::IntegerLiteral(0),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

#[test]
fn test_if_missing_else_errors() {
    let tokens = vec![
        Token::Keyword(Keyword::If), Token::BooleanLiteral(true),
        Token::Keyword(Keyword::Then), Token::IntegerLiteral(1),
    ];
    assert!(Expr::new(&tokens, 0).is_err());
}

// ── CompExpr and operator chains ──────────────────────────────────────────────

#[test]
fn test_comp_expr_with_boolean_op() {
    // x < 5: chained_expr is Some with the comparison op
    let tokens = vec![
        Token::Identifier("x".to_string()),
        Token::BooleanOps(BooleanOps::LessThan),
        Token::IntegerLiteral(5),
    ];
    match parse(tokens) {
        Expr::CompExpr(c) => {
            let chained = c.chained_expr.expect("Expected ChainedCompExpr");
            assert!(matches!(chained.comp_op, BooleanOps::LessThan));
        }
        _ => panic!("Expected CompExpr"),
    }
}

#[test]
fn test_comp_expr_without_operator_has_no_chain() {
    // bare integer has no chained comparison
    let tokens = vec![Token::IntegerLiteral(5)];
    match parse(tokens) {
        Expr::CompExpr(c) => assert!(c.chained_expr.is_none()),
        _ => panic!("Expected CompExpr"),
    }
}

#[test]
fn test_add_expr_chain_structure() {
    // 1 + 2 + 3: chained add has chained_expr on the first link
    let tokens = vec![
        Token::IntegerLiteral(1),
        Token::AdditiveOps(AdditiveOps::Add),
        Token::IntegerLiteral(2),
        Token::AdditiveOps(AdditiveOps::Add),
        Token::IntegerLiteral(3),
    ];
    match parse(tokens) {
        Expr::CompExpr(c) => {
            let first_chain = c.add_expr.chained_expr.as_ref().expect("Expected ChainedAddExpr");
            assert!(first_chain.chained_expr.is_some(), "Expected second chain link");
        }
        _ => panic!("Expected CompExpr"),
    }
}

#[test]
fn test_mul_expr_chain_structure() {
    // 2 * 3 * 4: mul chain has two links
    let tokens = vec![
        Token::IntegerLiteral(2),
        Token::MultiplicativeOps(MultiplicativeOps::Mul),
        Token::IntegerLiteral(3),
        Token::MultiplicativeOps(MultiplicativeOps::Mul),
        Token::IntegerLiteral(4),
    ];
    match parse(tokens) {
        Expr::CompExpr(c) => {
            let mul_chain = c.add_expr.mul_expr.chained_expr.as_ref().expect("Expected ChainedMulExpr");
            assert!(mul_chain.chained_expr.is_some(), "Expected second mul chain link");
        }
        _ => panic!("Expected CompExpr"),
    }
}

// ── AppExpr and application chains ────────────────────────────────────────────

#[test]
fn test_app_expr_single_application() {
    // f x: atom f with one chained application of x
    let tokens = vec![
        Token::Identifier("f".to_string()),
        Token::Identifier("x".to_string()),
    ];
    match parse(tokens) {
        Expr::CompExpr(c) => {
            let app = &c.add_expr.mul_expr.app_expr;
            assert!(app.chained_expr.is_some());
            // only one argument chained
            let chained = app.chained_expr.as_ref().unwrap();
            assert!(chained.chained_expr.is_none());
        }
        _ => panic!("Expected CompExpr"),
    }
}

#[test]
fn test_app_expr_curried_application_chain() {
    // f x y: two chained applications
    let tokens = vec![
        Token::Identifier("f".to_string()),
        Token::Identifier("x".to_string()),
        Token::Identifier("y".to_string()),
    ];
    match parse(tokens) {
        Expr::CompExpr(c) => {
            let app = &c.add_expr.mul_expr.app_expr;
            let first = app.chained_expr.as_ref().expect("Expected first chain");
            assert!(first.chained_expr.is_some(), "Expected second chain for y");
        }
        _ => panic!("Expected CompExpr"),
    }
}

// ── Atom ──────────────────────────────────────────────────────────────────────

#[test]
fn test_atom_integer_literal() {
    let tokens = vec![Token::IntegerLiteral(42)];
    match parse(tokens) {
        Expr::CompExpr(c) => {
            assert!(matches!(c.add_expr.mul_expr.app_expr.atom, Atom::IntegerLiteral(42)));
        }
        _ => panic!(),
    }
}

#[test]
fn test_atom_boolean_literal() {
    let tokens = vec![Token::BooleanLiteral(true)];
    match parse(tokens) {
        Expr::CompExpr(c) => {
            assert!(matches!(c.add_expr.mul_expr.app_expr.atom, Atom::BooleanLiteral(true)));
        }
        _ => panic!(),
    }
}

#[test]
fn test_atom_identifier() {
    let tokens = vec![Token::Identifier("myVar".to_string())];
    match parse(tokens) {
        Expr::CompExpr(c) => {
            assert!(matches!(&c.add_expr.mul_expr.app_expr.atom, Atom::Identifier(s) if s == "myVar"));
        }
        _ => panic!(),
    }
}

#[test]
fn test_atom_prioritized_expr_consumes_rparen() {
    // (5) + 3: RParen must be consumed so + parses correctly
    let tokens = vec![
        Token::LParen, Token::IntegerLiteral(5), Token::RParen,
        Token::AdditiveOps(AdditiveOps::Add), Token::IntegerLiteral(3),
    ];
    match parse(tokens) {
        Expr::CompExpr(c) => {
            assert!(matches!(c.add_expr.mul_expr.app_expr.atom, Atom::PrioritizedExpr(_)));
            assert!(c.add_expr.chained_expr.is_some(), "RParen not consumed — Add chain missing");
        }
        _ => panic!("Expected CompExpr"),
    }
}

#[test]
fn test_atom_unexpected_token_errors() {
    let tokens = vec![Token::EqualSign];
    assert!(Expr::new(&tokens, 0).is_err());
}
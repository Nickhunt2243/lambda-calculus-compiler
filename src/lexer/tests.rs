use super::*;
use super::types::{Token, Keyword, AdditiveOps, MultiplicativeOps, BooleanOps};

// ── helpers ──────────────────────────────────────────────────────────────────

fn lex(input: &str) -> Vec<Token> {
    Lexer::new(input).tokenize().unwrap().clone()
}

fn lex_err(input: &str) -> String {
    Lexer::new(input).tokenize().unwrap_err()
}

// ── keywords ─────────────────────────────────────────────────────────────────

#[test]
fn test_keyword_let() {
    let tokens = lex("let");
    assert!(matches!(tokens[0], Token::Keyword(Keyword::Let)));
}

#[test]
fn test_keyword_letrec() {
    let tokens = lex("letrec");
    assert!(matches!(tokens[0], Token::Keyword(Keyword::LetRec)));
}

#[test]
fn test_keyword_let_rec() {
    let tokens = lex("let rec");
    assert!(matches!(tokens[0], Token::Keyword(Keyword::Let)));
    assert!(matches!(&tokens[1], Token::Identifier(s) if s == "rec"));
}

#[test]
fn test_keyword_fn() {
    let tokens = lex("fn");
    assert!(matches!(tokens[0], Token::Keyword(Keyword::Fn)));
}

#[test]
fn test_keyword_in() {
    let tokens = lex("in");
    assert!(matches!(tokens[0], Token::Keyword(Keyword::In)));
}

#[test]
fn test_keyword_if() {
    let tokens = lex("if");
    assert!(matches!(tokens[0], Token::Keyword(Keyword::If)));
}

#[test]
fn test_keyword_then() {
    let tokens = lex("then");
    assert!(matches!(tokens[0], Token::Keyword(Keyword::Then)));
}

#[test]
fn test_keyword_else() {
    let tokens = lex("else");
    assert!(matches!(tokens[0], Token::Keyword(Keyword::Else)));
}

// ── identifiers ──────────────────────────────────────────────────────────────

#[test]
fn test_identifier_simple() {
    let tokens = lex("x");
    assert!(matches!(&tokens[0], Token::Identifier(s) if s == "x"));
}

#[test]
fn test_identifier_multi_char() {
    let tokens = lex("myVar");
    assert!(matches!(&tokens[0], Token::Identifier(s) if s == "myVar"));
}

#[test]
fn test_identifier_with_underscore() {
    let tokens = lex("my_var");
    assert!(matches!(&tokens[0], Token::Identifier(s) if s == "my_var"));
}

#[test]
fn test_identifier_starting_with_keyword_prefix() {
    // "letx" should be an identifier, not let + x
    let tokens = lex("letx");
    assert_eq!(tokens.len(), 1);
    assert!(matches!(&tokens[0], Token::Identifier(s) if s == "letx"));
}

#[test]
fn test_identifier_starting_with_fn_prefix() {
    // "fna" should be identifier, not fn + a
    let tokens = lex("fna");
    assert_eq!(tokens.len(), 1);
    assert!(matches!(&tokens[0], Token::Identifier(s) if s == "fna"));
}

#[test]
fn test_identifier_starting_with_if_prefix() {
    let tokens = lex("iffy");
    assert_eq!(tokens.len(), 1);
    assert!(matches!(&tokens[0], Token::Identifier(s) if s == "iffy"));
}

// ── integer literals ─────────────────────────────────────────────────────────

#[test]
fn test_integer_single_digit() {
    let tokens = lex("5");
    assert!(matches!(tokens[0], Token::IntegerLiteral(5)));
}

#[test]
fn test_integer_multi_digit() {
    let tokens = lex("123");
    assert!(matches!(tokens[0], Token::IntegerLiteral(123)));
}

#[test]
fn test_integer_zero() {
    let tokens = lex("0");
    assert!(matches!(tokens[0], Token::IntegerLiteral(0)));
}

#[test]
fn test_negative_integer_at_start() {
    // '-' at the start of input is a negative number
    let tokens = lex("-5");
    assert!(matches!(tokens[0], Token::IntegerLiteral(-5)));
}

#[test]
fn test_negative_integer_after_operator() {
    // '-' after an operator is a negative number
    let tokens = lex("1 + -5");
    assert!(matches!(tokens[2], Token::IntegerLiteral(-5)));
}

#[test]
fn test_subtraction_vs_negative() {
    // '5 - 3' should produce Sub, not a negative literal
    let tokens = lex("5 - 3");
    assert!(matches!(tokens[0], Token::IntegerLiteral(5)));
    assert!(matches!(tokens[1], Token::AdditiveOps(AdditiveOps::Sub)));
    assert!(matches!(tokens[2], Token::IntegerLiteral(3)));
}

#[test]
fn test_adjacent_addition() {
    // '5 - 3' should produce Sub, not a negative literal
    let tokens = lex("5+3");
    assert!(matches!(tokens[0], Token::IntegerLiteral(5)));
    assert!(matches!(tokens[1], Token::AdditiveOps(AdditiveOps::Add)));
    assert!(matches!(tokens[2], Token::IntegerLiteral(3)));
}

#[test]
fn test_adjacent_subtraction() {
    // '5 - 3' should produce Sub, not a negative literal
    let tokens = lex("5-3");
    assert!(matches!(tokens[0], Token::IntegerLiteral(5)));
    assert!(matches!(tokens[1], Token::AdditiveOps(AdditiveOps::Sub)));
    assert!(matches!(tokens[2], Token::IntegerLiteral(3)));
}

// ── boolean literals ──────────────────────────────────────────────────────────

#[test]
fn test_boolean_true() {
    let tokens = lex("true");
    assert!(matches!(tokens[0], Token::BooleanLiteral(true)));
}

#[test]
fn test_boolean_false() {
    let tokens = lex("false");
    assert!(matches!(tokens[0], Token::BooleanLiteral(false)));
}

// ── operators ─────────────────────────────────────────────────────────────────

#[test]
fn test_additive_add() {
    let tokens = lex("+");
    assert!(matches!(tokens[0], Token::AdditiveOps(AdditiveOps::Add)));
}

#[test]
fn test_additive_sub() {
    // '-' after a value is Sub
    let tokens = lex("x - y");
    assert!(matches!(tokens[1], Token::AdditiveOps(AdditiveOps::Sub)));
}

#[test]
fn test_multiplicative_mul() {
    let tokens = lex("*");
    assert!(matches!(tokens[0], Token::MultiplicativeOps(MultiplicativeOps::Mul)));
}

#[test]
fn test_multiplicative_div() {
    let tokens = lex("/");
    assert!(matches!(tokens[0], Token::MultiplicativeOps(MultiplicativeOps::Div)));
}

#[test]
fn test_equal_sign() {
    let tokens = lex("=");
    assert!(matches!(tokens[0], Token::EqualSign));
}

#[test]
fn test_arrow() {
    let tokens = lex("=>");
    assert!(matches!(tokens[0], Token::Arrow));
}

#[test]
fn test_equality() {
    let tokens = lex("==");
    assert!(matches!(tokens[0], Token::BooleanOps(BooleanOps::Equality)));
}

#[test]
fn test_less_than() {
    let tokens = lex("<");
    assert!(matches!(tokens[0], Token::BooleanOps(BooleanOps::LessThan)));
}

#[test]
fn test_less_than_equal_to() {
    let tokens = lex("<=");
    assert!(matches!(tokens[0], Token::BooleanOps(BooleanOps::LessThanEqualTo)));
}

#[test]
fn test_greater_than() {
    let tokens = lex(">");
    assert!(matches!(tokens[0], Token::BooleanOps(BooleanOps::GreaterThan)));
}

#[test]
fn test_greater_than_equal_to() {
    let tokens = lex(">=");
    assert!(matches!(tokens[0], Token::BooleanOps(BooleanOps::GreaterThanEqualTo)));
}

// ── parens ────────────────────────────────────────────────────────────────────

#[test]
fn test_lparen() {
    let tokens = lex("(");
    assert!(matches!(tokens[0], Token::LParen));
}

#[test]
fn test_rparen() {
    let tokens = lex(")");
    assert!(matches!(tokens[0], Token::RParen));
}

// ── whitespace handling ───────────────────────────────────────────────────────

#[test]
fn test_whitespace_ignored() {
    let tokens = lex("  let   x  ");
    assert_eq!(tokens.len(), 2);
    assert!(matches!(tokens[0], Token::Keyword(Keyword::Let)));
    assert!(matches!(&tokens[1], Token::Identifier(s) if s == "x"));
}

#[test]
fn test_newline_ignored() {
    let tokens = lex("let\nx");
    assert_eq!(tokens.len(), 2);
}

#[test]
fn test_trailing_whitespace() {
    let tokens = lex("x   ");
    assert_eq!(tokens.len(), 1);
}

// ── full expressions ──────────────────────────────────────────────────────────

#[test]
fn test_let_expression() {
    // let x = 5 in x
    let tokens = lex("let x = 5 in x");
    assert_eq!(tokens.len(), 6);
    assert!(matches!(tokens[0], Token::Keyword(Keyword::Let)));
    assert!(matches!(&tokens[1], Token::Identifier(s) if s == "x"));
    assert!(matches!(tokens[2], Token::EqualSign));
    assert!(matches!(tokens[3], Token::IntegerLiteral(5)));
    assert!(matches!(tokens[4], Token::Keyword(Keyword::In)));
    assert!(matches!(&tokens[5], Token::Identifier(s) if s == "x"));
}

#[test]
fn test_fn_expression() {
    // fn x => x
    let tokens = lex("fn x => x");
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[0], Token::Keyword(Keyword::Fn)));
    assert!(matches!(&tokens[1], Token::Identifier(s) if s == "x"));
    assert!(matches!(tokens[2], Token::Arrow));
    assert!(matches!(&tokens[3], Token::Identifier(s) if s == "x"));
}

#[test]
fn test_curried_add() {
    // let add = fn x => fn y => x + y in add 3 4
    let tokens = lex("let add = fn x => fn y => x + y in add 3 4");
    assert_eq!(tokens.len(), 16);
}

#[test]
fn test_if_expression() {
    // if x then 1 else 0
    let tokens = lex("if x then 1 else 0");
    assert_eq!(tokens.len(), 6);
    assert!(matches!(tokens[0], Token::Keyword(Keyword::If)));
    assert!(matches!(tokens[2], Token::Keyword(Keyword::Then)));
    assert!(matches!(tokens[4], Token::Keyword(Keyword::Else)));
}

#[test]
fn test_parenthesized_expression() {
    let tokens = lex("(x + y)");
    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens[0], Token::LParen));
    assert!(matches!(tokens[4], Token::RParen));
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn test_unexpected_character_error() {
    let err = lex_err("@");
    assert!(err.contains("Lex Error"));
}

#[test]
fn test_unexpected_character_hash() {
    let err = lex_err("#x");
    assert!(err.contains("Lex Error"));
}
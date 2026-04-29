use super::*;
use super::types::{Token, Keyword, AdditiveOps, MultiplicativeOps, BooleanOps};

fn lex(input: &str) -> Vec<Token> {
    Lexer::new(input).tokenize().unwrap().clone()
}

fn lex_err(input: &str) -> String {
    Lexer::new(input).tokenize().unwrap_err()
}

// Every keyword in a single valid token stream
#[test]
fn test_all_keywords_tokenize_correctly() {
    let tokens = lex("let letrec fn in if then else true false");
    assert!(matches!(tokens[0], Token::Keyword(Keyword::Let)));
    assert!(matches!(tokens[1], Token::Keyword(Keyword::LetRec)));
    assert!(matches!(tokens[2], Token::Keyword(Keyword::Fn)));
    assert!(matches!(tokens[3], Token::Keyword(Keyword::In)));
    assert!(matches!(tokens[4], Token::Keyword(Keyword::If)));
    assert!(matches!(tokens[5], Token::Keyword(Keyword::Then)));
    assert!(matches!(tokens[6], Token::Keyword(Keyword::Else)));
    assert!(matches!(tokens[7], Token::BooleanLiteral(true)));
    assert!(matches!(tokens[8], Token::BooleanLiteral(false)));
    assert_eq!(tokens.len(), 9);
}

// Every symbol and operator
#[test]
fn test_all_operators_tokenize_correctly() {
    let tokens = lex("= => == + 5 - * / < <= > >=");
    assert!(matches!(tokens[0], Token::EqualSign));
    assert!(matches!(tokens[1], Token::Arrow));
    assert!(matches!(tokens[2], Token::BooleanOps(BooleanOps::Equality)));
    assert!(matches!(tokens[3], Token::AdditiveOps(AdditiveOps::Add)));
    // '5 -' — '-' after an integer is Sub, not a negative literal
    assert!(matches!(tokens[4], Token::IntegerLiteral(5)));
    assert!(matches!(tokens[5], Token::AdditiveOps(AdditiveOps::Sub)));
    assert!(matches!(tokens[6], Token::MultiplicativeOps(MultiplicativeOps::Mul)));
    assert!(matches!(tokens[7], Token::MultiplicativeOps(MultiplicativeOps::Div)));
    assert!(matches!(tokens[8], Token::BooleanOps(BooleanOps::LessThan)));
    assert!(matches!(tokens[9], Token::BooleanOps(BooleanOps::LessThanEqualTo)));
    assert!(matches!(tokens[10], Token::BooleanOps(BooleanOps::GreaterThan)));
    assert!(matches!(tokens[11], Token::BooleanOps(BooleanOps::GreaterThanEqualTo)));
}

// Negative number vs subtraction — the tricky disambiguation
#[test]
fn test_negative_number_at_stream_start() {
    let tokens = lex("-5");
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0], Token::IntegerLiteral(-5)));
}

#[test]
fn test_subtraction_after_integer() {
    // 5 - 3: '-' after a value is Sub
    let tokens = lex("5 - 3");
    assert!(matches!(tokens[0], Token::IntegerLiteral(5)));
    assert!(matches!(tokens[1], Token::AdditiveOps(AdditiveOps::Sub)));
    assert!(matches!(tokens[2], Token::IntegerLiteral(3)));
}

#[test]
fn test_negative_number_after_operator() {
    // 4 + -3: '-' after an operator is a negative literal
    let tokens = lex("4 + -3");
    assert!(matches!(tokens[0], Token::IntegerLiteral(4)));
    assert!(matches!(tokens[1], Token::AdditiveOps(AdditiveOps::Add)));
    assert!(matches!(tokens[2], Token::IntegerLiteral(-3)));
}

#[test]
fn test_subtraction_of_negative() {
    // 4 - -3: sub operator followed by negative literal
    let tokens = lex("4 - -3");
    assert!(matches!(tokens[0], Token::IntegerLiteral(4)));
    assert!(matches!(tokens[1], Token::AdditiveOps(AdditiveOps::Sub)));
    assert!(matches!(tokens[2], Token::IntegerLiteral(-3)));
}

#[test]
fn test_negative_number_after_lparen() {
    // (-5) — '-' after '(' is negative
    let tokens = lex("(-5)");
    assert!(matches!(tokens[0], Token::LParen));
    assert!(matches!(tokens[1], Token::IntegerLiteral(-5)));
    assert!(matches!(tokens[2], Token::RParen));
}

#[test]
fn test_subtraction_after_rparen() {
    // (5) - 3 — '-' after ')' is subtraction
    let tokens = lex("(5) - 3");
    assert!(matches!(tokens[0], Token::LParen));
    assert!(matches!(tokens[1], Token::IntegerLiteral(5)));
    assert!(matches!(tokens[2], Token::RParen));
    assert!(matches!(tokens[3], Token::AdditiveOps(AdditiveOps::Sub)));
    assert!(matches!(tokens[4], Token::IntegerLiteral(3)));
}

// Identifier vs keyword disambiguation — prefix collisions
#[test]
fn test_keyword_prefix_becomes_identifier() {
    // Words that start with keyword prefixes must lex as identifiers
    let tokens = lex("letx letrecs fna iffy thenx inx elsey");
    assert_eq!(tokens.len(), 7);
    for token in &tokens {
        assert!(matches!(token, Token::Identifier(_)), "Expected identifier, got {:?}", token);
    }
}

#[test]
fn test_letrec_not_let_plus_rec() {
    // "letrec" is one token, "let rec" is two
    let single = lex("letrec");
    assert_eq!(single.len(), 1);
    assert!(matches!(single[0], Token::Keyword(Keyword::LetRec)));

    let two = lex("let rec");
    assert_eq!(two.len(), 2);
    assert!(matches!(two[0], Token::Keyword(Keyword::Let)));
    assert!(matches!(&two[1], Token::Identifier(s) if s == "rec"));
}

#[test]
fn test_identifier_at_stream_boundaries() {
    // identifier at the very start and very end of stream
    let tokens = lex("myVar");
    assert_eq!(tokens.len(), 1);
    assert!(matches!(&tokens[0], Token::Identifier(s) if s == "myVar"));
}

#[test]
fn test_identifier_adjacent_to_operators_no_spaces() {
    // x+y with no spaces — should be identifier, add, identifier
    let tokens = lex("x+y");
    assert!(matches!(&tokens[0], Token::Identifier(s) if s == "x"));
    assert!(matches!(tokens[1], Token::AdditiveOps(AdditiveOps::Add)));
    assert!(matches!(&tokens[2], Token::Identifier(s) if s == "y"));
}

// Error cases
#[test]
fn test_unexpected_characters_error() {
    assert!(lex_err("@").contains("Lex Error"));
    assert!(lex_err("#").contains("Lex Error"));
    assert!(lex_err("let x = $ in x").contains("Lex Error"));
}
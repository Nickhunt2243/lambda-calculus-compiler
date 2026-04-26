// use super::infer_types::infer_expr_type;
// use super::types::{Type, TypeInference, TypeVariableGenerate};
// use crate::ast::types::*;
// use crate::lexer::types::{Token, Keyword, AdditiveOps, MultiplicativeOps, BooleanOps};
// use std::collections::HashMap;
//
// // ── helpers ───────────────────────────────────────────────────────────────────
//
// fn infer(expr: &Expr) -> Type {
//     let env = HashMap::new();
//     let mut constraints = Vec::new();
//     let mut type_gen = TypeVariableGenerate::new();
//     *infer_expr_type(expr, &env, &mut constraints, &mut type_gen).unwrap()
// }
//
// fn infer_with_env(expr: &Expr, env: HashMap<String, Box<Type>>) -> Type {
//     let mut constraints = Vec::new();
//     let mut type_gen = TypeVariableGenerate::new();
//     *infer_expr_type(expr, &env, &mut constraints, &mut type_gen).unwrap()
// }
//
// fn infer_err(expr: &Expr) -> String {
//     let env = HashMap::new();
//     let mut constraints = Vec::new();
//     let mut type_gen = TypeVariableGenerate::new();
//     infer_expr_type(expr, &env, &mut constraints, &mut type_gen).unwrap_err()
// }
//
// fn infer_constraints(expr: &Expr) -> Vec<TypeInference> {
//     let env = HashMap::new();
//     let mut constraints = Vec::new();
//     let mut type_gen = TypeVariableGenerate::new();
//     infer_expr_type(expr, &env, &mut constraints, &mut type_gen).unwrap();
//     constraints
// }
//
// fn parse(tokens: Vec<Token>) -> Expr {
//     Expr::new(&tokens, 0).unwrap().0
// }
//
// // ── atom types ────────────────────────────────────────────────────────────────
//
// #[test]
// fn test_integer_literal_infers_int() {
//     let expr = parse(vec![Token::IntegerLiteral(42)]);
//     assert!(matches!(infer(&expr), Type::IntType));
// }
//
// #[test]
// fn test_boolean_literal_true_infers_bool() {
//     let expr = parse(vec![Token::BooleanLiteral(true)]);
//     assert!(matches!(infer(&expr), Type::BoolType));
// }
//
// #[test]
// fn test_boolean_literal_false_infers_bool() {
//     let expr = parse(vec![Token::BooleanLiteral(false)]);
//     assert!(matches!(infer(&expr), Type::BoolType));
// }
//
// #[test]
// fn test_identifier_resolves_from_env() {
//     let expr = parse(vec![Token::Identifier("x".to_string())]);
//     let mut env = HashMap::new();
//     env.insert("x".to_string(), Box::new(Type::IntType));
//     assert!(matches!(infer_with_env(&expr, env), Type::IntType));
// }
//
// #[test]
// fn test_identifier_not_in_env_errors() {
//     let expr = parse(vec![Token::Identifier("x".to_string())]);
//     let err = infer_err(&expr);
//     assert!(err.contains("Variable reference prior to declaration"));
// }
//
// // ── arithmetic constraints ────────────────────────────────────────────────────
//
// #[test]
// fn test_addition_infers_int() {
//     // 1 + 2
//     let expr = parse(vec![
//         Token::IntegerLiteral(1),
//         Token::AdditiveOps(AdditiveOps::Add),
//         Token::IntegerLiteral(2),
//     ]);
//     assert!(matches!(infer(&expr), Type::IntType));
// }
//
// #[test]
// fn test_addition_generates_int_constraints() {
//     // 1 + 2 — both sides should be constrained to IntType
//     let expr = parse(vec![
//         Token::IntegerLiteral(1),
//         Token::AdditiveOps(AdditiveOps::Add),
//         Token::IntegerLiteral(2),
//     ]);
//     let constraints = infer_constraints(&expr);
//     assert!(!constraints.is_empty());
//     assert!(constraints.iter().any(|c| matches!(*c.right, Type::IntType)));
// }
//
// #[test]
// fn test_multiplication_infers_int() {
//     let expr = parse(vec![
//         Token::IntegerLiteral(3),
//         Token::MultiplicativeOps(MultiplicativeOps::Mul),
//         Token::IntegerLiteral(4),
//     ]);
//     assert!(matches!(infer(&expr), Type::IntType));
// }
//
// #[test]
// fn test_chained_addition_infers_int() {
//     // 1 + 2 + 3
//     let expr = parse(vec![
//         Token::IntegerLiteral(1),
//         Token::AdditiveOps(AdditiveOps::Add),
//         Token::IntegerLiteral(2),
//         Token::AdditiveOps(AdditiveOps::Add),
//         Token::IntegerLiteral(3),
//     ]);
//     assert!(matches!(infer(&expr), Type::IntType));
// }
//
// // ── comparison constraints ─────────────────────────────────────────────────────
//
// #[test]
// fn test_less_than_infers_bool() {
//     // 1 < 2
//     let expr = parse(vec![
//         Token::IntegerLiteral(1),
//         Token::BooleanOps(BooleanOps::LessThan),
//         Token::IntegerLiteral(2),
//     ]);
//     assert!(matches!(infer(&expr), Type::BoolType));
// }
//
// #[test]
// fn test_greater_than_infers_bool() {
//     let expr = parse(vec![
//         Token::IntegerLiteral(5),
//         Token::BooleanOps(BooleanOps::GreaterThan),
//         Token::IntegerLiteral(3),
//     ]);
//     assert!(matches!(infer(&expr), Type::BoolType));
// }
//
// #[test]
// fn test_equality_infers_bool() {
//     // 1 == 1
//     let expr = parse(vec![
//         Token::IntegerLiteral(1),
//         Token::BooleanOps(BooleanOps::Equality),
//         Token::IntegerLiteral(1),
//     ]);
//     assert!(matches!(infer(&expr), Type::BoolType));
// }
//
// #[test]
// fn test_less_than_generates_int_constraints() {
//     // x < y should constrain both sides to IntType
//     let expr = parse(vec![
//         Token::IntegerLiteral(1),
//         Token::BooleanOps(BooleanOps::LessThan),
//         Token::IntegerLiteral(2),
//     ]);
//     let constraints = infer_constraints(&expr);
//     let int_constraints = constraints.iter().filter(|c| matches!(*c.right, Type::IntType)).count();
//     assert_eq!(int_constraints, 2);
// }
//
// // ── function declaration ──────────────────────────────────────────────────────
//
// #[test]
// fn test_fn_infers_func_type() {
//     // fn x => x
//     let expr = parse(vec![
//         Token::Keyword(Keyword::Fn),
//         Token::Identifier("x".to_string()),
//         Token::Arrow,
//         Token::Identifier("x".to_string()),
//     ]);
//     assert!(matches!(infer(&expr), Type::FuncType(_)));
// }
//
// #[test]
// fn test_fn_body_int_infers_func_to_int() {
//     // fn x => 5  — return type should be IntType
//     let expr = parse(vec![
//         Token::Keyword(Keyword::Fn),
//         Token::Identifier("x".to_string()),
//         Token::Arrow,
//         Token::IntegerLiteral(5),
//     ]);
//     match infer(&expr) {
//         Type::FuncType(func) => {
//             assert!(matches!(*func.return_type, Type::IntType));
//         }
//         _ => panic!("Expected FuncType"),
//     }
// }
//
// #[test]
// fn test_curried_fn_infers_nested_func_type() {
//     // fn x => fn y => x  — should be FuncType(_, FuncType(_, _))
//     let expr = parse(vec![
//         Token::Keyword(Keyword::Fn),
//         Token::Identifier("x".to_string()),
//         Token::Arrow,
//         Token::Keyword(Keyword::Fn),
//         Token::Identifier("y".to_string()),
//         Token::Arrow,
//         Token::Identifier("x".to_string()),
//     ]);
//     match infer(&expr) {
//         Type::FuncType(outer) => {
//             assert!(matches!(*outer.return_type, Type::FuncType(_)));
//         }
//         _ => panic!("Expected nested FuncType"),
//     }
// }
//
// // ── let expression ────────────────────────────────────────────────────────────
//
// #[test]
// fn test_let_infers_body_type() {
//     // let x = 5 in x — body returns x which is IntType
//     let expr = parse(vec![
//         Token::Keyword(Keyword::Let),
//         Token::Identifier("x".to_string()),
//         Token::EqualSign,
//         Token::IntegerLiteral(5),
//         Token::Keyword(Keyword::In),
//         Token::Identifier("x".to_string()),
//     ]);
//     assert!(matches!(infer(&expr), Type::IntType));
// }
//
// #[test]
// fn test_let_binding_available_in_body() {
//     // let x = 5 in x + 1
//     let expr = parse(vec![
//         Token::Keyword(Keyword::Let),
//         Token::Identifier("x".to_string()),
//         Token::EqualSign,
//         Token::IntegerLiteral(5),
//         Token::Keyword(Keyword::In),
//         Token::Identifier("x".to_string()),
//         Token::AdditiveOps(AdditiveOps::Add),
//         Token::IntegerLiteral(1),
//     ]);
//     assert!(matches!(infer(&expr), Type::IntType));
// }
//
// #[test]
// fn test_let_binding_not_visible_outside() {
//     // let x = 5 in 1   then try x alone — x should not be in scope
//     let body_expr = parse(vec![Token::Identifier("x".to_string())]);
//     let err = infer_err(&body_expr);
//     assert!(err.contains("Variable reference prior to declaration"));
// }
//
// // ── if expression ─────────────────────────────────────────────────────────────
//
// #[test]
// fn test_if_infers_branch_type() {
//     // if true then 1 else 0
//     let expr = parse(vec![
//         Token::Keyword(Keyword::If),
//         Token::BooleanLiteral(true),
//         Token::Keyword(Keyword::Then),
//         Token::IntegerLiteral(1),
//         Token::Keyword(Keyword::Else),
//         Token::IntegerLiteral(0),
//     ]);
//     assert!(matches!(infer(&expr), Type::IntType));
// }
//
// #[test]
// fn test_if_generates_bool_constraint_on_condition() {
//     // if true then 1 else 0 — condition should be constrained to BoolType
//     let expr = parse(vec![
//         Token::Keyword(Keyword::If),
//         Token::BooleanLiteral(true),
//         Token::Keyword(Keyword::Then),
//         Token::IntegerLiteral(1),
//         Token::Keyword(Keyword::Else),
//         Token::IntegerLiteral(0),
//     ]);
//     let constraints = infer_constraints(&expr);
//     assert!(constraints.iter().any(|c| matches!(*c.right, Type::BoolType)));
// }
//
// #[test]
// fn test_if_generates_branch_equality_constraint() {
//     // if true then 1 else 0 — then and else should be constrained equal
//     let expr = parse(vec![
//         Token::Keyword(Keyword::If),
//         Token::BooleanLiteral(true),
//         Token::Keyword(Keyword::Then),
//         Token::IntegerLiteral(1),
//         Token::Keyword(Keyword::Else),
//         Token::IntegerLiteral(0),
//     ]);
//     let constraints = infer_constraints(&expr);
//     // should have at least 2 constraints: cond==bool, then==else
//     assert!(constraints.len() >= 2);
// }
//
// // ── letrec ────────────────────────────────────────────────────────────────────
//
// #[test]
// fn test_letrec_generates_recursive_constraint() {
//     // letrec f = fn x => x in f 5
//     let expr = parse(vec![
//         Token::Keyword(Keyword::LetRec),
//         Token::Identifier("f".to_string()),
//         Token::EqualSign,
//         Token::Keyword(Keyword::Fn),
//         Token::Identifier("x".to_string()),
//         Token::Arrow,
//         Token::Identifier("x".to_string()),
//         Token::Keyword(Keyword::In),
//         Token::Identifier("f".to_string()),
//         Token::IntegerLiteral(5),
//     ]);
//     // letrec should produce a constraint tying f's type var to its value type
//     let constraints = infer_constraints(&expr);
//     assert!(!constraints.is_empty());
// }
//
// // ── type variable generator ───────────────────────────────────────────────────
//
// #[test]
// fn test_type_var_generator_produces_unique_names() {
//     use super::types::TypeVariableGenerate;
//     let mut type_gen = TypeVariableGenerate::new();
//     let a = type_gen.next();
//     let b = type_gen.next();
//     let c = type_gen.next();
//     assert_ne!(a.name, b.name);
//     assert_ne!(b.name, c.name);
//     assert_ne!(a.name, c.name);
// }
//
// #[test]
// fn test_type_var_generator_starts_with_a() {
//     use super::types::TypeVariableGenerate;
//     let mut type_gen = TypeVariableGenerate::new();
//     let first = type_gen.next();
//     assert_eq!(first.name, "'a");
// }
//
// #[test]
// fn test_type_var_generator_second_is_b() {
//     use super::types::TypeVariableGenerate;
//     let mut type_gen = TypeVariableGenerate::new();
//     type_gen.next();
//     let second = type_gen.next();
//     assert_eq!(second.name, "'b");
// }
//
// // ── occurs check ─────────────────────────────────────────────────────────────
//
// #[test]
// fn test_primitive_not_callable_errors() {
//     // 5 x — applying an integer as a function
//     let expr = parse(vec![
//         Token::IntegerLiteral(5),
//         Token::Identifier("x".to_string()),
//     ]);
//     let mut env = HashMap::new();
//     env.insert("x".to_string(), Box::new(Type::IntType));
//     let mut constraints = Vec::new();
//     let mut type_gen = TypeVariableGenerate::new();
//     let result = infer_expr_type(&expr, &env, &mut constraints, &mut type_gen);
//     assert!(result.is_err());
//     assert!(result.unwrap_err().contains("not callable"));
// }

use super::type_inference;
use super::types::{FinalType, FinalFuncType};
use crate::lexer::Lexer;
use crate::ast::parse;

// ── helpers ───────────────────────────────────────────────────────────────────

fn run(src: &str) -> FinalType {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize().unwrap();
    let ast = parse(tokens).unwrap();
    type_inference(&ast).unwrap()
}

fn run_err(src: &str) -> String {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize().unwrap();
    let ast = parse(tokens).unwrap();
    type_inference(&ast).unwrap_err()
}

fn is_int(t: &FinalType) -> bool {
    matches!(t, FinalType::IntType)
}

fn is_bool(t: &FinalType) -> bool {
    matches!(t, FinalType::BoolType)
}

fn is_func(t: &FinalType) -> bool {
    matches!(t, FinalType::FuncType(_))
}

// ── literals ──────────────────────────────────────────────────────────────────

#[test]
fn test_integer_literal() {
    assert!(is_int(&run("5")));
}

#[test]
fn test_negative_integer_literal() {
    assert!(is_int(&run("-5")));
}

#[test]
fn test_boolean_literal_true() {
    assert!(is_bool(&run("true")));
}

#[test]
fn test_boolean_literal_false() {
    assert!(is_bool(&run("false")));
}

// ── arithmetic ────────────────────────────────────────────────────────────────

#[test]
fn test_addition() {
    assert!(is_int(&run("1 + 2")));
}

#[test]
fn test_subtraction() {
    assert!(is_int(&run("5 - 3")));
}

#[test]
fn test_multiplication() {
    assert!(is_int(&run("3 * 4")));
}

#[test]
fn test_division() {
    assert!(is_int(&run("10 / 2")));
}

#[test]
fn test_chained_addition() {
    assert!(is_int(&run("1 + 2 + 3")));
}

#[test]
fn test_chained_multiplication() {
    assert!(is_int(&run("2 * 3 * 4")));
}

#[test]
fn test_mixed_arithmetic() {
    assert!(is_int(&run("1 + 2 * 3")));
}

// ── comparisons ───────────────────────────────────────────────────────────────

#[test]
fn test_less_than() {
    assert!(is_bool(&run("1 < 2")));
}

#[test]
fn test_greater_than() {
    assert!(is_bool(&run("5 > 3")));
}

#[test]
fn test_less_than_equal_to() {
    assert!(is_bool(&run("3 <= 3")));
}

#[test]
fn test_greater_than_equal_to() {
    assert!(is_bool(&run("4 >= 2")));
}

#[test]
fn test_equality_integers() {
    assert!(is_bool(&run("1 == 1")));
}

#[test]
fn test_equality_booleans() {
    assert!(is_bool(&run("true == false")));
}

// ── let expressions ───────────────────────────────────────────────────────────

#[test]
fn test_let_integer_binding() {
    assert!(is_int(&run("let x = 5 in x")));
}

#[test]
fn test_let_boolean_binding() {
    assert!(is_bool(&run("let x = true in x")));
}

#[test]
fn test_let_arithmetic_body() {
    assert!(is_int(&run("let x = 5 in x + 1")));
}

#[test]
fn test_let_nested() {
    assert!(is_int(&run("let x = 5 in let y = 3 in x + y")));
}

#[test]
fn test_let_binding_is_function() {
    assert!(is_int(&run("let f = fn x => x + 1 in f 5")));
}

#[test]
fn test_let_body_uses_comparison() {
    assert!(is_bool(&run("let x = 5 in x < 10")));
}

// ── function declarations ─────────────────────────────────────────────────────

#[test]
fn test_identity_function() {
    assert!(is_func(&run("fn x => x")));
}

#[test]
fn test_fn_returning_int() {
    let t = run("fn x => 5");
    match t {
        FinalType::FuncType(f) => assert!(is_int(&f.return_type)),
        _ => panic!("Expected FuncType"),
    }
}

#[test]
fn test_fn_returning_bool() {
    let t = run("fn x => true");
    match t {
        FinalType::FuncType(f) => assert!(is_bool(&f.return_type)),
        _ => panic!("Expected FuncType"),
    }
}

#[test]
fn test_curried_function_nested_type() {
    // fn x => fn y => x + y should be int -> int -> int
    let t = run("fn x => fn y => x + y");
    match t {
        FinalType::FuncType(outer) => {
            assert!(is_int(&outer.param_type));
            match *outer.return_type {
                FinalType::FuncType(inner) => {
                    assert!(is_int(&inner.param_type));
                    assert!(is_int(&inner.return_type));
                }
                _ => panic!("Expected inner FuncType"),
            }
        }
        _ => panic!("Expected FuncType"),
    }
}

#[test]
fn test_fn_param_constrained_to_int_by_arithmetic() {
    // fn x => x + 1 — param must be int
    let t = run("fn x => x + 1");
    match t {
        FinalType::FuncType(f) => {
            assert!(is_int(&f.param_type));
            assert!(is_int(&f.return_type));
        }
        _ => panic!("Expected FuncType"),
    }
}

#[test]
fn test_fn_param_constrained_to_int_by_comparison() {
    // fn x => x < 5 — param is int, return is bool
    let t = run("fn x => x < 5");
    match t {
        FinalType::FuncType(f) => {
            assert!(is_int(&f.param_type));
            assert!(is_bool(&f.return_type));
        }
        _ => panic!("Expected FuncType"),
    }
}

// ── function application ──────────────────────────────────────────────────────

#[test]
fn test_apply_identity_to_int() {
    assert!(is_int(&run("(fn x => x) 5")));
}

#[test]
fn test_apply_identity_to_bool() {
    assert!(is_bool(&run("(fn x => x) true")));
}

#[test]
fn test_apply_add_one() {
    let t = run("let x = fn x => x + 1 in x 5");
    println!("{:?}", t);
    assert!(is_int(&t));
}

#[test]
fn test_apply_curried_add() {
    assert!(is_int(&run("let add = fn x => fn y => x + y in add 3 4")));
}

#[test]
fn test_partial_application_returns_func() {
    // add 3 should return int -> int
    let t = run("let add = fn x => fn y => x + y in add 3");
    match t {
        FinalType::FuncType(f) => {
            assert!(is_int(&f.param_type));
            assert!(is_int(&f.return_type));
        }
        _ => panic!("Expected FuncType from partial application"),
    }
}

// ── if expressions ────────────────────────────────────────────────────────────

#[test]
fn test_if_int_branches() {
    assert!(is_int(&run("if true then 1 else 0")));
}

#[test]
fn test_if_bool_branches() {
    assert!(is_bool(&run("if true then true else false")));
}

#[test]
fn test_if_with_comparison_condition() {
    assert!(is_int(&run("if 1 < 2 then 10 else 20")));
}

#[test]
fn test_if_with_let_in_branch() {
    assert!(is_int(&run("if true then let x = 5 in x else 0")));
}

#[test]
fn test_nested_if() {
    assert!(is_int(&run("if true then if false then 1 else 2 else 3")));
}

#[test]
fn test_if_condition_must_be_bool() {
    // condition is constrained to bool — using an int comparison satisfies this
    assert!(is_bool(&run("if 1 == 1 then true else false")));
}

// ── letrec ────────────────────────────────────────────────────────────────────

#[test]
fn test_letrec_identity() {
    assert!(is_int(&run("letrec f = fn x => x in f 5")));
}

#[test]
fn test_letrec_returns_function() {
    assert!(is_func(&run("letrec f = fn x => x + 1 in f")));
}

#[test]
fn test_letrec_factorial_structure() {
    assert!(is_int(&run(
        "letrec fact = fn n => if n < 2 then 1 else n * fact (n - 1) in fact 5"
    )));
}

#[test]
fn test_letrec_body_type_is_return_type() {
    // letrec f = fn x => x in 42 — body is just 42, not f
    assert!(is_int(&run("letrec f = fn x => x in 42")));
}

// ── complex programs ──────────────────────────────────────────────────────────

#[test]
fn test_compose_two_functions() {
    assert!(is_int(&run(
        "let double = fn x => x * 2 in let inc = fn x => x + 1 in double (inc 3)"
    )));
}

#[test]
fn test_higher_order_apply() {
    // let apply = fn f => fn x => f x in apply (fn x => x + 1) 5
    assert!(is_int(&run(
        "let apply = fn f => fn x => f x in apply (fn x => x + 1) 5"
    )));
}

#[test]
fn test_boolean_predicate_function() {
    assert!(is_bool(&run("let is_positive = fn x => x > 0 in is_positive 5")));
}

#[test]
fn test_deeply_nested_let() {
    assert!(is_int(&run(
        "let a = 1 in let b = 2 in let c = 3 in a + b + c"
    )));
}

#[test]
fn test_function_returning_function() {
    // let make_adder = fn x => fn y => x + y in make_adder 5
    let t = run("let make_adder = fn x => fn y => x + y in make_adder 5");
    match t {
        FinalType::FuncType(f) => {
            assert!(is_int(&f.param_type));
            assert!(is_int(&f.return_type));
        }
        _ => panic!("Expected FuncType"),
    }
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn test_unbound_variable_errors() {
    let err = run_err("x");
    assert!(err.contains("Variable reference prior to declaration"));
}

#[test]
fn test_unbound_variable_in_body_errors() {
    let err = run_err("let x = 5 in y");
    assert!(err.contains("Variable reference prior to declaration"));
}

#[test]
fn test_int_bool_branch_mismatch_errors() {
    // then branch is int, else branch is bool — must unify
    let err = run_err("if true then 1 else true");
    assert!(err.contains("Failed to unify"));
}

#[test]
fn test_applying_int_as_function_errors() {
    let err = run_err("5 3");
    assert!(err.contains("not callable"));
}

#[test]
fn test_applying_bool_as_function_errors() {
    let err = run_err("true 5");
    assert!(err.contains("not callable"));
}

#[test]
fn test_arithmetic_on_bool_errors() {
    // true + 1 — bool can't unify with int in arithmetic
    let err = run_err("true + 1");
    assert!(err.contains("Failed to unify"));
}
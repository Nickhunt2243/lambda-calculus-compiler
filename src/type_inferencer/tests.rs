use super::type_inference;
use super::types::FinalType;
use crate::lexer::Lexer;
use crate::ast::parse;

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

fn is_int(t: &FinalType) -> bool { matches!(t, FinalType::IntType) }
fn is_bool(t: &FinalType) -> bool { matches!(t, FinalType::BoolType) }

// ── Function type inference ───────────────────────────────────────────────────

// fn x => x + 1: param constrained to int through arithmetic
#[test]
fn test_fn_param_constrained_by_arithmetic() {
    match run("fn x => x + 1") {
        FinalType::FuncType(f) => {
            assert!(is_int(&f.param_type));
            assert!(is_int(&f.return_type));
        }
        _ => panic!("Expected FuncType"),
    }
}

// fn x => x < 5: param int, return bool
#[test]
fn test_fn_param_constrained_by_comparison() {
    match run("fn x => x < 5") {
        FinalType::FuncType(f) => {
            assert!(is_int(&f.param_type));
            assert!(is_bool(&f.return_type));
        }
        _ => panic!("Expected FuncType"),
    }
}

// fn x => fn y => x + y: int -> int -> int (fully resolved)
#[test]
fn test_curried_fn_fully_resolves() {
    match run("fn x => fn y => x + y") {
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

// fn x => x: unconstrained param is polymorphic
#[test]
fn test_fn_unconstrained_param_is_polymorphic() {
    match run("fn x => x") {
        FinalType::FuncType(f) => assert!(matches!(*f.param_type, FinalType::Polymorphic(_))),
        _ => panic!("Expected FuncType"),
    }
}

// ── Deeply nested inference ───────────────────────────────────────────────────

// let with fn value and if in body
#[test]
fn test_let_fn_with_if_body() {
    assert!(is_int(&run(
        "let f = fn x => x + 1 in if f 3 < 10 then f 5 else f 0"
    )));
}

// nested let with arithmetic and comparison
#[test]
fn test_deeply_nested_let_with_conditionals() {
    assert!(is_bool(&run(
        "let x = 5 in let y = x + 3 in let z = y * 2 in z > 10"
    )));
}

// fn body containing let and if
#[test]
fn test_fn_body_with_let_and_if() {
    assert!(is_int(&run(
        "let clamp = fn x => let lo = 0 in let hi = 100 in if x < lo then lo else if x > hi then hi else x in clamp 50"
    )));
}

// triple curried application
#[test]
fn test_triple_curried_application() {
    assert!(is_int(&run(
        "let f = fn x => fn y => fn z => x + y + z in f 1 2 3"
    )));
}

// ── letrec ────────────────────────────────────────────────────────────────────

// factorial: recursive, uses comparison, multiplication, subtraction
#[test]
fn test_letrec_factorial() {
    assert!(is_int(&run(
        "letrec fact = fn n => if n < 2 then 1 else n * fact (n - 1) in fact 5"
    )));
}

// sum: different recursive structure using addition
#[test]
fn test_letrec_sum() {
    assert!(is_int(&run(
        "letrec sum = fn n => if n < 1 then 0 else n + sum (n - 1) in sum 10"
    )));
}

// letrec body type is independent of the recursive function's type
#[test]
fn test_letrec_body_type_independent_of_fn() {
    assert!(is_bool(&run(
        "letrec f = fn x => x + 1 in 5 > 3"
    )));
}

// letrec with wrong argument type should fail
#[test]
fn test_letrec_wrong_arg_type_errors() {
    let err = run_err(
        "letrec fact = fn n => if n < 2 then 1 else n * fact (n - 1) in fact true"
    );
    assert!(err.contains("Failed to unify"));
}

// ── polymorphic let ───────────────────────────────────────────────────────────

// identity applied to int and bool in same expression
#[test]
fn test_let_polymorphic_identity_both_types() {
    assert!(is_int(&run(
        "let id = fn x => x in if id true then id 5 else id 0"
    )));
}

// higher order apply used at two different types
#[test]
fn test_let_polymorphic_apply_different_types() {
    assert!(is_int(&run(
        "let apply = fn f => fn x => f x in
         let b = apply (fn x => x) true in
         apply (fn x => x + 1) 5"
    )));
}

// compose: tests that type variables thread correctly through multiple polymorphic uses
#[test]
fn test_let_polymorphic_compose() {
    assert!(is_int(&run(
        "let compose = fn f => fn g => fn x => f (g x) in
         compose (fn x => x + 1) (fn x => x * 2) 3"
    )));
}

// outer binding accessible after inner shadow
#[test]
fn test_let_shadow_outer_accessible_before_shadow() {
    assert!(is_int(&run(
        "let id = fn x => x in let a = id 5 in let id = fn x => true in a"
    )));
}

// monomorphic constraint correctly rejects wrong type
#[test]
fn test_let_monomorphic_rejects_wrong_type() {
    let err = run_err("let f = fn x => x + 1 in f true");
    assert!(err.contains("Failed to unify"));
}

// ∀a. a -> int applied to both int and bool in arithmetic
#[test]
fn test_polymorphic_const_used_in_arithmetic() {
    assert!(is_int(&run(
        "let const_one = fn x => 1 in const_one true + const_one 5"
    )));
}

// shadowed binding with different type
#[test]
fn test_polymorphic_shadow_changes_type() {
    assert!(is_bool(&run(
        "let id = fn x => x in let id = fn x => true in id 5"
    )));
}

// polymorphic fn applied in both if branches with different types
#[test]
fn test_polymorphic_in_if_branches() {
    assert!(is_bool(&run(
        "let id = fn x => x in if true then id true else id false"
    )));
}

// ── if type constraints ───────────────────────────────────────────────────────

// condition must be bool — int condition fails
#[test]
fn test_if_int_condition_errors() {
    let err = run_err("if 5 then 1 else 0");
    assert!(err.contains("Failed to unify"));
}

// branch type mismatch fails even when nested
#[test]
fn test_if_nested_branch_mismatch_errors() {
    let err = run_err("if true then (if false then 1 else true) else 0");
    assert!(err.contains("Failed to unify"));
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn test_unbound_variable_errors() {
    assert!(run_err("x").contains("Variable reference prior to declaration"));
}

#[test]
fn test_variable_out_of_scope_errors() {
    assert!(run_err("let x = 5 in y").contains("Variable reference prior to declaration"));
}

#[test]
fn test_int_applied_as_function_errors() {
    assert!(run_err("5 3").contains("not callable"));
}

#[test]
fn test_bool_applied_as_function_errors() {
    assert!(run_err("true 5").contains("not callable"));
}

#[test]
fn test_bool_in_arithmetic_errors() {
    assert!(run_err("true + 1").contains("Failed to unify"));
}

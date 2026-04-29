# Lambda Calculus Compiler

A compiler frontend for a small lambda calculus language, written in Rust. Currently implements a full pipeline from source text through to Hindley-Milner type inference, with LLVM code generation up next.

---

## Inspiration
### RIT CSCI 742 Compiler Construction
In 2024 I took a graduate-level compiler construction course at RIT that
covered the full pipeline from lexing and parsing through to type inference.
We worked from the Dragon Book and implemented a compiler featuring an LR(1)
parser, unification, and Hindley-Milner type inference. This project is a
continuation of that work — rebuilding the ideas I found most interesting
from scratch in Rust, and pushing further into code generation with LLVM.

---

## Pipeline

```
Source Text → Lexer → Parser (AST) → Type Inferencer → [LLVM Codegen — coming soon]
```

---

## Features

### Lexer
Tokenises the source language, supporting integer and boolean literals, arithmetic and comparison operators, and keywords for `let`, `letrec`, `if/then/else`, and lambda expressions (`fn`).

### Parser
Produces a typed AST with explicit expression nodes for function declarations, application, let/let-rec bindings, conditionals, and arithmetic. The grammar handles operator precedence correctly through layered production rules (comparison → additive → multiplicative → application → atom).

### Type Inferencer — Algorithm W (Hindley-Milner)

The type inferencer is a full implementation of **Algorithm W**, the classic constraint-based type inference algorithm used in ML and Haskell. It infers principal types for expressions without requiring any annotations from the programmer.

The implementation covers:

- **Constraint generation** — each expression node is walked recursively, generating equality constraints between type variables as it goes
- **Unification** — constraints are solved via Robinson's unification algorithm, substituting type variables until a consistent solution is found or a type error is reported
- **Let-polymorphism** — `let` bindings are fully generalised before being added to the environment, meaning a bound function can be used at multiple different types in the same scope (e.g. `let id = fn x => x in ...`)
- **`letrec` / recursive bindings** — handled by introducing a fresh type variable for the recursive name before inferring the body, then unifying it with the inferred type, allowing self-referential functions to type-check correctly
- **Occurs check** — prevents constructing infinite types (e.g. `'a = 'a -> 'b`) by checking that a type variable does not appear within the type it is being unified with
- **Generalization** and **Instantiation** — polymorphic type schemes are instantiated with fresh type variables at each use site, preserving soundness

The result is a `FinalType` — either a concrete type (`Int`, `Bool`, a function type), or a `Polymorphic` type variable for genuinely unconstrained parameters.

A comprehensive test suite covers function inference, let-polymorphism, recursive bindings, higher-order functions, and expected type errors.

---

## Coming Soon — LLVM Code Generation

The next stage is to lower the typed AST into LLVM IR using the `inkwell` Rust bindings. The plan is to start with the straightforward cases (integer arithmetic, conditionals, function definitions and calls) and build up to closures and polymorphic dispatch.

---

## Language Syntax (Quick Reference)

```
let double = fn x => x * 2 in double 5

let rec fact = fn n => if n < 2 then 1 else n * fact (n - 1) in fact 10

let apply = fn f => fn x => f x in apply (fn x => x) true
```

## Disclaimer

Claude was used as a learning aid during development — to discuss concepts, check understanding, and explore edge cases — but not for code generation. All implementation is my own.
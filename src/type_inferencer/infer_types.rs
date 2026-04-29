use crate::ast::types::{LetExpr, IfExpr, FunctionDeclExpr, CompExpr, AddExpr, MulExpr, AppExpr, Atom, Expr, LetRecExpr};
use std::collections::{HashMap, HashSet};
use crate::lexer::types::BooleanOps;
use crate::type_inferencer::types::{TypeVariable, Type, FuncType, TypeInference, TypeVariableGenerate, TypeScheme};
use crate::type_inferencer::unification::unification;

fn occurs_in(left_type: &TypeVariable, right_type: &Type) -> Result<(), String> {
    match right_type {
        Type::IntType | Type::BoolType => Ok(()),
        Type::FuncType(func_type) => {
            occurs_in(left_type, &func_type.param_type)
                .map_err(|_| format!("Infinite type: {:?} in {:?}", left_type, right_type))?;
            occurs_in(left_type, &func_type.return_type)
                .map_err(|_| format!("Infinite type: {:?} in {:?}", left_type, right_type))?;
            Ok(())
        },
        Type::TypeVariable(right_type_var) => {
            if left_type.name == right_type_var.name {
                return Err("Infinite type variables are not allowed".to_string());
            }
            Ok(())
        }
    }
}

fn free_type_vars(body_type: &Type, quantified_vars: &HashSet<TypeVariable>) -> HashSet<TypeVariable> {
    match body_type {
        Type::IntType | Type::BoolType => HashSet::new(),
        Type::TypeVariable(t) => HashSet::from([t.clone()]).difference(quantified_vars).cloned().collect(),
        Type::FuncType(func_type) => {
            let mut vars = free_type_vars(&func_type.param_type, quantified_vars);
            vars.extend(free_type_vars(&func_type.return_type, quantified_vars));
            vars
        }
    }
}

fn free_type_vars_in_env(env: &HashMap<String, TypeScheme>) -> HashSet<TypeVariable> {
    env.values()
        .flat_map(|scheme| free_type_vars(&scheme.body_type, &scheme.quantified_vars.iter().cloned().collect()))
        .collect()
}

fn generalize(body_type: Type, env: &HashMap<String, TypeScheme>) -> TypeScheme {
    let free_body_vars = free_type_vars(&body_type, &HashSet::new());
    let free_env_vars = free_type_vars_in_env(env);
    TypeScheme {
        quantified_vars: free_body_vars.difference(&free_env_vars).cloned().collect(),
        body_type: Box::new(body_type)
    }
}

pub fn apply_simple_substitutions(return_type: Type, substitutions: HashMap<TypeVariable, Type>) -> Result<Type, String> {
    match return_type {
        Type::IntType => Ok(Type::IntType),
        Type::BoolType => Ok(Type::BoolType),
        Type::FuncType(func_type) => {
            let param_type = apply_simple_substitutions(*func_type.param_type, substitutions.clone())?;
            let return_type = apply_simple_substitutions(*func_type.return_type, substitutions)?;
            Ok(Type::FuncType(
                Box::new(FuncType {
                    param_type: Box::new(param_type),
                    return_type: Box::new(return_type)
                })
            ))
        },
        Type::TypeVariable(type_variable) => {
            match substitutions.get(&type_variable) {
                Some(return_type) => {
                    apply_simple_substitutions(return_type.clone(), substitutions.clone())
                }
                None => Ok(Type::TypeVariable(type_variable.clone()))
            }
        }
    }
}

fn instantiate(scheme: TypeScheme, type_var_generator: &mut TypeVariableGenerate) -> Result<Box<Type>, String> {
    let mut type_substitutions: HashMap<TypeVariable, Type> = HashMap::new();
    for quantified_type in scheme.quantified_vars {
        type_substitutions.insert(quantified_type, Type::TypeVariable(type_var_generator.next()));
    }

    let substituted_var = apply_simple_substitutions(*scheme.body_type.clone(), type_substitutions)?;
    Ok(Box::new(substituted_var))
}

pub fn infer_expr_type<>(
    ast: &Expr,
    env: &HashMap<String, TypeScheme>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    match ast {
        Expr::FunctionDeclExpr(func_expr) => infer_func_decl_type(&func_expr, env, inferred_types, type_var_generator),
        Expr::LetExpr(let_expr) => infer_let_type(let_expr, env, inferred_types, type_var_generator),
        Expr::LetRecExpr(let_rec_expr) => infer_let_rec_type(let_rec_expr, env, inferred_types, type_var_generator),
        Expr::IfExpr(if_expr) => infer_if_type(if_expr, env, inferred_types, type_var_generator),
        Expr::CompExpr(comp_expr) => infer_comp_type(comp_expr, env, inferred_types, type_var_generator)
    }
}

fn infer_func_decl_type(
    ast: &FunctionDeclExpr,
    env: &HashMap<String, TypeScheme>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    let new_param_type_variable = type_var_generator.next();
    let mut local_env = env.clone();
    local_env.insert(ast.param.clone(), TypeScheme::from_type_var(new_param_type_variable.clone()));
    let body_type = infer_expr_type(&ast.body_expr, &local_env, inferred_types, type_var_generator)?;

    Ok(Box::new(Type::FuncType(Box::new(FuncType {
        param_type: Box::new(Type::TypeVariable(new_param_type_variable)),
        return_type: body_type
    }))))
}
fn infer_let_type(
    ast: &LetExpr,
    env: &HashMap<String, TypeScheme>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    let snapshot_idx = inferred_types.len();
    let mut local_env = env.clone();
    let identifier_type = infer_expr_type(&ast.value, env, inferred_types, type_var_generator)?;
    let mut snapshot_inferred_types: Vec<TypeInference> = inferred_types.drain(snapshot_idx..).collect();

    let substitutions = unification(&mut snapshot_inferred_types)?;
    let final_func_type = apply_simple_substitutions(*identifier_type, substitutions)?;
    let type_scheme = generalize(final_func_type, env);

    local_env.insert(ast.identifier.clone(), type_scheme);
    infer_expr_type(&ast.body_expr, &local_env, inferred_types, type_var_generator)
}
/**
let apply =
    fn f =>
        fn x => f x in

    apply (fn x => x) true

x                  -> 'b                    {f: 'a, x: 'b} []
f                  -> 'b => 'c              {f: 'a, x: 'b} []
f x                -> 'c                    {f: 'a, x: 'b} [('a, 'b => 'c)]
fn x => f x        -> 'b => 'c              {f: 'a} [('a, 'b => 'c)]
fn f = fn x => f x -> 'a => 'b => 'c        {f: } [('a, 'b => 'c), ()]


apply => type(fn f => fn x => f x), apply = 'c => 'a => 'b
f     => type(fn x => f x)        , f = 'a => 'b

*/
fn infer_let_rec_type(
    ast: &LetRecExpr,
    env: &HashMap<String, TypeScheme>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    let rec_type_var = type_var_generator.next();
    let mut local_env = env.clone();
    local_env.insert(ast.identifier.clone(), TypeScheme::from_type_var(rec_type_var.clone()));

    // snapshot before inferring value
    let snapshot_idx = inferred_types.len();

    let identifier_type = infer_expr_type(&ast.value, &local_env, inferred_types, type_var_generator)?;

    // add the recursive constraint: 'a = inferred type of value
    inferred_types.push(TypeInference {
        left: Box::new(Type::TypeVariable(rec_type_var.clone())),
        right: identifier_type.clone()
    });

    // Drain all inferred types post inferring value expr
    let mut value_constraints: Vec<TypeInference> = inferred_types.drain(snapshot_idx..).collect();
    // Unify against possible constraints
    let substitutions = unification(&mut value_constraints)?;
    // find what the value expr resolved to — that's the actual type of f
    let final_type = apply_simple_substitutions(
        *identifier_type,
        substitutions.clone()
    )?;
    // Generalize the final type of f
    let type_scheme = generalize(final_type, env);
    // Insert type scheme into local env :)
    local_env.insert(ast.identifier.clone(), type_scheme);

    infer_expr_type(&ast.body_expr, &local_env, inferred_types, type_var_generator)
}
fn infer_if_type(
    ast: &IfExpr,
    env: &HashMap<String, TypeScheme>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    let bool_type = infer_expr_type(&ast.bool_expr, env, inferred_types, type_var_generator)?;
    let then_type = infer_expr_type(&ast.then_expr, env, inferred_types, type_var_generator)?;
    let else_type = infer_expr_type(&ast.else_expr, env, inferred_types, type_var_generator)?;

    inferred_types.push(TypeInference {left: bool_type, right: Box::new(Type::BoolType)});

    match &*then_type {
        Type::TypeVariable(then_type_var) => occurs_in(&then_type_var, &*else_type)?,
        _ => {}
    }
    match &*else_type {
        Type::TypeVariable(else_type_var) => occurs_in(&else_type_var, &*then_type)?,
        _ => {}
    }

    inferred_types.push(TypeInference {left: then_type.clone(), right: else_type});

    Ok(then_type)
}
fn infer_comp_type(
    ast: &CompExpr,
    env: &HashMap<String, TypeScheme>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    let left_hand_type = infer_add_expr_type(&ast.add_expr, env, inferred_types, type_var_generator)?;

    if let Some(chained_expr) = &ast.chained_expr {
        let right_hand_type = infer_add_expr_type(
            &chained_expr.add_expr, env, inferred_types, type_var_generator
        )?;

        match chained_expr.comp_op {
            BooleanOps::Equality => {
                match left_hand_type.as_ref() {
                    Type::TypeVariable(type_var) => occurs_in(&type_var, &*right_hand_type)?,
                    _ => {}
                }
                match right_hand_type.as_ref() {
                    Type::TypeVariable(type_var) => occurs_in(&type_var, &*left_hand_type)?,
                    _ => {}
                }
                inferred_types.push(TypeInference {left: left_hand_type.clone(), right: right_hand_type});
            }
            _ => {
                inferred_types.push(TypeInference {left: left_hand_type.clone(), right: Box::new(Type::IntType)});
                inferred_types.push(TypeInference{left: right_hand_type.clone(), right: Box::new(Type::IntType)});
            }
        }
        return Ok(Box::new(Type::BoolType));
    }
    Ok(left_hand_type)
}

fn infer_add_expr_type(
    ast: &AddExpr,
    env: &HashMap<String, TypeScheme>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    let left_hand_mul_expr_type = infer_mul_expr_type(
        &ast.mul_expr,
        env,
        inferred_types,
        type_var_generator,
    )?;
    let mut current = &ast.chained_expr;
    let mut has_chain = false;
    while let Some(chained_expr) = current {
        let right_hand_mul_expr_type = infer_mul_expr_type(
            &chained_expr.mul_expr,
            env,
            inferred_types,
            type_var_generator,
        )?;
        if !has_chain {
            inferred_types.push(TypeInference {left: left_hand_mul_expr_type.clone(), right: Box::new(Type::IntType)});
        }
        inferred_types.push(TypeInference {left: right_hand_mul_expr_type.clone(), right: Box::new(Type::IntType)});
        current = &chained_expr.chained_expr;
        has_chain = true;
    };

    Ok(if has_chain { Box::new(Type::IntType) } else { left_hand_mul_expr_type })
}

fn infer_mul_expr_type(
    ast: &MulExpr,
    env: &HashMap<String, TypeScheme>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    let left_hand_app_expr_type = infer_app_expr_type(
        &ast.app_expr,
        env,
        inferred_types,
        type_var_generator,
    )?;
    let mut current = &ast.chained_expr;
    let mut has_chain = false;
    while let Some(chained_mul_expr) = current {
        let right_hand_app_expr_type = infer_app_expr_type(
            &chained_mul_expr.app_expr,
            env,
            inferred_types,
            type_var_generator,
        )?;
        if !has_chain {
            inferred_types.push(TypeInference {left: left_hand_app_expr_type.clone(), right: Box::new(Type::IntType)});
        }
        inferred_types.push(TypeInference {left: right_hand_app_expr_type.clone(), right: Box::new(Type::IntType)});
        current = &chained_mul_expr.chained_expr;
        has_chain = true;
    };

    Ok(
        if has_chain {Box::new(Type::IntType)} else {left_hand_app_expr_type}
    )
}

fn infer_app_expr_type(
    ast: &AppExpr,
    env: &HashMap<String, TypeScheme>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    let mut left_hand_atom_type = infer_atom_type(
        &ast.atom,
        env,
        inferred_types,
        type_var_generator,
    )?;
    let mut current = &ast.chained_expr;
    while let Some(chained_app_expr) = current {
        let right_hand_atom_type = infer_atom_type(
            &chained_app_expr.atom,
            env,
            inferred_types,
            type_var_generator,
        )?;

        match left_hand_atom_type.as_ref() {
            Type::FuncType(func_type) => {
                let return_type = func_type.return_type.clone();
                let param_type = func_type.param_type.clone();
                match &*func_type.param_type {
                    Type::TypeVariable(param_type_var) => occurs_in(&param_type_var, &*right_hand_atom_type)?,
                    _ => {}
                }
                match &*right_hand_atom_type {
                    Type::TypeVariable(right_type_var) => occurs_in(&right_type_var, &*func_type.param_type)?,
                    _ => {}
                }
                inferred_types.push(TypeInference {left: param_type, right: right_hand_atom_type.clone()});
                left_hand_atom_type = return_type;
            },
            Type::TypeVariable(type_var) => {
                match &ast.atom {
                    Atom::Identifier(_) => occurs_in(&type_var, &*right_hand_atom_type)?,
                    _ => {}
                }
                let new_type_var = Box::new(Type::TypeVariable(type_var_generator.next()));
                let new_func_type = Type::FuncType(
                    Box::new(FuncType {
                        param_type: right_hand_atom_type,
                        return_type: new_type_var.clone(),
                    })
                );

                inferred_types.push(TypeInference {left: left_hand_atom_type.clone(), right: Box::new(new_func_type)});
                left_hand_atom_type = new_type_var;
            },
            _ => return Err(format!("Primitive int and bool are not callable: {:?} ({:?})", left_hand_atom_type, right_hand_atom_type))
        };

        current = &chained_app_expr.chained_expr;
    };

    Ok(left_hand_atom_type)
}

fn infer_atom_type(
    ast: &Atom,
    env: &HashMap<String, TypeScheme>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    match ast {
        Atom::Identifier(identifier) => {
            match env.get(identifier) {
                Some(type_scheme) => {
                    Ok(instantiate(type_scheme.clone(), type_var_generator)?)
                }
                None => Err(format!("Variable reference prior to declaration: {}", identifier))
            }
        },
        Atom::IntegerLiteral(_) => Ok(Box::new(Type::IntType)),
        Atom::BooleanLiteral(_) => Ok(Box::new(Type::BoolType)),
        Atom::PrioritizedExpr(prio_expr) => infer_expr_type(&prio_expr, env, inferred_types, type_var_generator),
    }
}
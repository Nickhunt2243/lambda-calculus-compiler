use crate::ast::types::{LetExpr, IfExpr, FunctionDeclExpr, CompExpr, AddExpr, MulExpr, AppExpr, Atom, Expr, LetRecExpr};
use std::collections::HashMap;
use crate::lexer::types::BooleanOps;
use crate::type_inferencer::types::{TypeVariable, Type, FuncType, TypeInference, TypeVariableGenerate};

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

pub fn infer_expr_type<>(
    ast: &Expr,
    env: &HashMap<String, Box<Type>>,
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
    env: &HashMap<String, Box<Type>>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    let new_param_type_variable = type_var_generator.next();
    let mut local_env = env.clone();
    let type_var_type = Type::TypeVariable(new_param_type_variable);
    local_env.insert(ast.param.clone(), Box::new(type_var_type.clone()));
    let body_type = infer_expr_type(&ast.body_expr, &local_env, inferred_types, type_var_generator)?;

    Ok(Box::new(Type::FuncType(Box::new(FuncType {
        param_type: Box::new(type_var_type),
        return_type: body_type
    }))))
}
fn infer_let_type(
    ast: &LetExpr,
    env: &HashMap<String, Box<Type>>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    let mut local_env = env.clone();
    let identifier_type = infer_expr_type(&ast.value, env, inferred_types, type_var_generator)?;
    local_env.insert(ast.identifier.clone(), identifier_type);

    infer_expr_type(&ast.body_expr, &local_env, inferred_types, type_var_generator)
}

fn infer_let_rec_type(
    ast: &LetRecExpr,
    env: &HashMap<String, Box<Type>>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    let rec_type_var = type_var_generator.next();
    let mut local_env = env.clone();
    let type_var_type = Type::TypeVariable(rec_type_var.clone());
    local_env.insert(ast.identifier.clone(), Box::new(type_var_type.clone()));
    let identifier_type = infer_expr_type(&ast.value, &local_env, inferred_types, type_var_generator)?;
    inferred_types.push(TypeInference {left: Box::new(type_var_type), right: identifier_type});

    infer_expr_type(&ast.body_expr, &local_env, inferred_types, type_var_generator)
}
fn infer_if_type(
    ast: &IfExpr,
    env: &HashMap<String, Box<Type>>,
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
    env: &HashMap<String, Box<Type>>,
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
    env: &HashMap<String, Box<Type>>,
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
    env: &HashMap<String, Box<Type>>,
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
    env: &HashMap<String, Box<Type>>,
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
                match &*func_type.param_type {
                    Type::TypeVariable(param_type_var) => occurs_in(&param_type_var, &*right_hand_atom_type)?,
                    _ => {}
                }
                match &*right_hand_atom_type {
                    Type::TypeVariable(right_type_var) => occurs_in(&right_type_var, &*func_type.param_type)?,
                    _ => {}
                }
                inferred_types.push(TypeInference {left: left_hand_atom_type.clone(), right: right_hand_atom_type.clone()});
                left_hand_atom_type = func_type.return_type.clone();
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
    env: &HashMap<String, Box<Type>>,
    inferred_types: &mut Vec<TypeInference>,
    type_var_generator: &mut TypeVariableGenerate
) -> Result<Box<Type>, String> {
    match ast {
        Atom::Identifier(identifier) => {
            match env.get(identifier) {
                Some(type_var) => Ok(type_var.clone()),
                None => Err(format!("Variable reference prior to declaration: {}", identifier))
            }
        },
        Atom::IntegerLiteral(_) => Ok(Box::new(Type::IntType)),
        Atom::BooleanLiteral(_) => Ok(Box::new(Type::BoolType)),
        Atom::PrioritizedExpr(prio_expr) => infer_expr_type(&prio_expr, env, inferred_types, type_var_generator),
    }
}
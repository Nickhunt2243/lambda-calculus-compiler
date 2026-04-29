use std::collections::HashMap;
use crate::type_inferencer::types::{Type, TypeInference, TypeVariable, FuncType};

pub fn unification(inferred_types: &mut Vec<TypeInference>) -> Result<HashMap<TypeVariable, Type>, String> {
    let mut substitutions: HashMap<TypeVariable, Type> = HashMap::new();
    let mut idx = 0;
    while idx < inferred_types.len() {
        let left = *inferred_types[idx].left.clone();
        let right = *inferred_types[idx].right.clone();
            unify_inferred_types(
            &left,
            &right,
            inferred_types,
            &mut substitutions,
        )?;
        idx += 1;
    }

    Ok(substitutions)
}


fn perform_substitution(original_type: Type, sub_var: &TypeVariable, sub_type: &Type) -> Result<Type, String> {
    match original_type {
        Type::FuncType(func_type) => {

            let new_param_type: Type = match *func_type.param_type {
                Type::TypeVariable(type_variable) => {
                    if sub_var == &type_variable { sub_type.clone() } else { Type::TypeVariable(type_variable) }
                },
                other => other
            };

            let new_return_type: Type = perform_substitution(*func_type.return_type, sub_var, sub_type)?;

            Ok(Type::FuncType(
                Box::new(FuncType{
                    param_type: Box::new(new_param_type),
                    return_type: Box::new(new_return_type)})
            ))
        },
        Type::TypeVariable(type_variable) => {
            Ok(if sub_var == &type_variable { sub_type.clone() } else { Type::TypeVariable(type_variable) })
        }
        _ => Ok(original_type)
    }
}

fn substitute_inferred_type(
    sub_var: &TypeVariable,
    sub_type: &Type,
    inferred_types: &mut Vec<TypeInference>,
    substitutions: &mut HashMap<TypeVariable, Type>,
) -> Result<(), String> {
    substitutions.insert(sub_var.clone(), sub_type.clone());
    for type_inference in inferred_types.iter_mut() {
        let new_left = perform_substitution(*type_inference.left.clone(), sub_var, sub_type)?;
        let new_right = perform_substitution(*type_inference.right.clone(), sub_var, sub_type)?;
        type_inference.left = Box::new(new_left);
        type_inference.right = Box::new(new_right);
    }
    Ok(())
}

fn unify_inferred_types(
    left: &Type,
    right: &Type,
    inferred_types: &mut Vec<TypeInference>,
    substitutions: &mut HashMap<TypeVariable, Type>,
) -> Result<(), String> {
    match (left, right) {
        (Type::IntType, Type::BoolType)
        | (Type::BoolType, Type::IntType)
        => Err(format!("Failed to unify types: {:?} <> {:?}", left, right)),
        (Type::IntType, Type::IntType)
        | (Type::BoolType, Type::BoolType)
        => Ok(()),
        (Type::IntType, Type::TypeVariable(right_type_var))
        | (Type::BoolType, Type::TypeVariable(right_type_var))
        | (Type::FuncType(_), Type::TypeVariable(right_type_var))
        => substitute_inferred_type(right_type_var, left, inferred_types, substitutions),
        (Type::TypeVariable(left_type_var), Type::IntType)
        | (Type::TypeVariable(left_type_var), Type::BoolType)
        | (Type::TypeVariable(left_type_var), Type::FuncType(_))
        | (Type::TypeVariable(left_type_var), Type::TypeVariable(_))
        => substitute_inferred_type(left_type_var, right, inferred_types, substitutions),
        (Type::FuncType(left_func_type), Type::FuncType(right_fund_type))
        => {
            unify_inferred_types(
                &*left_func_type.param_type,
                &*right_fund_type.param_type,
                inferred_types,
                substitutions,
            )?;
            unify_inferred_types(
                &*left_func_type.return_type,
                &*right_fund_type.return_type,
                inferred_types,
                substitutions,
            )
        },
        (other_left, other_right) => Err(format!("Failed to unify types: {:?} <> {:?}", other_left, other_right))
    }
}

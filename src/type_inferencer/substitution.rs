use std::collections::HashMap;
use crate::type_inferencer::types::{FinalType, Type, TypeVariable, FinalFuncType};


pub fn apply_substitutions(return_type: Type, substitutions: HashMap<TypeVariable, Type>) -> Result<FinalType, String> {
    match return_type {
        Type::IntType => Ok(FinalType::IntType),
        Type::BoolType => Ok(FinalType::BoolType),
        Type::FuncType(func_type) => {
            let param_type = apply_substitutions(*func_type.param_type, substitutions.clone())?;
            let return_type = apply_substitutions(*func_type.return_type, substitutions)?;
            Ok(FinalType::FuncType(
                Box::new(FinalFuncType {
                    param_type: Box::new(param_type),
                    return_type: Box::new(return_type)
                })
            ))
        },
        Type::TypeVariable(type_variable) => {
            match substitutions.get(&type_variable) {
                Some(return_type) => {
                    apply_substitutions(return_type.clone(), substitutions.clone())
                }
                None => Ok(FinalType::Polymorphic(type_variable.name.clone()))
            }
        }
    }
}
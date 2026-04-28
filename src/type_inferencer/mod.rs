mod unification;
mod substitution;
mod infer_types;
mod types;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use infer_types::infer_expr_type;
use crate::ast::types::Expr;
use crate::type_inferencer::substitution::apply_substitutions;
use crate::type_inferencer::types::{FinalType, Type, TypeInference, TypeVariableGenerate};
use crate::type_inferencer::unification::unification;

pub fn type_inference(top_level_expr: &Expr) -> Result<FinalType, String> {
    let mut inferred_types: Vec<TypeInference> = Vec::new();
    let mut type_var_generator = TypeVariableGenerate::new();
    let env = HashMap::new();
    let return_type: Box<Type> = infer_expr_type(top_level_expr, &env, &mut inferred_types, &mut type_var_generator)?;
    let substitutions = unification(&mut inferred_types)?;
    apply_substitutions(*return_type, substitutions)
}
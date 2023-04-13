use super::*;
use crate::fhirpath::{Collection, Value, STRING};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub type Function =
    fn(input: &Collection, params: &Collection) -> Result<Collection, EvaluationError>;

lazy_static! {
    pub static ref BUILTIN_FUNCTIONS: HashMap<&'static str, Function> =
        HashMap::from([("replace", replace as Function)]);
}

fn replace(input: &Collection, params: &Collection) -> Result<Collection, EvaluationError> {
    let Value::String(str) = input.singleton(STRING)? else {return Err(EvaluationError::InvalidFunctionArguments(params.clone()))};
    if let (Some(Value::String(pattern)), Some(Value::String(substitution))) =
        (params.get(0), params.get(1))
    {
        Ok(Collection::from(Value::String(
            str.replace(pattern, substitution),
        )))
    } else {
        let mut params = input.clone();
        params.extend(input.iter().map(|v| v.clone()));
        Err(EvaluationError::InvalidFunctionArguments(params))
    }
}

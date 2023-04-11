use std::num::ParseIntError;

use crate::fhirpath::{Collection, Type};

#[derive(Debug)]
pub enum EvaluationError {
    InvalidInteger(String, ParseIntError),
    InvalidDecimal(String),
    InvalidAST,
    ExpectedSingleton(Type),
    FunctionUnavailable(String),
    InvalidFunctionArguments(Collection),
}

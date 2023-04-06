use std::num::ParseIntError;

pub enum EvaluationError {
    InvalidInteger(String, ParseIntError),
    InvalidDecimal(String),
}

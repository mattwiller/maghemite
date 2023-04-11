use rust_decimal::Decimal;

use super::*;
use crate::{
    fhirpath::{Collection, Value},
    parser::ASTNode,
};

pub struct Visitor {}

impl Visitor {
    pub fn new() -> Self {
        Visitor {}
    }

    pub fn visit_node(&mut self, node: &ASTNode) -> Result<Collection, EvaluationError> {
        return match node {
            ASTNode::StringLiteral(str) => Ok(vec![Value::String(str.to_owned())]),
            ASTNode::NumberLiteral(str) => {
                if str.contains('.') {
                    let n = Decimal::from_str_radix(str, 10)
                        .map_err(|_| EvaluationError::InvalidDecimal(str.to_string()))?;
                    Ok(vec![Value::Decimal(n)])
                } else {
                    let n = str::parse::<i32>(str)
                        .map_err(|e| EvaluationError::InvalidInteger(str.to_string(), e))?;
                    Ok(vec![Value::Integer(n)])
                }
            }
            ASTNode::BooleanLiteral(val) => Ok(vec![Value::Boolean(*val)]),
            _ => panic!("Unsupported node type"),
        };
    }
}

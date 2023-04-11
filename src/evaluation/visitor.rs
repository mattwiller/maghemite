use super::*;
use crate::fhirpath::{Collection, Value};
use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::parser::ASTNode;

pub struct Visitor {
    functions: HashMap<&'static str, Function>,
}

impl Visitor {
    pub fn new() -> Self {
        Visitor {
            functions: BUILTIN_FUNCTIONS.clone(),
        }
    }

    pub fn visit_node(&mut self, node: &ASTNode) -> Result<Collection, EvaluationError> {
        return match node {
            ASTNode::BooleanLiteral(val) => Ok(Collection::from(Value::Boolean(*val))),
            ASTNode::StringLiteral(str) => Ok(Collection::from(Value::String(str.to_owned()))),
            ASTNode::NumberLiteral(str) => {
                if str.contains('.') {
                    let n = Decimal::from_str_radix(str, 10)
                        .map_err(|_| EvaluationError::InvalidDecimal(str.to_string()))?;

                    Ok(Collection::from(Value::Decimal(n)))
                } else {
                    let n = str::parse::<i32>(str)
                        .map_err(|e| EvaluationError::InvalidInteger(str.to_string(), e))?;

                    Ok(Collection::from(Value::Integer(n)))
                }
            }
            ASTNode::InvocationExpression(left, right) => {
                let input = self.visit_node(left)?;
                match right.as_ref() {
                    ASTNode::Function(left, right) => {
                        let ASTNode::Identifier(name) = left.as_ref() else {return Err(EvaluationError::InvalidAST)};
                        let param_list = self.visit_node(&right)?;

                        if let Some(func) = self.functions.get(name.as_str()) {
                            func(&input, &param_list)
                        } else {
                            Err(EvaluationError::FunctionUnavailable(name.to_string()))
                        }
                    }
                    ASTNode::Identifier(_) => todo!(),
                    _ => Err(EvaluationError::InvalidAST),
                }
            }
            ASTNode::ParamList(inner) => {
                if let Some(params) = inner {
                    self.visit_node(params)
                } else {
                    Ok(Collection::new())
                }
            }
            ASTNode::Union(left, right) => {
                let c1 = self.visit_node(left)?;
                let c2 = self.visit_node(right)?;

                Ok(Collection::from_iter(
                    c1.iter().chain(c2.iter()).map(|v| v.clone()),
                ))
            }
            _ => panic!("Unsupported node type {:?}", node),
        };
    }
}

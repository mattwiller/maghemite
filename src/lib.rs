use evaluation::{EvaluationError, Visitor};
use fhirpath::Collection;
use parser::{ASTNode, Lexer, Parser, ParserError};

pub mod evaluation;
pub mod fhirpath;
pub mod parser;

pub struct Expression {
    raw: String,
    ast: ASTNode,
}

impl Expression {
    pub fn new(str: &str) -> Result<Expression, ParserError> {
        let tokens = Lexer::new(str).tokenize()?;
        let ast = Parser::new(tokens).parse()?;

        Ok(Expression {
            raw: str.to_string(),
            ast: *ast,
        })
    }

    pub fn evaluate(&self) -> Result<Collection, EvaluationError> {
        let mut visitor = Visitor::new();
        Ok(visitor.visit_node(&self.ast)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fhirpath::Value;
    use itertools::*;
    use pretty_assertions::assert_eq;
    use rust_decimal::{prelude::FromPrimitive, Decimal};

    #[test]
    fn test_literal_evaluation() {
        struct TestCase<'a> {
            expr: &'a str,
            expected: Collection,
        }

        let cases = vec![
            // Boolean
            TestCase {
                expr: "true",
                expected: vec![Value::Boolean(true)],
            },
            TestCase {
                expr: "false",
                expected: vec![Value::Boolean(false)],
            },
            // String
            TestCase {
                expr: "'hello, world'",
                expected: vec![Value::String("hello, world".to_string())],
            },
            // Number
            TestCase {
                expr: "14060",
                expected: vec![Value::Integer(14060)],
            },
            TestCase {
                expr: "0.00729735257",
                expected: vec![Value::Decimal(Decimal::from_f64(0.00729735257).unwrap())],
            },
        ];

        for case in cases {
            if let Ok(expr) = Expression::new(case.expr) {
                if let Ok(result) = expr.evaluate() {
                    assert_eq!(result.len(), case.expected.len());
                    for (actual, expected) in result.iter().zip_eq(&case.expected) {
                        assert_eq!(actual, expected);
                    }
                } else {
                    panic!("error evaluating expression {}", case.expr)
                }
            } else {
                panic!("error parsing expression {}", case.expr)
            }
        }
    }
}

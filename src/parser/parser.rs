use super::*;
use iota::iota;
use std::collections::VecDeque;

pub struct Parser {
    input: VecDeque<Token>,
}

pub type PrefixFn<'a> =
    fn(parser: &'a mut Parser, token: &'a Token) -> Result<Box<ASTNode>, ParserError>;
pub type InfixFn<'a> = fn(
    parser: &'a mut Parser,
    left: Box<ASTNode>,
    token: &'a Token,
) -> Result<Box<ASTNode>, ParserError>;

pub struct ParseRule<'a> {
    precedence: u8,
    prefix_parselet: Option<PrefixFn<'a>>,
    infix_parselet: Option<InfixFn<'a>>,
}

iota! {
    const INITIAL_PRECEDENCE: u8 = iota;
    , DOT_PRECEDENCE
}

impl Token {
    fn parse_rule(&self) -> ParseRule {
        match &self {
            Token::Dot => ParseRule {
                precedence: DOT_PRECEDENCE,
                prefix_parselet: None,
                infix_parselet: Some(parse_dot),
            },
            Token::Identifier(_) => ParseRule {
                precedence: INITIAL_PRECEDENCE,
                prefix_parselet: Some(parse_identifier),
                infix_parselet: None,
            },
            _ => todo!(),
        }
    }

    fn precedence(&self) -> u8 {
        self.parse_rule().precedence
    }

    fn prefix_parselet(&self) -> Option<PrefixFn> {
        self.parse_rule().prefix_parselet
    }

    fn infix_parselet(&self) -> Option<InfixFn> {
        self.parse_rule().infix_parselet
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            input: VecDeque::from(tokens),
        }
    }

    pub fn parse(mut self) -> Result<Box<ASTNode>, ParserError> {
        self.parse_expression(INITIAL_PRECEDENCE)
    }

    fn parse_expression(&mut self, base_precedence: u8) -> Result<Box<ASTNode>, ParserError> {
        let token = self.next_token().ok_or(ParserError::EOF)?;

        let prefix_parselet = token
            .prefix_parselet()
            .ok_or(ParserError::UnexpectedToken(token.clone()))?;

        let mut left = prefix_parselet(self, &token)?;
        while let Some(next_token) = self.peek() {
            if token.precedence() < base_precedence {
                break;
            }

            if let Some(infix_parselet) = next_token.infix_parselet() {
                self.next_token();
                left = infix_parselet(self, left, &next_token)?;
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn next_token(&mut self) -> Option<Token> {
        self.input.pop_front()
    }

    fn peek(&self) -> Option<Token> {
        self.input.front().map(|t| t.clone())
    }
}

fn parse_dot(
    parser: &mut Parser,
    left: Box<ASTNode>,
    token: &Token,
) -> Result<Box<ASTNode>, ParserError> {
    let right = parser.parse_expression(token.precedence())?;
    Ok(Box::new(ASTNode::InvocationExpression(left, right)))
}

fn parse_identifier(_parser: &mut Parser, token: &Token) -> Result<Box<ASTNode>, ParserError> {
    if let Token::Identifier(identifier) = token {
        return Ok(Box::new(ASTNode::Identifier(identifier.clone())));
    } else {
        return Err(ParserError::UnexpectedToken(token.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        struct TestCase {
            input: Vec<Token>,
            expected: Box<ASTNode>,
        }
        let test_cases = vec![TestCase {
            input: vec![
                Token::identifier("Patient"),
                Token::Dot,
                Token::identifier("name"),
                Token::Dot,
                Token::identifier("family"),
                Token::Dot,
                Token::identifier("replace"),
                // Token::LeftParen,
                // Token::String("er".to_string()),
                // Token::Comma,
                // Token::String("iams".to_string()),
                // Token::RightParen,
            ],
            expected: ASTNode::invocation(
                ASTNode::invocation(
                    ASTNode::invocation(
                        ASTNode::identifier("Patient"),
                        ASTNode::identifier("name"),
                    ),
                    ASTNode::identifier("family"),
                ),
                ASTNode::identifier("replace"),
            ),
        }];

        for test in test_cases {
            let parser = Parser::new(test.input);
            let ast = parser.parse().expect("failed building AST");
            assert_eq!(ast, test.expected, "output AST does not match expected");
        }
    }
}

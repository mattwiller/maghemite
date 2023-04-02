use super::*;
use iota::iota;
use log::*;
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
    , PLUS_PRECENDENCE
    , MINUS_PRECENDENCE
    , DOT_PRECEDENCE
    , LPAREN_PRECENDENCE
    , RPAREN_PRECENDENCE
    , COMMA_PRECENDENCE
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
            Token::LeftParen => ParseRule {
                precedence: LPAREN_PRECENDENCE,
                prefix_parselet: None, // TODO: implement paren group
                infix_parselet: Some(parse_function),
            },
            Token::RightParen => ParseRule {
                precedence: RPAREN_PRECENDENCE,
                prefix_parselet: Some(|_, _| Ok(ASTNode::empty_params())),
                infix_parselet: Some(parse_param_list),
            },
            Token::Comma => ParseRule {
                precedence: COMMA_PRECENDENCE,
                prefix_parselet: None,
                infix_parselet: Some(parse_union),
            },
            Token::String(_) => ParseRule {
                precedence: INITIAL_PRECEDENCE,
                prefix_parselet: Some(parse_string_literal),
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
        debug!("expression starts with {:?}", token);

        let prefix_parselet = token
            .prefix_parselet()
            .ok_or(ParserError::UnexpectedToken(token.clone()))?;

        let mut left = prefix_parselet(self, &token)?;
        while let Some(next_token) = self.peek() {
            if next_token.precedence() <= base_precedence {
                break;
            }

            debug!("parsing infix token {:?}", next_token);
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

fn parse_identifier(_: &mut Parser, token: &Token) -> Result<Box<ASTNode>, ParserError> {
    if let Token::Identifier(identifier) = token {
        return Ok(Box::new(ASTNode::Identifier(identifier.clone())));
    } else {
        return Err(ParserError::UnexpectedToken(token.clone()));
    }
}

fn parse_function(
    parser: &mut Parser,
    left: Box<ASTNode>,
    token: &Token,
) -> Result<Box<ASTNode>, ParserError> {
    let right = parser.parse_expression(token.precedence())?;
    Ok(Box::new(ASTNode::Function(left, right)))
}

fn parse_param_list(
    _: &mut Parser,
    left: Box<ASTNode>,
    _: &Token,
) -> Result<Box<ASTNode>, ParserError> {
    Ok(Box::new(ASTNode::ParamList(Some(left))))
}

fn parse_union(
    parser: &mut Parser,
    left: Box<ASTNode>,
    token: &Token,
) -> Result<Box<ASTNode>, ParserError> {
    let right = parser.parse_expression(token.precedence())?;
    Ok(Box::new(ASTNode::Union(left, right)))
}

fn parse_string_literal(_: &mut Parser, token: &Token) -> Result<Box<ASTNode>, ParserError> {
    if let Token::String(s) = token {
        return Ok(Box::new(ASTNode::StringLiteral(s.clone())));
    } else {
        return Err(ParserError::UnexpectedToken(token.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parser() {
        env_logger::init();

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
                Token::LeftParen,
                Token::string("er"),
                Token::Comma,
                Token::string("iams"),
                Token::RightParen,
            ],
            expected: ASTNode::invocation(
                ASTNode::invocation(
                    ASTNode::invocation(
                        ASTNode::identifier("Patient"),
                        ASTNode::identifier("name"),
                    ),
                    ASTNode::identifier("family"),
                ),
                ASTNode::function(
                    ASTNode::identifier("replace"),
                    ASTNode::params(ASTNode::union(
                        ASTNode::string("er"),
                        ASTNode::string("iams"),
                    )),
                ),
            ),
        }];

        for test in test_cases {
            let parser = Parser::new(test.input);
            let ast = parser.parse().expect("failed building AST");
            assert_eq!(ast, test.expected, "output AST does not match expected");
        }
    }
}

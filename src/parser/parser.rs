use super::*;
use iota::iota;

pub struct Parser {
    input: Vec<Token>,
    position: usize,
}

pub type PrefixFn<'a> = fn(parser: &'a Parser, token: Token) -> Box<ASTNode>;
pub type InfixFn<'a> = fn(parser: &'a Parser, left: Box<ASTNode>, token: Token) -> Box<ASTNode>;

pub struct ParseRule {
    precedence: u8,
    prefix_parselet: Option<PrefixFn<'static>>,
    infix_parselet: Option<InfixFn<'static>>,
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
                infix_parselet: Some(parse_dot_infix),
            },
            _ => unimplemented!(),
        }
    }
}

impl Parser {
    pub fn parse() -> Box<ASTNode> {
        todo!()
    }
}

fn parse_dot_infix<'a>(parser: &'a Parser, left: Box<ASTNode>, token: Token) -> Box<ASTNode> {
    todo!()
}

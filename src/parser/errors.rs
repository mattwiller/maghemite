use super::*;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(Token),
    EOF,
}

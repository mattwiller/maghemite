use super::*;

#[derive(Debug)]
pub enum ParserError {
    InvalidIdentifierCharacter(char),
    InvalidString,
    UnexpectedToken(Token),
    EOF,
}

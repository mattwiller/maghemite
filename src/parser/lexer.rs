#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    String(String),
    Number(String),
    Identifier(String),
    Dot,
    Comma,
    LeftParen,
    RightParen,
    Plus,
    Minus,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

const TOKEN_SIZE_ESTIMATE: usize = 4;

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::with_capacity(self.input.len() / TOKEN_SIZE_ESTIMATE);

        while self.position < self.input.len() {
            let c = self.input[self.position];
            let token = match c {
                ' ' | '\r' | '\n' | '\t' => None,
                '.' => Some(Token::Dot),
                ',' => Some(Token::Comma),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                n @ '0'..='9' => Some(Token::Number(n.to_string())),
                '\'' => {
                    if self.position + 1 >= self.input.len() {
                        return Err(LexerError::InvalidString);
                    }
                    let str: String = self
                        .input
                        .get(self.position + 1..)
                        .unwrap()
                        .iter()
                        .take_while(|&&x| x != '\'')
                        .collect();
                    self.position += str.len() + 1;
                    Some(Token::String(str))
                }
                n => {
                    if !is_valid_identifier_char(n) || n.is_numeric() {
                        return Err(LexerError::InvalidIdentifierCharacter(n));
                    }

                    let identifier: String = self
                        .input
                        .get(self.position..)
                        .unwrap()
                        .iter()
                        .take_while(|&&x| is_valid_identifier_char(x))
                        .collect();

                    self.position += identifier.len() - 1;
                    Some(Token::Identifier(identifier))
                }
            };
            if let Some(token) = token {
                tokens.push(token);
            }
            self.position += 1;
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }
}

fn is_valid_identifier_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[derive(Debug)]
pub enum LexerError {
    InvalidIdentifierCharacter(char),
    InvalidString,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        struct TestCase {
            expression: &'static str,
            expected: Vec<Token>,
        }
        let test_cases = vec![TestCase {
            expression: "Patient.name.family.replace('er', 'iams')",
            expected: vec![
                Token::Identifier("Patient".to_string()),
                Token::Dot,
                Token::Identifier("name".to_string()),
                Token::Dot,
                Token::Identifier("family".to_string()),
                Token::Dot,
                Token::Identifier("replace".to_string()),
                Token::LeftParen,
                Token::String("er".to_string()),
                Token::Comma,
                Token::String("iams".to_string()),
                Token::RightParen,
                Token::EOF,
            ],
        }];

        for test in test_cases {
            let mut lex = Lexer::new(test.expression);
            let tokens = lex.tokenize();

            if tokens.is_err() {
                assert!(false, "{:?}", tokens)
            }

            assert_eq!(tokens.unwrap(), test.expected);
        }
    }
}

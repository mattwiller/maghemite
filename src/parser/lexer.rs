use super::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Token {
    String(String),
    Number(String),
    Boolean(bool),
    Identifier(String),
    Plus,
    Minus,
    Dot,
    LeftParen,
    RightParen,
    Comma,
}

impl Token {
    pub fn identifier(s: &str) -> Self {
        Token::Identifier(s.to_string())
    }

    pub fn string(s: &str) -> Self {
        Token::String(s.to_string())
    }
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

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut tokens = Vec::with_capacity(self.input.len() / TOKEN_SIZE_ESTIMATE);

        while self.position < self.input.len() {
            let c = self.input[self.position];
            if let Some(token) = match c {
                ' ' | '\r' | '\n' | '\t' => None, // Skip whitespace
                '.' => Some(Token::Dot),
                ',' => Some(Token::Comma),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '0'..='9' => {
                    let str: String = self
                        .input
                        .get(self.position..)
                        .unwrap()
                        .iter()
                        .take_while(|&&x| x.is_ascii_digit() || x == '.')
                        .collect();
                    self.position += str.len();
                    Some(Token::Number(str))
                }
                '\'' => {
                    if self.position + 1 >= self.input.len() {
                        return Err(ParserError::InvalidString);
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
                    if !is_valid_identifier_char(n) || n.is_ascii_digit() {
                        return Err(ParserError::InvalidIdentifierCharacter(n));
                    }

                    let identifier: String = self
                        .input
                        .get(self.position..)
                        .unwrap()
                        .iter()
                        .take_while(|&&x| is_valid_identifier_char(x))
                        .collect();

                    self.position += identifier.len() - 1;

                    let token = if identifier == "true" {
                        Token::Boolean(true)
                    } else if identifier == "false" {
                        Token::Boolean(false)
                    } else {
                        Token::Identifier(identifier)
                    };
                    Some(token)
                }
            } {
                tokens.push(token);
            }
            self.position += 1;
        }

        Ok(tokens)
    }
}

fn is_valid_identifier_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
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
        let test_cases = vec![
            TestCase {
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
                ],
            },
            TestCase {
                expression: "12345 + 67890",
                expected: vec![
                    Token::Number("12345".to_string()),
                    Token::Plus,
                    Token::Number("67890".to_string()),
                ],
            },
            TestCase {
                expression: "true",
                expected: vec![Token::Boolean(true)],
            },
            TestCase {
                expression: "false",
                expected: vec![Token::Boolean(false)],
            },
        ];

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

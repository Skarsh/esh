pub mod token;

use token::{Token, TokenKind, lookup_identifier};

struct TextSpan {
    start: usize,
    end: usize,
}

impl TextSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }

    pub fn to_string<'a>(&self, input: &'a str) -> &'a str {
        &input[self.start..self.end]
    }
}

pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl<'a> Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            source: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current_char() {
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Token::new(TokenKind::Equal)
                } else {
                    Token::new(TokenKind::Assign)
                }
            }
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    Token::new(TokenKind::NotEqual)
                } else {
                    Token::new(TokenKind::Bang)
                }
            }
            Some('+') => Token::new(TokenKind::Plus),
            Some('-') => Token::new(TokenKind::Minus),
            Some('*') => Token::new(TokenKind::Asterisk),
            Some('/') => Token::new(TokenKind::Slash),
            Some('<') => Token::new(TokenKind::LessThan),
            Some('>') => Token::new(TokenKind::GreaterThan),
            Some(';') => Token::new(TokenKind::Semicolon),
            Some(',') => Token::new(TokenKind::Comma),
            Some('(') => Token::new(TokenKind::LParen),
            Some(')') => Token::new(TokenKind::RParen),
            Some('{') => Token::new(TokenKind::LBrace),
            Some('}') => Token::new(TokenKind::RBrace),
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let identifier = self.read_identifier();
                return Token::new(lookup_identifier(&identifier));
            }
            Some(ch) if ch.is_digit(10) => {
                let number_str = self.read_number();
                return Token::new(TokenKind::Number(
                    number_str.to_string().parse().unwrap_or(0.0),
                ));
            }
            None => Token::new(TokenKind::Eof),
            _ => Token::new(TokenKind::Illegal),
        };
        self.advance();
        token
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.source[start_pos..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.position;
        while let Some(ch) = self.current_char() {
            if ch.is_digit(10) || ch == '.' {
                self.advance();
            } else {
                break;
            }
        }
        self.source[start_pos..self.position].iter().collect()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) {
        if self.position < self.source.len() {
            self.position += 1
        }
    }

    fn current_char(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    fn peek_char(&self) -> Option<char> {
        self.source.get(self.position + 1).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        expected_kind: TokenKind,
        expected_literal: String,
    }

    #[test]
    fn test_next_token() {
        let source = r#"
            let five = 5;
        "#;

        let mut lexer = Lexer::new(source);

        let test_cases = vec![
            TestCase {
                expected_kind: TokenKind::Let,
                expected_literal: "let".to_string(),
            },
            TestCase {
                expected_kind: TokenKind::Identifier("five".to_string()),
                expected_literal: "five".to_string(),
            },
            TestCase {
                expected_kind: TokenKind::Assign,
                expected_literal: "=".to_string(),
            },
            TestCase {
                expected_kind: TokenKind::Number(5.0),
                expected_literal: "5".to_string(),
            },
            TestCase {
                expected_kind: TokenKind::Semicolon,
                expected_literal: ";".to_string(),
            },
        ];

        for case in test_cases {
            let token = lexer.next_token();
            assert_eq!(case.expected_kind, token.kind());
            assert_eq!(case.expected_literal, token.literal().to_string())
        }
    }
}

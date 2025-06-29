#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Special Tokens
    Illegal, // Represents a token we don't recognize
    Eof,     // End of File

    // Identifiers + Literals
    Identifier(String), // e.g., add, foobar, x, y
    Number(f64),        // e.g., 123, 3.14

    // Operators
    Assign,      // =
    Plus,        // +
    Minus,       // -
    Bang,        // !
    Asterisk,    // *
    Slash,       // /
    Equal,       // ==
    NotEqual,    // !=
    LessThan,    // <
    GreaterThan, // >

    // Delimiters
    Comma,     // ,
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

pub struct Token {
    kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }
}

pub fn lookup_identifier(ident: &str) -> TokenKind {
    match ident {
        "fn" => TokenKind::Function,
        "let" => TokenKind::Let,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "return" => TokenKind::Return,
        _ => TokenKind::Identifier(ident.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        identifier: &'a str,
        kind: TokenKind,
    }

    impl<'a> TestCase<'a> {
        fn new(identifier: &'a str, kind: TokenKind) -> Self {
            Self { identifier, kind }
        }
    }

    #[test]
    fn test_lookup_identifier() {
        let mut test_cases = vec![];

        test_cases.push(TestCase::new("fn", TokenKind::Function));
        test_cases.push(TestCase::new("let", TokenKind::Let));
        test_cases.push(TestCase::new("true", TokenKind::True));
        test_cases.push(TestCase::new("false", TokenKind::False));
        test_cases.push(TestCase::new("if", TokenKind::If));
        test_cases.push(TestCase::new("else", TokenKind::Else));
        test_cases.push(TestCase::new("return", TokenKind::Return));

        for test_case in test_cases {
            let kind = lookup_identifier(test_case.identifier);
            assert_eq!(kind, test_case.kind);
        }
    }
}

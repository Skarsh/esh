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

impl TokenKind {
    pub fn to_string(&self) -> String {
        match self {
            TokenKind::Illegal => "Illegal".to_string(),
            TokenKind::Eof => "Eof".to_string(),
            TokenKind::Identifier(val) => val.to_string(),
            TokenKind::Number(val) => val.to_string(),
            TokenKind::Assign => "=".to_string(),
            TokenKind::Plus => "+".to_string(),
            TokenKind::Minus => "-".to_string(),
            TokenKind::Bang => "!".to_string(),
            TokenKind::Asterisk => "*".to_string(),
            TokenKind::Slash => "/".to_string(),
            TokenKind::Equal => "==".to_string(),
            TokenKind::NotEqual => "!=".to_string(),
            TokenKind::LessThan => "<".to_string(),
            TokenKind::GreaterThan => ">".to_string(),
            TokenKind::Comma => ",".to_string(),
            TokenKind::Semicolon => ";".to_string(),
            TokenKind::LParen => "(".to_string(),
            TokenKind::RParen => ")".to_string(),
            TokenKind::LBrace => "{".to_string(),
            TokenKind::RBrace => "}".to_string(),
            TokenKind::Function => "fn".to_string(),
            TokenKind::Let => "let".to_string(),
            TokenKind::True => "true".to_string(),
            TokenKind::False => "false".to_string(),
            TokenKind::If => "if".to_string(),
            TokenKind::Else => "else".to_string(),
            TokenKind::Return => "return".to_string(),
        }
    }
}

pub struct Token {
    kind: TokenKind,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        let literal = kind.to_string();
        Self { kind, literal }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }

    pub fn literal(&self) -> &str {
        &self.literal
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

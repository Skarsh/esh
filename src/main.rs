use esh::lexer::{Lexer, token::TokenKind};

fn main() {
    let source_code = r#"
        let five = 5;
        let ten = 10;
    "#;

    let mut lexer = Lexer::new(source_code);
    loop {
        let token = lexer.next_token();
        let kind = token.kind();
        if kind == TokenKind::Eof {
            println!("{:?}", kind);
            break;
        }
        println!("{:?}", kind);
    }
}

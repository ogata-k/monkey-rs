/// Tokenとして便指揮できる識別句の一覧
#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    // 特殊な状態
    ILLEGAL,
    EOF,

    //識別子とリテラル
    IDENT,
    INT,

    // 演算子
    ASSIGN,
    PLUS,

    // デリミタ
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // キーワード
    FUNCTION,
    LET,
}


/// 読んだ文字とそれに対応する識別句からなるトークン
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    /// 初期化関数
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        return Token {
            token_type,
            literal: literal.to_string(),
        };
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::Token;
    use crate::token::TokenType;

    #[test]
    fn test_no_line() {
        let input = "";
        let tests = [Token::new(TokenType::EOF, "")];
        let mut lexer = Lexer::new(input);
        for (i, tt) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(tok.token_type, tt.token_type);
            assert_eq!(tok.literal, tt.literal);
        }
    }

    #[test]
    fn test_next_token() {
        let input =
        "
        +(){},;
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        ";

        let tests = [
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::PLUS, "+"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::FUNCTION, "fn"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::PLUS,  "+"),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "result"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::EOF, "")
        ];

        let mut lexer = Lexer::new(input);

        for (i, tt) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(tok.token_type, tt.token_type);
            assert_eq!(tok.literal, tt.literal);
        }
    }
}

use std::collections::HashMap;

/// Tokenとして便指揮できる識別句の一覧
#[derive(Debug, Eq, PartialEq, Clone)]
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

impl TokenType {
    /// 予約語一覧を取得する
    pub fn keywords() -> HashMap<String, TokenType> {
        return vec![
            ("fn".to_string(), TokenType::FUNCTION),
            ("let".to_string(), TokenType::LET)
        ].into_iter().collect();
    }

    /// 引数が予約語か識別句かどうかでTokenTypeを返す
    pub fn lookup_ident(ident: &str) ->TokenType{
        let keywords = TokenType::keywords();
        if keywords.contains_key(ident) {
            return keywords.get(ident).unwrap().clone();
        }
        return TokenType::IDENT;
    }
}


/// 読んだ文字とそれに対応する識別句からなるトークン
#[derive(Debug)]
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
        =+(){},;
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
            Token::new(TokenType::PLUS, "+"),
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

            println!("{:?} is {:?}?", tok, tt);
            assert_eq!(tok.token_type, tt.token_type);
            assert_eq!(tok.literal, tt.literal);
        }
    }
}

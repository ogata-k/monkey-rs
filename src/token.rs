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

pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token{
    pub fn new(token_type: TokenType, literal: &str) -> Self{
        return Token{
            token_type,
            literal: literal.to_string()
        };
    }
}

#[cfg(test)]
mod test {
    use crate::token::Token;
    use crate::token::TokenType;
    use crate::lexer::Lexer;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = [
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::PLUS, "+"),
            Token::new(TokenType::LPAREN, "("),
            Token::new(TokenType::RPAREN, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::EOF, "")
        ];

        let mut lexer = Lexer::new(input);

        for (i, tt) in tests.iter().enumerate(){
            let tok = lexer.next_token();

            assert_eq!(tok.token_type, tt.token_type);
            assert_eq!(tok.literal, tt.literal);
        }
    }
}

pub struct Token {
    token_type: TokenType,
    literal: String,
}

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



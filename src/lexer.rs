use crate::token::{Token, TokenType};

/// 字句解析器
pub struct Lexer {
    input: String,
    // 対象の文字列
    position: usize,
    // 入力に対する現在の位置(現在の文字の位置)
    read_position: usize,
    // これから読み込む位置(現在の文字の次の位置)
    ch: Option<char>,        // 現在検査中の文字
}

impl Lexer {
    /// 初期化関数
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.to_string(),
            // positionは解析が済んだ最終位置
            position: 0,
            // read_positionは現在読んでいる位置
            read_position: 0,
            ch: None,
        };

        l.read_char();
        return l;
    }

    /// 文字として認識しない空白扱いできる記号を飛ばす関数
    fn skip_whitespace(&mut self) {
        loop {
            if let Some(c) = self.ch {
                if (c == ' ' || c == '\t' || c == '\n' || c == '\r') {
                    self.read_char();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    /// 一文字分を呼んで状態を更新するメソッド
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position as usize);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// 識別子を読んで返す関数
    fn read_identifier(&mut self) -> String {
        // 文字の位置の始点
        let position = self.position;
        loop {
            if let Some(c) = self.ch {
                if is_letter(&c) {
                    self.read_char();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        return self.input.as_str()[position..self.position].to_string();
    }

    /// 入力の次の部分を呼んでToken構造体を生成するメソッド
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch.clone() {
            Some('=') => { Token::new(TokenType::ASSIGN, "=") }
            Some(';') => { Token::new(TokenType::SEMICOLON, ";") }
            Some('(') => { Token::new(TokenType::LPAREN, "(") }
            Some(')') => { Token::new(TokenType::RPAREN, ")") }
            Some(',') => { Token::new(TokenType::COMMA, ",") }
            Some('+') => { Token::new(TokenType::PLUS, "+") }
            Some('{') => { Token::new(TokenType::LBRACE, "{") }
            Some('}') => { Token::new(TokenType::RBRACE, "}") }
            Some(c) => {
                if is_letter(&c) {
                    let ident = self.read_identifier();
                    let token_type = TokenType::lookup_ident(&ident);
                    Token::new(token_type, &ident)
                } else {
                    Token::new(TokenType::ILLEGAL, &c.to_string())
                }
            }
            None => { Token::new(TokenType::EOF, "") }
        };

        self.read_char();
        return tok;
    }
}

/// monkeyの識別子用の文字判定関数
fn is_letter(ch: &char) -> bool {
    return ('a' <= *ch && *ch <= 'z') || ('A' <= *ch && *ch <= 'Z') || *ch == '_';
}

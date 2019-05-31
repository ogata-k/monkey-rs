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
            self.ch = self.input.chars().nth(self.read_position);
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

    /// 数字を読んで返す関数
    fn read_number(&mut self) -> String {
        // 文字の位置の始点
        let position = self.position;
        loop {
            if let Some(c) = self.ch {
                if is_digit(&c) {
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
        let mut tok: Option<Token> = None;
        match self.ch.clone() {
            Some('=') => {
                tok = Some(Token::new(TokenType::ASSIGN, "="));
                self.read_char();
            }
            Some(';') => {
                tok = Some(Token::new(TokenType::SEMICOLON, ";"));
                self.read_char();
            }
            Some('(') => {
                tok = Some(Token::new(TokenType::LPAREN, "("));
                self.read_char();
            }
            Some(')') => {
                tok = Some(Token::new(TokenType::RPAREN, ")"));
                self.read_char();
            }
            Some(',') => {
                tok = Some(Token::new(TokenType::COMMA, ","));
                self.read_char();
            }
            Some('+') => {
                tok = Some(Token::new(TokenType::PLUS, "+"));
                self.read_char();
            }
            Some('{') => {
                tok = Some(Token::new(TokenType::LBRACE, "{"));
                self.read_char();
            }
            Some('}') => {
                tok = Some(Token::new(TokenType::RBRACE, "}"));
                self.read_char();
            }
            Some(c) => {
                if is_letter(&c) {
                    let ident = self.read_identifier();
                    let token_type = TokenType::lookup_ident(&ident);
                    tok = Some(Token::new(token_type, &ident));
                } else if is_digit(&c) {
                    tok = Some(Token::new(TokenType::INT, &self.read_number()));
                } else {
                    tok = Some(Token::new(TokenType::ILLEGAL, &c.to_string()));
                }
            }
            None => {
                if self.position == self.input.len() {
                    tok = Some(Token::new(TokenType::EOF, ""));
                } else {
                    tok  =Some(Token::new(TokenType::ILLEGAL, ""));
                }
            }
        };

        if tok.is_none() { tok = Some(Token::new(TokenType::ILLEGAL, "")); }
        return tok.unwrap();
    }
}

/// 識別子用の文字判定関数
fn is_letter(ch: &char) -> bool {
    return ('a' <= *ch && *ch <= 'z') || ('A' <= *ch && *ch <= 'Z') || *ch == '_';
}

/// 数字用の判定関数
fn is_digit(ch: &char) -> bool {
    return '0' <= *ch && *ch <= '9';
}
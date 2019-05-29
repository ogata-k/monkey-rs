use crate::token;
use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    // 対象の文字列
    position: u32,
    // 入力に対する現在の位置(現在の文字の位置)
    read_position: u32,
    // これから読み込む位置(現在の文字の次の位置)
    ch: Option<char>,        // 現在検査中の文字
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None,
        };

        l.read_char();
        return l;
    }


    fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u32 {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position as usize);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            Some('=') => { Token::new(TokenType::ASSIGN, "=") },
            Some(';') => { Token::new(TokenType::SEMICOLON, ";") },
            Some('(') => { Token::new(TokenType::LPAREN, "(") },
            Some(')') => { Token::new(TokenType::RPAREN, ")") },
            Some(',') => { Token::new(TokenType::COMMA, ",") },
            Some('+') => { Token::new(TokenType::PLUS, "+") },
            Some('{') => { Token::new(TokenType::LBRACE, "{") },
            Some('}') => { Token::new(TokenType::RBRACE, "}") },
            Some(_) => {panic!("unknown token");},
            None => { Token::new(TokenType::EOF, "") },
        };

        self.read_char();
        return tok;
    }
}

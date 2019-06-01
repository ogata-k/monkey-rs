use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::Program;

/// パーサー(構文解析器)
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl std::fmt::Debug for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Parser{{position: {}, current_token{:?}, peek_token{:?}}}",
               self.lexer.get_position(),
               self.current_token,
               self.peek_token
        )
    }
}

impl Parser {
    /// 初期化関数
    pub fn new(mut lexer: Lexer) -> Self {
        let first = lexer.next_token();
        let second = lexer.next_token();
        return Parser {
            lexer,
            current_token: first,
            peek_token: second,
        };
    }

    /// 保持している字句解析器を使って一文字読む関数
    pub fn next_token(&mut self) {
        std::mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    /// 字句解析器の結果を元にMonkeyプログラムを表す解釈木を生成する関数
    pub fn parse_program(&mut self) -> Program {
        unimplemented!()
    }
}
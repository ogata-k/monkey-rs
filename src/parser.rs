use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::ast::*;
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
    pub fn parse_program(&mut self) -> Option<Program> {
        // 簡易的にOption型にしているがResultを返すように修正してもよい
        let mut program = Program::new();
        let eof: Token = Token::new(TokenType::EOF, "");

        // 今まはOption型のため簡易的に例外処理をpanicで済ましている。
        loop {
            // 正常終了
            if self.current_token == eof {
                break;
            }

            // 異常終了
            if self.current_token.is_illegal_token() {
                panic!("異常なトークンを検出しました。");
            }

            let stmt_opt = self.parse_statement();

            // 異常終了(後日式にも対応したら変更する必要がある)
            if stmt_opt.is_none() {
                panic!("文をパースできませんでした。");
            }
            let stmt = stmt_opt.unwrap();
            program.statements.push(stmt);
            self.next_token();
        }
        println!("parsed program is {:?}", program);
        return Some(program);
    }

    /// 現在起点となってるトークンの型を確認する関数
    fn current_token_is(&self, token_type: TokenType) -> bool {
        return self.current_token.get_token_type() == token_type;
    }

    /// 先のトークンの型を確認する関数
    fn peek_token_is(&self, token_type: TokenType) -> bool{
        return self.peek_token.get_token_type() == token_type;
    }

    /// 文用のパーサー
    pub fn parse_statement(&mut self) -> Option<Statement> {
        if self.current_token.is_let_token() {
            return self.parse_let_statement()
        } else {
            return None;
        }
    }

    /// 次の型を読んで予期している型かどうかを判定する関数
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }

    /// let文をパースするためのパーサー
    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.current_token.is_let_token() {
            return None;
        }
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }
        let ident = Expression::Identifier {
            token: Token::new(TokenType::IDENT, &self.current_token.get_literal()),
            value: self.current_token.get_literal(),
        };

        // TODO ident_stubをのちにパースされた式で置き換える
        let ident_stub = Expression::Identifier {
            token: Token::new(TokenType::IDENT, &self.current_token.get_literal()),
            value: self.current_token.get_literal(),
        };
        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        // TODO セミコロンに遭遇するまで式を読み飛ばしてしまってる
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        let let_statement = Statement::LetStatement {
                token: Token::new(TokenType::LET, "let"),
                name: Box::new(ident),
                value: Box::new(ident_stub),
            };
        println!("let statement {:?}", let_statement);
        return Some(let_statement);
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::ast::*;
    use crate::token::*;

    /// let文の構文解析用のテスト
    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program_opt = parser.parse_program();
        if program_opt.is_none() {
            assert!(false, "let文のパースに失敗しました。");
        }
        let program = program_opt.unwrap();
        let statements = &program.statements;
        if statements.len() != 3 {
            println!("statements is {:?}", statements);
            assert!(false, "let文の個数が不適切です。");
        }

        let tests = ["x", "y", "foobar"];

        for (i, test) in tests.iter().enumerate() {
            let stmt = &statements[i];
            test_let_statement(stmt, test);
        }
    }

    // 束縛される値は後でやるとして、束縛時の変数名をテストする関数
    fn test_let_statement(stmt: &Statement, test: &str) {
        println!("input statement {:?}, test statement {:?}", stmt, test);
        match stmt {
            Statement::LetStatement { token, name, value } => {
                // トークンのletで始まってるか確認
                assert_eq!(token.get_literal(), "let");
                // 束縛変数名の確認
                assert_eq!(name.get_value(), test);
                // TODO 束縛された値の確認
                // assert_eq!(value.get_value(), "");
            }
            _ => { assert!(false, "let文ではありません。"); }
        }
    }
}
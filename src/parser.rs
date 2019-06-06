use std::collections::HashMap;

use crate::ast::*;
use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

/// 前置関数
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum PrefixFns {
    // 何か
}

impl PrefixFns {
    pub fn get_fn(&self) -> Box<(Fn() -> Option<Expression>)> {
        unimplemented!()
    }
}

/// 中置関数
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum InfixFns {
    // 何か
}

impl InfixFns {
    pub fn get_fn(&self) -> Box<(Fn(Expression) -> Option<Expression>)> {
        unimplemented!()
    }
}

/// 式で認識する演算
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum Opt {
    LOWEST,
    EQUALS,
    // ==
    LESSGREATER,
    // > or <
    SUM,
    // +
    PRODUCT,
    //*
    PREFIX,
    // -x or !x
    CALL,        // myFunction(x)
}

/// パーサー(構文解析器)
pub struct Parser {
    lexer: Lexer,
    // 入力を元にトークンを返すための字句解析器
    current_token: Token,
    // 現在読んでいるトークン
    peek_token: Token,
    // 一つ先のトークン
    errors: Vec<String>, // パースシテ失敗したときのエラー文の集まり

    // 前置構文解析関数
    prefix_parse_fns: HashMap<TokenType, PrefixFns>,
    // 中置構文解析関数
    infix_parse_fns: HashMap<TokenType, InfixFns>,
}

impl std::fmt::Debug for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Parser{{position: {}, current_token{:?}, peek_token{:?}}}",
            self.lexer.get_position(),
            self.current_token,
            self.peek_token
        )
    }
}

impl Parser {
    // 基本的な関数群
    /// 初期化関数
    pub fn new(mut lexer: Lexer) -> Self {
        let first = lexer.next_token();
        let second = lexer.next_token();
        let mut parser = Parser {
            lexer,
            current_token: first,
            peek_token: second,
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        return parser;
    }

    /// 前置関数を登録する関数
    pub fn register_prefix(&mut self, token_type: TokenType, prefix_fn: PrefixFns) {
        self.prefix_parse_fns.insert(token_type, prefix_fn);
    }

    /// 中置関数を登録する関数
    pub fn register_infix(&mut self, token_type: TokenType, infix_fn: InfixFns) {
        self.infix_parse_fns.insert(token_type, infix_fn);
    }

    /// 先のトークンの型を確認する関数
    fn peek_token_is(&self, token_type: TokenType) -> bool {
        return self.peek_token.get_token_type() == token_type;
    }

    /// 現在起点となってるトークンの型を確認する関数
    fn current_token_is(&self, token_type: TokenType) -> bool {
        return self.current_token.get_token_type() == token_type;
    }

    // 読み込み用の関数群
    /// 保持している字句解析器を使って一文字読む関数
    pub fn next_token(&mut self) {
        std::mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    /// 次の型を読んで予期している型かどうかを判定する関数
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type.clone()) {
            self.next_token();
            return true;
        } else {
            self.make_peek_error(token_type);
            return false;
        }
    }

    // パース処理
    /// 字句解析器の結果を元にMonkeyプログラムを表す解釈木を生成する関数
    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::new();

        loop {
            // 正常終了
            if self.current_token.token_type_is(TokenType::EOF) {
                break;
            }

            // 異常終了
            if self.current_token.token_type_is(TokenType::ILLEGAL) {
                self.make_illegal_error();
                break;
            }

            let stmt_opt = self.parse_statement();

            // 異常終了(後日式にも対応したら変更する必要がある)
            if stmt_opt.is_none() {
                self.make_statement_parse_error();
                break;
            }
            let stmt = stmt_opt.unwrap();
            program.statements.push(stmt);
            self.next_token();
        }
        return Some(program);
    }

    /// 文用のパーサー
    pub fn parse_statement(&mut self) -> Option<Statement> {
        match &self.current_token {
            tok if tok.token_type_is(TokenType::LET) => {
                return self.parse_let_statement();
            }
            tok if tok.token_type_is(TokenType::RETURN) => {
                return self.parse_return_statement();
            }
            _ => {
                return self.parse_expression_statement();
            }
        }
    }

    /// let文をパースするためのパーサー
    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.current_token_is(TokenType::LET) {
            return None;
        }
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }
        let ident = Expression::Identifier {
            token: Token::new(TokenType::IDENT, &self.current_token.get_literal()),
            value: self.current_token.get_literal(),
        };

        // TODO expression_stubをのちにパースされた式で置き換える
        let expression_stub = Expression::Identifier {
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
            value: Box::new(expression_stub),
        };
        return Some(let_statement);
    }

    /// return文をパースするためパーサー
    fn parse_return_statement(&mut self) -> Option<Statement> {
        if !self.current_token_is(TokenType::RETURN) {
            self.make_peek_error(TokenType::RETURN);
            return None;
        }
        self.next_token();

        // TODO セミコロンに遭遇するまで式を読み飛ばしてしまっている。
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        // TODO expressionをパースできるようになったらパースしたものと置き換える
        let expression_stub = Expression::Identifier {
            token: Token::new(TokenType::IDENT, ""),
            value: "".to_string(),
        };

        let return_stmt = Statement::ReturnStatement {
            token: Token::new(TokenType::RETURN, "return"),
            return_value: Box::new(expression_stub),
        };
        return Some(return_stmt);
    }

    /// 式文をパースするためのパーサー
    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let c_tok = self.current_token.clone();
        let expression_opt = self.parse_expression(Opt::LOWEST);
        if expression_opt.is_none() {
            return None;
        }
        let expression = expression_opt.unwrap();
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        let stmt = Statement::ExpressionStatement {
            token: c_tok,
            expression: Box::new(expression),
        };
        return Some(stmt);
    }

    /// 式をパースする関数
    fn parse_expression(&self, opt: Opt) -> Option<Expression> {
        return match self.current_token.get_token_type() {
            TokenType::IDENT => self.parse_identifier(),
            // TODO ほかのパターンも実装
            _ => panic!("まだ実装していません"),
        };
    }

    /// 認識句用の式をパースする関数
    fn parse_identifier(&self) -> Option<Expression> {
        return Some(Expression::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.get_literal(),
        });
    }

    // エラー関係の関数群
    /// パースエラーを返す関数
    pub fn get_errors(&self) -> Vec<String> {
        return self.errors.clone();
    }
    ///  異常なトークンを検出した場合のエラー
    fn make_illegal_error(&mut self) {
        let msg = "異常なトークンを検出しました。".to_string();
        self.errors.push(msg);
    }

    /// 文のパースに失敗した場合のエラー
    fn make_statement_parse_error(&mut self) {
        let msg = "文をパースできませんでした。".to_string();
        self.errors.push(msg);
    }

    /// 先読み時に発生したエラー用をフォーマットを使って生成して追加する。
    fn make_peek_error(&mut self, expect_type: TokenType) {
        let msg = format!(
            "トークン型{:?}を期待して読み込みましたが、実際に読み込んだトークン型は{:?}でした。",
            expect_type,
            self.current_token.get_token_type()
        );
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod test {
    use crate::ast::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    /// パースエラーがあれば出力する関数
    fn check_parser_errors(parser: &Parser) {
        use std::io::Write;
        let errors = parser.get_errors();
        if errors.len() == 0 {
            return;
        }
        let mut e_writer = std::io::stderr();
        writeln!(
            e_writer,
            "\n\nパースエラーが{}件発生しました。",
            errors.len()
        )
            .unwrap();
        for error in errors {
            writeln!(e_writer, "{}", error).unwrap();
        }
        writeln!(e_writer, "").unwrap();
    }

    /// return 文の構文解析用のテスト
    #[test]
    fn test_return_statements() {
        let input = "
            return 5;
            return 10;
            return 993322;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program_opt = parser.parse_program();
        check_parser_errors(&parser);

        if program_opt.is_none() {
            assert!(false, "return文のパースに失敗しました。");
        }
        let program = program_opt.unwrap();
        let statements = &program.statements;
        if statements.len() != 3 {
            assert!(false, "return文の個数が不適切です。");
        }

        for stmt in program.statements {
            test_return_statement(&stmt);
        }
    }

    // 束縛される値は後でやるとして、束縛時の変数名をテストする関数
    fn test_return_statement(stmt: &Statement) {
        match stmt {
            Statement::ReturnStatement {
                token,
                return_value: _,
            } => {
                // トークンのreturnで始まってるか確認
                assert_eq!(token.get_literal(), "return");
                // TODO 戻り値の確認
                // assert_eq!(return_value.get_value(), "");
            }
            _ => {
                assert!(false, "return文ではありません。");
            }
        }
    }

    /// let文の構文解析用のテスト
    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program_opt = parser.parse_program();
        check_parser_errors(&parser);

        if program_opt.is_none() {
            assert!(false, "let文のパースに失敗しました。");
        }
        let program = program_opt.unwrap();
        let statements = &program.statements;
        if statements.len() != 3 {
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
        match stmt {
            Statement::LetStatement {
                token,
                name,
                value: _,
            } => {
                // トークンのletで始まってるか確認
                assert_eq!(token.get_literal(), "let");
                // 束縛変数名の確認
                assert_eq!(name.get_value(), test);
                // TODO 束縛された値の確認
                // assert_eq!(value.get_value(), "");
            }
            _ => {
                assert!(false, "let文ではありません。");
            }
        }
    }

    /// 識別子をパースするテスト
    #[test]
    pub fn test_identifier_expression() {
        let input = "foobar";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program_opt = parser.parse_program();
        check_parser_errors(&parser);
        if program_opt.is_none() {
            assert!(false, "識別子のパースに失敗しました。");
        }
        let program = program_opt.unwrap();

        if program.statements.len() != 1 {
            assert!(
                false,
                "適切な個数の識別子をパースすることができませんでした。"
            );
        }

        let stmt = &program.statements[0];
        if let Statement::ExpressionStatement {
            token: _,
            expression,
        } = stmt
        {
            if let Expression::Identifier {
                ref token,
                ref value,
            } = **expression
            {
                if &token.get_literal() != "foobar" {
                    assert!(false, "input's token literal is not \"foobar\"");
                }
                if value != "foobar" {
                    assert!(false, "token literal is not \"foobar\"");
                }
            }
        } else {
            assert!(false, "input is not expression-statement");
        }
    }
}

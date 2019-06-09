use crate::ast::*;
use crate::ast::Program;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

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
    CALL, // myFunction(x)
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
    /// 中置演算子の優先順位を返す関数
    fn precedences(token_type: &TokenType) -> Opt {
        match token_type {
            TokenType::EQ | TokenType::NEQ => Opt::EQUALS,
            TokenType::PLUS | TokenType::MINUS => Opt::SUM,
            TokenType::ASTERISK | TokenType::SLASH => Opt::PRODUCT,
            TokenType::LT | TokenType::GT => Opt::LESSGREATER,
            _ => Opt::LOWEST,
        }
    }

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
        };
        return parser;
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

    /// 次に読み込む演算子が中置演算子のトークンか調べる関数
    fn peek_token_is_infix(&self) -> bool {
        // 中置演算子の優先順位表をもちいて最低順位以外に変換できれば中置演算子ではない
        self.peek_precedence() != Opt::LOWEST
    }

    /// 現在読み込んでいるトークンの優先順位を取得する関数
    fn current_precedence(&self) -> Opt {
        Parser::precedences(&self.current_token.get_token_type())
    }

    /// 次に読み込むトークンの優先順位を取得する関数
    fn peek_precedence(&self) -> Opt {
        Parser::precedences(&self.peek_token.get_token_type())
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
    fn parse_expression(&mut self, precedence: Opt) -> Option<Expression> {
        let mut left = match self.current_token.get_token_type() {
            TokenType::IDENT => self.parse_identifier(),
            TokenType::INT => self.parse_integer_literal(),
            TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression(),
            _ => None,
        }?;

        loop {
            // 文末終了で抜けるか次に解析しようとしていた中置演算子の優先順位が今の優先順位より低いときに終了する
            // 括弧はまだ
            if self.peek_token_is(TokenType::SEMICOLON) || precedence >= self.peek_precedence() {
                break;
            }
            if self.peek_token_is_infix() {
                self.next_token();
                left = self.parse_infix_expression(left)?;
            } else {
                return Some(left);
            }
        }
        return Some(left);
    }

    /// 認識句用の式をパースする関数
    fn parse_identifier(&self) -> Option<Expression> {
        return Some(Expression::Identifier {
            token: self.current_token.clone(),
            value: self.current_token.get_literal(),
        });
    }

    /// 整数リテラルのパーサー
    fn parse_integer_literal(&self) -> Option<Expression> {
        let lit = self.current_token.get_literal().parse::<i64>().ok()?;
        return Some(Expression::IntegerLiteral {
            token: self.current_token.clone(),
            value: lit,
        });
    }

    /// 前置演算子付きの式をパースする関数
    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        // ここに来るということは前置演算子を持つ式だと確定してるはず
        let tok = self.current_token.clone();
        self.next_token();
        let exp = self.parse_expression(Opt::PREFIX)?;
        let expression = Expression::PrefixExpression {
            operator: tok.get_literal().clone(),
            token: tok,
            right_exp: Box::new(exp),
        };
        return Some(expression);
    }

    /// 中置演算子式を優先規則を元にパースする関数
    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let current = self.current_token.clone();
        self.next_token();
        let precedence = self.current_precedence();
        let expression = Expression::InfixExpression {
            operator: current.get_literal(),
            token: current,
            left_exp: Box::new(left),
            right_exp: Box::new(self.parse_expression(precedence)?),
        };
        return Some(expression);
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
    use crate::token::{Token, TokenType};

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
            assert!(false, "プログラムのパースに失敗しました。");
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
                    assert!(
                        false,
                        "入力から\"foobar\"識別子を得ることができませんでした"
                    );
                }
                if value != "foobar" {
                    assert!(
                        false,
                        "トークンのリテラルが\"foobar\"でありませんでした。"
                    );
                }
            }
        } else {
            assert!(false, "入力が式文ではありません");
        }
    }

    ///  整数リテラルをパースするテスト
    #[test]
    fn test_integer_literal_expression() {
        let input = "5";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program_opt = parser.parse_program();
        check_parser_errors(&parser);
        if program_opt.is_none() {
            assert!(false, "プログラムのパースに失敗しました。");
        }
        let program = program_opt.unwrap();

        if program.statements.len() != 1 {
            assert!(
                false,
                "適切な個数の整数リテラルをパースすることができませんでした。"
            );
        }

        let stmt = &program.statements[0];
        if let Statement::ExpressionStatement {
            token: _,
            expression,
        } = stmt
        {
            if let Expression::IntegerLiteral { ref token, value } = **expression {
                assert_eq!(token.get_literal(), "5");
                assert_eq!(value, 5_i64);
            }
        } else {
            assert!(false, "入力が式文ではありません");
        }
    }

    /// 前置演算子をパースするテスト
    #[test]
    fn test_prefix_expressions() {
        let prefix_tests = vec![
            // (input, operator_lit, int_val)
            ("!5;", "!", 5_i64),
            ("-15", "-", 15_i64),
        ];

        for (input, prefix, v) in prefix_tests {
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program_opt = parser.parse_program();
            check_parser_errors(&parser);
            if program_opt.is_none() {
                assert!(false, "プログラムのパースに失敗しました。");
            }
            let program = program_opt.unwrap();

            if program.statements.len() != 1 {
                assert!(
                    false,
                    "適切な個数の整数リテラルをパースすることができませんでした。"
                );
            }

            let stmt = &program.statements[0];
            if let Statement::ExpressionStatement {
                token: _,
                expression,
            } = stmt
            {
                if let Expression::PrefixExpression {
                    operator,
                    token,
                    right_exp: exp,
                } = &**expression
                {
                    assert_eq!(&token.get_literal(), operator);
                    assert_eq!(operator, prefix);
                    test_integer_literal(v, &**exp);
                }
            } else {
                assert!(false, "入力が式文ではありません");
            }
        }
    }

    /// 整数の前につく前置演算子をテストした際に整数値をテストするヘルパー関数
    fn test_integer_literal(v: i64, exp: &Expression) {
        if let Expression::IntegerLiteral { token, value } = exp {
            assert_eq!(token.get_token_type(), TokenType::INT);
            assert_eq!(token.get_literal(), format!("{}", v));
            assert_eq!(*value, v);
        } else {
            assert!(false, "整数リテラルではありませんでした。")
        }
    }


    /// 整数リテラルの中置演算子をパースするテスト
    #[test]
    fn test_infix_expressions() {
        let infix_tests = vec![
            // (input: &str, left_value: i64, operator: &str, right_value: i64)
            ("5 + 5;", 5_i64, "+", 5_i64),
            ("5 - 5;", 5_i64, "-", 5_i64),
            ("5 * 5;", 5_i64, "*", 5_i64),
            ("5 / 5;", 5_i64, "/", 5_i64),
            ("5 > 5;", 5_i64, ">", 5_i64),
            ("5 < 5;", 5_i64, "<", 5_i64),
            ("5 == 5;", 5_i64, "==", 5_i64),
            ("5 != 5;", 5_i64, "!=", 5_i64),
        ];

        for (input, left_value, infix_op, right_value) in infix_tests {
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program_opt = parser.parse_program();
            check_parser_errors(&parser);
            if program_opt.is_none() {
                assert!(false, "プログラムのパースに失敗しました。");
            }
            let program = program_opt.unwrap();

            if program.statements.len() != 1 {
                assert!(
                    false,
                    "適切な個数の整数リテラルをパースすることができませんでした。: {:?}",
                    program.statements
                );
            }

            let stmt = &program.statements[0];
            if let Statement::ExpressionStatement {
                token: _,
                expression,
            } = stmt
            {
                if let Expression::InfixExpression {
                    operator,
                    token,
                    left_exp,
                    right_exp,
                } = &**expression
                {
                    assert_eq!(&token.get_literal(), operator);
                    assert_eq!(operator, infix_op);
                    test_integer_literal(left_value, &**left_exp);
                    test_integer_literal(right_value, &**right_exp);
                }
            } else {
                assert!(false, "入力が式文ではありません");
            }
        }
    }
}

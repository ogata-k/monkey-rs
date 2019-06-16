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
    errors: Vec<String>, // パースして失敗したときのエラー文の集まり
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
            TokenType::LPAREN => Opt::CALL,
            _ => Opt::LOWEST,
        }
    }

    // 基本的な関数群
    /// 初期化関数
    pub fn new(mut lexer: Lexer) -> Self {
        let first = lexer.next_token();
        let second = lexer.next_token();
        let parser = Parser {
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
            TokenType::IF => self.parse_if_expression(),
            TokenType::FUNCTION => self.parse_function_literal(),
            TokenType::IDENT => self.parse_identifier(),
            TokenType::INT => self.parse_integer_literal(),
            TokenType::TRUE | TokenType::FALSE => self.parse_boolean_literal(),
            TokenType::BANG | TokenType::MINUS => self.parse_prefix_expression(),
            TokenType::LPAREN => self.parse_grouped_expression(),
            _ => None,
        }?;

        loop {
            // 文末終了で抜けるか次に解析しようとしていた中置演算子の優先順位が今の優先順位より低いときに終了する
            if self.peek_token_is(TokenType::SEMICOLON) || precedence >= self.peek_precedence() {
                break;
            }
            if self.peek_token_is_infix() {
                if self.peek_token_is(TokenType::LPAREN)
                    && (left.get_token().token_type_is(TokenType::FUNCTION)
                        || left.get_token().token_type_is(TokenType::IDENT))
                {
                    // 関数呼び出しの時
                    left = self.parse_call_expression(left)?;
                } else {
                    self.next_token();
                    left = self.parse_infix_expression(left)?;
                }
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

    /// 整数リテラルのパーサー
    fn parse_boolean_literal(&self) -> Option<Expression> {
        let lit = self.current_token.get_literal().parse::<bool>().ok()?;
        return Some(Expression::BooleanLiteral {
            token: self.current_token.clone(),
            value: lit,
        });
    }

    /// 関数リテラルのパーサー
    fn parse_function_literal(&mut self) -> Option<Expression> {
        // ここに来るときはFUNCTIONトークン型を読み込んでいる
        let tok = self.current_token.clone();
        self.next_token();
        if !self.current_token_is(TokenType::LPAREN) {
            return None;
        }
        self.next_token();
        let mut parameters = vec![];
        if !self.parse_function_parameters(&mut parameters) {
            return None;
        };
        if !self.peek_token_is(TokenType::LBRACE) {
            return None;
        }
        self.next_token();
        let body = self.parse_block_statement()?;
        return Some(Expression::FunctionLiteral {
            token: tok,
            parameters,
            body,
        });
    }

    /// 関数リテラルの引数部分のパーサー。成功時にtrueを返す。
    fn parse_function_parameters(&mut self, parameters: &mut Vec<Box<Expression>>) -> bool {
        if self.current_token_is(TokenType::RPAREN) {
            return true;
        }
        loop {
            let ident_opt = self.parse_identifier();
            if ident_opt.is_none() {
                return false;
            }
            parameters.push(Box::new(ident_opt.unwrap()));
            if self.peek_token_is(TokenType::COMMA) {
                self.next_token();
                self.next_token();
                continue;
            }
            if self.peek_token_is(TokenType::RPAREN) {
                self.next_token();
                return true;
            }
            if self.peek_token_is(TokenType::EOF) {
                // 右丸括弧の前に読み込みが最後まで終了することはないので、もし先に終了したら失敗扱い
                return false;
            }
        }
    }

    /// 関数呼び出しをパースする関数
    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let tok = self.current_token.clone();
        self.next_token();
        let mut arguments = vec![];
        if !self.current_token_is(TokenType::LPAREN) {
            return None;
        }
        self.next_token();
        if !self.parse_call_arguments(&mut arguments) {
            return None;
        }
        self.next_token();
        Some(Expression::CallExpression {
            token: tok,
            function: Box::new(function),
            arguments,
        })
    }

    /// 関数呼び出しの引数をパースする関数
    /// 成功ならtrue
    fn parse_call_arguments(&mut self, arguments: &mut Vec<Box<Expression>>) -> bool {
        if self.current_token_is(TokenType::RPAREN) {
            return true;
        }

        loop {
            let arg_opt = self.parse_expression(Opt::LOWEST);
            if arg_opt.is_none() {
                return false;
            }
            arguments.push(Box::new(arg_opt.unwrap()));
            if self.peek_token_is(TokenType::COMMA) {
                self.next_token();
                self.next_token();
                continue;
            }
            if self.peek_token_is(TokenType::RPAREN) {
                return true;
            }
            if self.peek_token_is(TokenType::EOF) {
                // 右丸括弧の前に読み込みが最後まで終了することはないので、もし先に終了したら失敗扱い
                return false;
            }
        }
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
        let precedence = self.current_precedence();
        self.next_token();
        let expression = Expression::InfixExpression {
            operator: current.get_literal(),
            token: current,
            left_exp: Box::new(left),
            right_exp: Box::new(self.parse_expression(precedence)?),
        };
        return Some(expression);
    }

    /// if-else文をパースするプログラム
    fn parse_if_expression(&mut self) -> Option<Expression> {
        // ここに入ってきたときにはIFトークンを読み込んでいる状態なので読み進める
        let tok = self.current_token.clone();
        self.next_token();
        let condition = self.parse_expression(Opt::LOWEST)?;
        if self.current_token_is(TokenType::RPAREN) {
            self.next_token();
            if !self.current_token_is(TokenType::LBRACE) {
                return None;
            }
            let consequence = self.parse_block_statement()?;
            self.next_token();
            let alt = if self.current_token_is(TokenType::ELSE) {
                self.next_token();
                if !self.current_token_is(TokenType::LBRACE) {
                    return None;
                }
                self.parse_block_statement()
            } else {
                None
            };
            if alt.is_some() {
                self.next_token();
            }
            return Some(Expression::IfExpression {
                token: tok,
                condition: Box::new(condition),
                consequence: Box::new(consequence),
                alternative: Box::new(alt),
            });
        }
        return None;
    }

    /// 波括弧に囲まれた部分をパースする
    fn parse_block_statement(&mut self) -> Option<Statement> {
        // ここに来るときは左波括弧のトークンを読み込んだ時
        let brace_tok = self.current_token.clone();
        let mut statements = vec![];
        self.next_token();
        loop {
            if self.current_token_is(TokenType::RBRACE) || self.current_token_is(TokenType::EOF) {
                break;
            }
            let stmt = self.parse_statement()?;
            statements.push(Box::new(stmt));
            // セミコロンなので読み飛ばす
            self.next_token();
        }
        let block = Statement::BlockStatement {
            token: brace_tok,
            statements,
        };
        return Some(block);
    }

    /// 丸括弧で囲まれたグループの式をパースする
    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        // ここに来るときは左丸括弧を読み込んだ時なのでひとつ消費して次から調べる
        self.next_token();
        let exp = self.parse_expression(Opt::LOWEST);
        if !self.peek_token_is(TokenType::RPAREN) {
            return None;
        }
        self.next_token();
        return exp;
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
    use crate::lexer::*;
    use crate::parser::*;
    use crate::token::*;

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
    fn test_identifier_expression() {
        let input = "foobar";
        test_identifier(input, "foobar")
    }

    ///  識別子をパースするテストのヘルパー関数
    fn test_identifier(input: &str, res: &str) {
        let lexer = Lexer::new(input);
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
                if &token.get_literal() != res {
                    assert!(
                        false,
                        "入力から\"{}\"識別子を得ることができませんでした",
                        res
                    );
                }
                if value != "foobar" {
                    assert!(
                        false,
                        "トークンのリテラルが\"{}\"でありませんでした。",
                        res
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

        let lexer = Lexer::new(input);
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

    ///  整数リテラルをパースするテスト
    #[test]
    fn test_boolean_literal_expression() {
        let tests = [("false", false), ("true", true)];

        for (input, res) in tests.to_vec().into_iter() {
            let lexer = Lexer::new(input);
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
                    "適切な個数のリテラルをパースすることができませんでした。"
                );
            }

            let stmt = &program.statements[0];
            if let Statement::ExpressionStatement {
                token: _,
                expression,
            } = stmt
            {
                if let Expression::BooleanLiteral { ref token, value } = **expression {
                    assert_eq!(token.get_literal(), res.to_string());
                    assert_eq!(value, res);
                }
            } else {
                assert!(false, "入力が式文ではありません");
            }
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
            let lexer = Lexer::new(input);
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
            let lexer = Lexer::new(input);
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

    /// if式のifブロックのみをパースするテスト
    #[test]
    fn test_if_expression() {
        let input = "if (x > y){ x; }";

        let lexer = Lexer::new(input);
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
                "適切な個数の文をパースすることができませんでした。: {:?}",
                program.statements
            );
        }
        if let Statement::ExpressionStatement {
            token: _,
            expression,
        } = &program.statements[0]
        {
            assert_eq!(expression.to_string(), "if (x > y) {x}");
            if let Expression::IfExpression {
                token: _,
                condition,
                consequence,
                alternative,
            } = &**expression
            {
                assert_eq!(condition.to_string(), "(x > y)");
                assert_eq!(consequence.to_string(), "{x}");
                assert!(alternative.is_none(), "else節が存在しています。");
            } else {
                assert!(
                    false,
                    "パース結果がif文ではありませんでした。"
                );
            }
        } else {
            assert!(false, "入力が式文ではありません。");
        }
    }

    /// if-else式をパースするテスト
    #[test]
    fn test_if_else_expression() {
        let input = "if (x > y){ x; }else{y;}";

        let lexer = Lexer::new(input);
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
                "適切な個数の文をパースすることができませんでした。: {:?}",
                program.statements
            );
        }
        if let Statement::ExpressionStatement {
            token: _,
            expression,
        } = &program.statements[0]
        {
            assert_eq!(expression.to_string(), "if (x > y) {x} else {y}");
            if let Expression::IfExpression {
                token: _,
                condition,
                consequence,
                alternative,
            } = &**expression
            {
                assert_eq!(condition.to_string(), "(x > y)");
                assert_eq!(consequence.to_string(), "{x}");
                if let Some(alt) = &**alternative {
                    assert_eq!(alt.to_string(), "{y}")
                } else {
                    assert!(false, "else節がうまく読み込めません。");
                }
            } else {
                assert!(
                    false,
                    "パース結果がif文ではありませんでした。"
                );
            }
        } else {
            assert!(false, "入力が式文ではありません。");
        }
    }

    /// 関数リテラルのパースをするテスト
    #[test]
    fn test_function_literal() {
        let tests = [
            // (input, expect)
            ("fn() {}", "fn() {}"),
            ("fn(x){}", "fn(x) {}"),
            ("fn(x, y) {}", "fn(x, y) {}"),
            ("fn(x, y) {x+y}", "fn(x, y) {(x + y)}"),
        ];

        for (input, expect) in tests.into_iter() {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program_opt = parser.parse_program();
            check_parser_errors(&parser);

            if program_opt.is_none() {
                assert!(
                    false,
                    "プログラムをパースできませんでした。"
                );
            }
            let program = program_opt.unwrap();
            if program.statements.len() != 1 {
                assert!(
                    false,
                    "適切な個数の文をパースすることができませんでした。: {:?}",
                    program.statements
                );
            }
            if let Statement::ExpressionStatement {
                token: _,
                expression,
            } = &program.statements[0]
            {
                assert_eq!(expression.to_string(), expect.to_string());
                if let Expression::FunctionLiteral {
                    token,
                    parameters: _,
                    body: _,
                } = &**expression
                {
                    assert!(token.token_type_is(TokenType::FUNCTION));
                } else {
                    assert!(false, "関数リテラルではありませんでした。");
                }
            } else {
                assert!(false, "入力が式文ではありません。");
            }
        }
    }

    /// 関数呼び出しのパーステスト
    #[test]
    fn test_call_expression() {
        let tests = [
            // (input, expect)
            ("add()", "add()"),
            ("add(1, 2 * 3, 4 + 5)", "add(1, (2 * 3), (4 + 5))"),
            ("a + add(b*c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4+5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "sub(a + b + c * d / f + g)",
                "sub((((a + b) + ((c * d) / f)) + g))",
            ),
            ("fn(a, b) {a + b;}(3, 4)", "fn(a, b) {(a + b)}(3, 4)"),
        ];
        for (input, expect) in tests.to_vec().into_iter() {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program_opt = parser.parse_program();
            check_parser_errors(&parser);

            if program_opt.is_none() {
                assert!(
                    false,
                    "プログラムをパースできませんでした。"
                );
            }
            let program = program_opt.unwrap();
            println!("program: {}", program.to_string());
            if program.statements.len() != 1 {
                assert!(
                    false,
                    "適切な個数の文をパースすることができませんでした。: {:?} => {}",
                    program.statements,
                    program.to_string()
                );
            }
            if let Statement::ExpressionStatement {
                token: _,
                expression,
            } = &program.statements[0]
            {
                assert_eq!(expression.to_string(), expect.to_string());
            } else {
                assert!(false, "入力が式文ではありません。");
            }
        }
    }

    /// 括弧と関数を除いて、異なる優先度で式をパースできているかのテスト
    #[test]
    fn test_operator_precedences() {
        let tests = [
            // (input, expect)
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("a + b + -c", "((a + b) + (-c))"),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("5 < 3 != !!true", "((5 < 3) != (!(!true)))"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5+5)*2", "((5 + 5) * 2)"),
            ("2 / ( 5 - 5)", "(2 / (5 - 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];

        for (input, expect) in tests.iter() {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program_opt = parser.parse_program();
            check_parser_errors(&parser);
            if program_opt.is_none() {
                assert!(
                    false,
                    "プログラムをパースすることができませんでした。"
                );
            }
            let program = program_opt.unwrap();
            let actual = program.to_string();
            assert!(
                &actual == *expect,
                "{} => {:?}\n{} ?= {}",
                input,
                program,
                actual,
                expect
            );
            assert_eq!(&actual, *expect);
        }
    }
}

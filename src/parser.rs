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
    pub fn parse_program(&mut self) -> Option<Program> {
        // 簡易的にOption型にしているがResultを返すように修正してもよい
        unimplemented!()
    }
}

#[cfg(test)]
mod test{
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
            assert!(false,"let文のパースに失敗しました。");
        }
        let program = program_opt.unwrap();
        if program.get_statements().len() != 3 {
            assert!(false, "let文の個数が不適切です。");
        }

        let tests = ["x", "y", "foobar"];

        for (i, test) in tests.iter().enumerate() {
            let stmt = &(program.get_statements()[i]);
            test_let_statement(stmt, test);
        }
    }

    // 束縛される値は後でやるとして、束縛時の変数名をテストする関数
    fn test_let_statement(stmt: &Statement, test: &str){
        match stmt {
            Statement::LetStatement{token, name, value} => {
                // トークンのletで始まってるか確認
                assert_eq!(token.get_literal(), "test");
                // 束縛変数名の確認
                assert_eq!(name.get_value(), test);
                // TODO 束縛された値の確認
                // assert_eq!(value.get_value(), "");


            },
            _ => {assert!(false, "let文ではありません。");}
        }
    }
}
use crate::token::Token;

/// ノード
pub trait Node {
    fn token_literal(&self) -> String;
}

/// 文用のノード
#[derive(Debug, PartialEq)]
pub enum Statement {
    // ここにStatementに関する構造体を定義していく
    /// let文用のノード
    /// <token> <name> = <value>;
    /// つまり、let <name> = <value>;
    LetStatement {
        token: Token,  // Token::LET
        name: Box<Expression>,  // 束縛対象の変数名、Expression::Identifierのみ
        value: Box<Expression>,  // 束縛する対象
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement{token, name, value} => {
                token.get_literal()
            }
        }
    }
}

/// 式用のノード
#[derive(Debug, PartialEq)]
pub enum Expression {
    // ここにExpressionに関する構造体を定義していく
    /// 識別子を表すノード
    Identifier {
        token: Token, // Token::Ident
        value: String,  // 識別子が保持する値
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier{token, value} => {
                token.get_literal()
            }
        }
    }
}

impl Expression {
    /// 保持する値のゲッター
    pub fn get_value(&self) -> String{
        match self {
            Expression::Identifier{token, value} => {
                value.to_string()
            }
        }
    }
}

/// Monkeyプログラムをあらわす構造体
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    /// 初期化関数
    pub fn new() -> Program {
        return Program{
            statements: Vec::new(),
        };
    }


/// ルートノードであるprogramノードのリテラルを返す
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return "".to_string();
        }
    }
}

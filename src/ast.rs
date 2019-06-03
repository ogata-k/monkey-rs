use std::fmt::Write;

use crate::token::Token;

/// ノード
pub trait Node: ToString {
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
        // Token::LET
        token: Token,
        name: Box<Expression>,
        // 束縛対象の変数名、Expression::Identifierのみ
        value: Box<Expression>, // 束縛する対象
    },
    /// return文用のノード
    /// <token> <return_value>;
    /// つまり、return <return_value>;
    ReturnStatement {
        // Token::Return
        token: Token,
        return_value: Box<Expression>, // 戻り値
    },
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        match self {
            Statement::LetStatement { token, name, value } => {
                write!(s, "{}", token.get_literal() + " ").unwrap();
                write!(s, "{}", name.to_string()).unwrap();
                let v = value.to_string();
                if v != "".to_string() {
                    write!(s, " {} {}", "=", &v).unwrap();
                }
                write!(s, "{}", ";").unwrap();
            }
            Statement::ReturnStatement { token, return_value } => {
                write!(s, "{}", token.get_literal() + " ").unwrap();
                let v = return_value.to_string();
                if v != "".to_string() {
                    write!(s, " {} {}", " =", &v).unwrap();
                }
                write!(s, "{}", ";").unwrap();
            },
        }
        return s;
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement { token, name: _, value: _ } => token.get_literal(),
            Statement::ReturnStatement { token, return_value: _ } => token.get_literal(),
        }
    }
}

/// 式用のノード
#[derive(Debug, PartialEq)]
pub enum Expression {
    // ここにExpressionに関する構造体を定義していく
    NonValue,
    /// 識別子を表すノード
    Identifier {
        token: Token,
        // Token::Ident
        value: String, // 識別子が保持する値
    },
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        match self {
            Expression::NonValue => {
                write!(s, "{}", "").unwrap();
            },
            Expression::Identifier { token: _, value } => {
                write!(s, "{}", value).unwrap();
            }
        }
        return s;
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier { token, value: _ } => token.get_literal(),
            Expression::NonValue => { "".to_string() }
        }
    }
}

impl Expression {
    /// 保持する値のゲッター
    pub fn get_value(&self) -> String {
        match self {
            Expression::Identifier { token: _, value } => value.to_string(),
            Expression::NonValue => "".to_string(),
        }
    }
}

/// 式文用のノード
#[derive(Debug, PartialEq)]
pub enum ExpressionStatement {
    ExpressionStatement {
        token: Token,
        // 式の最初のトークン
        expression: Expression,
    }
}

impl ToString for ExpressionStatement {
    fn to_string(&self) -> String {
        match self {
            ExpressionStatement::ExpressionStatement { token: _, expression } => {
                if *expression != Expression::NonValue {
                    return expression.to_string();
                }
                return "".to_string();
            }
        }
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        match self {
            ExpressionStatement::ExpressionStatement { token, expression: _ } => token.get_literal(),
        }
    }
}

/// Monkeyプログラムをあらわす構造体
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl ToString for Program {
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        for stmt in self.statements.iter() {
            write!(s, "{}", stmt.to_string()).unwrap();
        }
        return s;
    }
}

impl Program {
    /// 初期化関数
    pub fn new() -> Program {
        return Program {
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

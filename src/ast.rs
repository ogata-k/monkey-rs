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
    LetStatement {
        token: Token,
        // nameはExpression::Identifier限定なのでつくるときに判定を
        name: Box<Expression>,
        value: Box<Expression>,
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
        token: Token,
        value: String,
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

/// Monkeyプログラムをあらわす構造体
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return "".to_string();
        }
    }
}

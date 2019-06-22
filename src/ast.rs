use std::fmt::Write;

use crate::token::Token;

/// ノード
pub trait Node: ToString {
    fn token_literal(&self) -> String;
    fn get_token(&self) -> Token;
}

/// 文用のノード
#[derive(Debug, PartialEq)]
pub enum Statement {
    // ここにStatementに関する構造体を定義していく
    ExpressionStatement {
        token: Token,
        // 式の最初のトークン
        expression: Box<Expression>,
    },
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
    /// 波括弧の中にあるいくつかの式の集まり
    BlockStatement {
        token: Token,
        statements: Vec<Box<Statement>>,
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
            Statement::ReturnStatement {
                token,
                return_value,
            } => {
                write!(s, "{}", token.get_literal() + " ").unwrap();
                let v = return_value.to_string();
                if v != "".to_string() {
                    write!(s, " {} {}", " =", &v).unwrap();
                }
                write!(s, "{}", ";").unwrap();
            }
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => {
                write!(s, "{};", expression.to_string()).unwrap();
            }
            Statement::BlockStatement {
                token: _,
                statements,
            } => {
                write!(s, "{{").unwrap();
                for stmt in statements.into_iter() {
                    write!(s, "{}", stmt.to_string()).unwrap();
                }
                write!(s, "}}").unwrap();
            }
        }
        return s;
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement {
                token,
                name: _,
                value: _,
            } => token.get_literal(),
            Statement::ReturnStatement {
                token,
                return_value: _,
            } => token.get_literal(),
            Statement::ExpressionStatement {
                token,
                expression: _,
            } => token.get_literal(),
            Statement::BlockStatement {
                token,
                statements: _,
            } => token.get_literal(),
        }
    }

    fn get_token(&self) -> Token {
        let tok = match self {
            Statement::LetStatement {
                token,
                name: _,
                value: _,
            } => token,
            Statement::ExpressionStatement {
                token,
                expression: _,
            } => token,
            Statement::ReturnStatement {
                token,
                return_value: _,
            } => token,
            Statement::BlockStatement {
                token,
                statements: _,
            } => token,
        };
        return tok.clone();
    }
}

/// 式用のノード
#[derive(Debug, PartialEq)]
pub enum Expression {
    // ここにExpressionに関する構造体を定義していく
    /// 識別子を表すノード
    Identifier {
        token: Token,
        // Token::Ident
        value: String, // 識別子が保持する値
    },
    /// 整数リテラル用のノード
    IntegerLiteral { token: Token, value: i64 },
    /// 真偽値リテラル用のノード
    BooleanLiteral { token: Token, value: bool },
    /// 関数リテラル用のノード
    FunctionLiteral {
        token: Token,
        // Expression::Identifierの集まり
        parameters: Vec<Box<Expression>>,
        // 関数本体。Statement::BlockStatementのこと
        body: Statement,
    },
    /// 前置演算子式用のノード
    PrefixExpression {
        // 判断に使ったトークン
        token: Token,
        // 演算子の記号
        operator: String,
        // 前置演算子の引数(すなわち、右辺式)
        right_exp: Box<Expression>,
    },
    /// 中置算子式用のノード
    InfixExpression {
        // 判断に使ったトークン
        token: Token,
        // 演算子の記号
        operator: String,
        // 左辺式
        left_exp: Box<Expression>,
        // 右辺式
        right_exp: Box<Expression>,
    },
    /// IF式用のノード
    /// ELSEがあればAlternativeに追加
    IfExpression {
        token: Token,
        condition: Box<Expression>,
        // Statement::BlockStatementでStatementの集まりを表す。
        consequence: Box<Statement>,
        // Else節。Statement::BlockStatementでStatementの集まりを表す。
        alternative: Box<Option<Statement>>,
    },
    /// 関数呼び出し式用のノード
    CallExpression {
        // '('トークン
        token: Token,
        // Expression::Identifier または Expression::FunctionLiteral
        function: Box<Expression>,
        arguments: Vec<Box<Expression>>,
    },
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        match self {
            Expression::Identifier { token: _, value } => {
                write!(s, "{}", value).unwrap();
            }
            Expression::IntegerLiteral { token: _, value } => {
                write!(s, "{}", value).unwrap();
            }
            Expression::BooleanLiteral { token: _, value } => {
                write!(s, "{}", value).unwrap();
            }
            Expression::FunctionLiteral {
                token,
                parameters,
                body,
            } => {
                write!(s, "{}(", token.get_literal()).unwrap();
                for (i, parameter) in parameters.into_iter().enumerate() {
                    if i == 0 {
                        write!(s, "{}", parameter.to_string()).unwrap();
                    } else {
                        write!(s, ", {}", parameter.to_string()).unwrap();
                    }
                }
                write!(s, ")").unwrap();
                write!(s, "{}", body.to_string()).unwrap();
            }
            Expression::PrefixExpression {
                token: _,
                operator,
                right_exp,
            } => {
                write!(s, "({}{})", operator, right_exp.to_string()).unwrap();
            }
            Expression::InfixExpression {
                token: _,
                operator,
                left_exp,
                right_exp,
            } => {
                write!(
                    s,
                    "({} {} {})",
                    left_exp.to_string(),
                    operator,
                    right_exp.to_string()
                )
                .unwrap();
            }
            Expression::IfExpression {
                token: _,
                condition,
                consequence,
                alternative,
            } => {
                write!(s, "if {}{}", condition.to_string(), consequence.to_string()).unwrap();
                if let Some(ref alt) = **alternative {
                    write!(s, " else{}", alt.to_string()).unwrap();
                }
            }
            Expression::CallExpression {
                token: _,
                function,
                arguments,
            } => {
                write!(s, "{}", function.to_string()).unwrap();
                write!(s, "(").unwrap();
                for (i, arg) in arguments.into_iter().enumerate() {
                    if i == 0 {
                        write!(s, "{}", arg.to_string()).unwrap();
                    } else {
                        write!(s, ", {}", arg.to_string()).unwrap();
                    }
                }
                write!(s, ")").unwrap();
            }
        }
        return s;
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier { token, value: _ } => token.get_literal(),
            Expression::IntegerLiteral { token, value: _ } => token.get_literal(),
            Expression::BooleanLiteral { token, value: _ } => token.get_literal(),
            Expression::FunctionLiteral {
                token,
                parameters: _,
                body: _,
            } => token.get_literal(),
            Expression::PrefixExpression {
                token,
                operator: _,
                right_exp: _,
            } => token.get_literal(),
            Expression::InfixExpression {
                token,
                operator: _,
                left_exp: _,
                right_exp: _,
            } => token.get_literal(),
            Expression::IfExpression {
                token,
                condition: _,
                consequence: _,
                alternative: _,
            } => token.get_literal(),
            Expression::CallExpression {
                token,
                function: _,
                arguments: _,
            } => token.get_literal(),
        }
    }

    fn get_token(&self) -> Token {
        let tok = match self {
            Expression::Identifier { token, value: _ } => token,
            Expression::IntegerLiteral { token, value: _ } => token,
            Expression::BooleanLiteral { token, value: _ } => token,
            Expression::FunctionLiteral {
                token,
                parameters: _,
                body: _,
            } => token,
            Expression::PrefixExpression {
                token,
                operator: _,
                right_exp: _,
            } => token,
            Expression::InfixExpression {
                token,
                operator: _,
                left_exp: _,
                right_exp: _,
            } => token,
            Expression::IfExpression {
                token,
                condition: _,
                consequence: _,
                alternative: _,
            } => token,
            Expression::CallExpression {
                token,
                function: _,
                arguments: _,
            } => token,
        };
        return tok.clone();
    }
}

impl Expression {
    /// 保持する値のゲッター
    pub fn get_value(&self) -> String {
        match self {
            Expression::Identifier { token: _, value } => value.to_string(),
            Expression::IntegerLiteral { token: _, value } => format!("{}", value),
            Expression::BooleanLiteral { token: _, value } => format!("{}", value),
            Expression::FunctionLiteral {
                token: _,
                parameters: _,
                body: _,
            } => "".to_string(),
            Expression::PrefixExpression {
                token: _,
                operator,
                right_exp: _,
            } => operator.to_string(),
            Expression::InfixExpression {
                token: _,
                operator,
                left_exp: _,
                right_exp: _,
            } => operator.to_string(),
            Expression::IfExpression {
                token: _,
                condition: _,
                consequence: _,
                alternative: _,
            } => "".to_string(),
            Expression::CallExpression {
                token: _,
                function,
                arguments: _,
            } => function.to_string(),
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

#[cfg(test)]
mod test {
    use crate::ast::*;
    use crate::token::*;

    #[test]
    fn test_to_string() {
        let program = Program {
            statements: vec![Statement::LetStatement {
                token: Token::new(TokenType::LET, "let"),
                name: Box::new(Expression::Identifier {
                    token: Token::new(TokenType::IDENT, "myVar"),
                    value: "myVar".to_string(),
                }),
                value: Box::new(Expression::Identifier {
                    token: Token::new(TokenType::IDENT, "anotherVar"),
                    value: "anotherVar".to_string(),
                }),
            }],
        };
        assert_eq!(program.to_string(), "let myVar = anotherVar;".to_string());
    }
}

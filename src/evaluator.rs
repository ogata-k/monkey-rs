use crate::ast::{Expression, Program, Statement};
use crate::object::Object;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Eval {}

impl Eval {
    pub fn eval_program(program: &Program) -> Object {
        Self::eval_statements(&program.statements)
    }

    fn eval_statements(statements: &Vec<Statement>) -> Object {
        let mut result = Object::NULL;

        for statement in statements {
            result = Self::eval_statement(&statement);
            if result.get_type().is_return_value() {
                break;
            }
        }
        result
    }

    fn eval_statement(statement: &Statement) -> Object {
        let mut result = Object::NULL;

        match statement {
            stmt @ Statement::ExpressionStatement {
                token: _,
                expression: _,
            } => {
                result = Self::eval_expression_statement(stmt);
            }
            stmt @ Statement::LetStatement {
                token: _,
                name: _,
                value: _,
            } => unimplemented!(),
            Statement::ReturnStatement {
                token: _,
                return_value,
            } => {
                result = Self::eval_return_statement(return_value);
            },
            stmt @ Statement::BlockStatement {
                token: _,
                statements: _,
            } => {
                result = Self::eval_block_statement(&stmt);
            }
        }
        result
    }

    fn eval_expression_statement(statement: &Statement) -> Object {
        let mut result = Object::NULL;
        match statement {
            Statement::ExpressionStatement {
                token: _,
                expression: exp,
            } => {
                result = Self::eval_expression(exp);
            }
            _ => unreachable!(),
        }
        result
    }

    fn eval_return_statement(return_value: &Expression) -> Object {
        let value = Eval::eval_expression(return_value);
        Object::ReturnValue {value: Box::new(value)}
    }

    fn eval_block_statement(block: &Statement) -> Object {
        let mut result = Object::NULL;
        if let Statement::BlockStatement { token: _, statements} = block{
            for statement in statements {
                result = Self::eval_statement(&statement);
            }
        }
        result
    }

    fn eval_expression(expression: &Expression) -> Object {
        let mut result = Object::NULL;
        match expression {
            Expression::Identifier { token: _, value: _ } => unimplemented!(),
            Expression::IntegerLiteral { token: _, value } => {
                result = Object::Integer { value: *value };
            }
            Expression::BooleanLiteral { token: _, value } => {
                if *value {
                    result = Object::BOOLEAN_TRUE;
                } else {
                    result = Object::BOOLEAN_FALSE;
                }
            }
            Expression::FunctionLiteral {
                token: _,
                parameters: _,
                body: _,
            } => unimplemented!(),
            Expression::PrefixExpression {
                token: _,
                operator,
                right_exp,
            } => {
                let right = Eval::eval_expression(right_exp);
                result  = Eval::eval_prefix_expression(&operator, &right);
            },
            Expression::InfixExpression {
                token: _,
                operator,
                left_exp,
                right_exp,
            } => {
                let left = Eval::eval_expression(left_exp);
                let right = Eval::eval_expression(right_exp);
                result = Eval::eval_infix_expression(&operator, &left, &right);
            },
            Expression::IfExpression {
                token: _,
                condition,
                consequence,
                alternative,
            } => {
                let cond = Eval::eval_expression(condition);

                if cond.is_truthy() {
                    return Eval::eval_statement(consequence);
                } else {
                    if let Some(alt) = &**alternative {
                        return Eval::eval_statement(alt);
                    } else {
                        return Object::Null;
                    }
                }
            },
            Expression::CallExpression {
                token: _,
                function: _,
                arguments: _,
            } => unimplemented!(),
        }
        result
    }

    fn eval_prefix_expression(operator: &str, right: &Object) -> Object {
        match operator {
            "!" => Eval::eval_bang_operation(right),
            "-" => Eval::eval_minus_operation(right),
            _ => Object::NULL,
        }
    }

    fn eval_bang_operation(right: &Object) -> Object {
        match right {
            Object::Boolean {value} => {
                if *value {
                    Object::BOOLEAN_FALSE
                } else {
                    Object::BOOLEAN_TRUE
                }
            },
            _ => Object::BOOLEAN_FALSE,
        }
    }

    fn eval_minus_operation(right: &Object) -> Object {
        match right {
            Object::Integer{value} => Object::Integer{value: -(*value)},
            _ => Object::NULL,
        }
    }

    fn eval_infix_expression(operator: &str, left: &Object, right: &Object) -> Object {
        let left_type = left.get_type();
        let right_type = right.get_type();
        if left_type.is_integer() && right_type.is_integer() {
            Eval::eval_integer_infix_expression(operator, left, right)
        } else if left_type.is_boolean() && right_type.is_boolean() {
            Eval::eval_boolean_infix_expression(operator, left, right)
        } else {
            // TODO others
            Object::NULL
        }
    }

    fn eval_integer_infix_expression(operator: &str, left: &Object, right: &Object) -> Object {
        let left_int = left.inspect().parse::<i64>().unwrap();
        let right_int = right.inspect().parse::<i64>().unwrap();
        match operator {
            "+" => Object::Integer { value: left_int + right_int},
            "-" => Object::Integer { value: left_int - right_int},
            "*" => Object::Integer { value: left_int * right_int},
            "/" => Object::Integer { value: left_int / right_int},
            "<" => Object::Boolean { value: left_int < right_int},
            ">" => Object::Boolean { value: left_int > right_int},
            "==" => Object::Boolean { value: left_int == right_int},
            "!=" => Object::Boolean { value: left_int != right_int},
            _ => Object::NULL,
        }
    }

    fn eval_boolean_infix_expression(operator: &str, left: &Object, right: &Object) -> Object {
        let left_bool = left.inspect().parse::<bool>().unwrap();
        let right_bool = right.inspect().parse::<bool>().unwrap();
        match operator {
            "==" => Object::Boolean { value: left_bool == right_bool},
            "!=" => Object::Boolean { value: left_bool != right_bool},
            _ => Object::NULL,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::evaluator::Eval;
    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;

    #[test]
    fn test_eval_integer_expression() {
        let tests = [
            ("5;", Object::Integer { value: 5 }),
            ("10;", Object::Integer { value: 10 }),
            ("-5;", Object::Integer { value: -5 }),
            ("-10;", Object::Integer { value: -10 }),
            ("5 + 5 + 5 + 5 - 10;", Object::Integer {value: 10}),
            ("2 * 2 * 2 * 2 * 2;", Object::Integer {value: 32}),
            ("-50 + 100 + -50;", Object::Integer {value: 0}),
            ("5 * 2 + 10;", Object::Integer {value: 20}),
            ("5 + 2 * 10;", Object::Integer {value: 25}),
            ("20 + 2 * -10;", Object::Integer {value: 0}),
            ("50 / 2 * 2 + 10;", Object::Integer {value: 60}),
            ("2 * (5 + 10);", Object::Integer {value: 30}),
            ("3 * 3 * 3 + 10;", Object::Integer {value: 37}),
            ("3 * (3 * 3 + 10);", Object::Integer {value: 57}),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10;", Object::Integer {value: 50}),
            ("1 < 2;", Object::Boolean { value: true }),
            ("1 > 2;", Object::Boolean { value: false }),
            ("1 < 1;", Object::Boolean { value: false }),
            ("1 > 1;", Object::Boolean { value: false }),
            ("1 == 1;", Object::Boolean { value: true }),
            ("1 != 1;", Object::Boolean { value: false }),
            ("1 == 2;", Object::Boolean { value: false }),
            ("1 != 2;", Object::Boolean { value: true }),
        ];

        do_test(&tests);
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = [
            ("true;", Object::Boolean { value: true }),
            ("false;", Object::Boolean { value: false }),
            ("true == true;", Object::Boolean { value: true }),
            ("false == false;", Object::Boolean { value: true }),
            ("true != false;", Object::Boolean { value: true }),
            ("false != true;", Object::Boolean { value: true }),
            ("(1 < 2) == true;", Object::Boolean { value: true }),
            ("(1 < 2) == false;", Object::Boolean { value: false }),
            ("(1 > 2) == true;", Object::Boolean { value: false }),
            ("(1 > 2) == false;", Object::Boolean { value: true }),
        ];

        do_test(&tests);
    }

    #[test]
    fn test_bang_operator() {
        let tests = [
            ("!true;", Object::BOOLEAN_FALSE),
            ("!false;", Object::BOOLEAN_TRUE),
            ("!5;", Object::BOOLEAN_FALSE),
            ("!!true;", Object::BOOLEAN_TRUE),
            ("!!false;", Object::BOOLEAN_FALSE),
            ("!!5;", Object::BOOLEAN_TRUE),
        ];
        do_test(&tests);
    }

    #[test]
    fn test_if_expressions() {
        let tests = [
            ("if (true) {10;};", Object::Integer {value: 10}),
            ("if (false) {10;};", Object::NULL),
            ("if (1) {10;};", Object::Integer {value: 10}),
            ("if (1 < 2) {10;};", Object::Integer {value: 10}),
            ("if (1 > 2) {10;};", Object::NULL),
            ("if (1 < 2) {10;} else {20;};", Object::Integer {value: 10}),
            ("if (1 > 2) {10;} else {20;};", Object::Integer {value: 20}),
        ];
        do_test(&tests);
    }

    #[test]
    fn test_eval_return_statements() {
        let tests = [
            ("return 5;", Object::ReturnValue {value: Box::new(Object::Integer { value: 5 })}),
            ("return 10;", Object::ReturnValue {value: Box::new(Object::Integer { value: 10 })}),
            ("5; return 5;", Object::ReturnValue {value: Box::new(Object::Integer { value: 5 })}),
            ("return 5; 5;", Object::ReturnValue {value: Box::new(Object::Integer { value: 5 })}),
            ("5; return 5; 5;", Object::ReturnValue {value: Box::new(Object::Integer { value: 5 })}),
            ("return -5;", Object::ReturnValue {value: Box::new(Object::Integer { value: -5 })}),
            ("return -10;", Object::ReturnValue {value: Box::new(Object::Integer { value: -10 })}),
            ("return 5 + 5 + 5 + 5 - 10;", Object::ReturnValue {value: Box::new(Object::Integer {value: 10})}),
            ("return 2 * 2 * 2 * 2 * 2;", Object::ReturnValue {value: Box::new(Object::Integer {value: 32})}),
            ("return -50 + 100 + -50;", Object::ReturnValue {value: Box::new(Object::Integer {value: 0})}),
            ("return 5 * 2 + 10;", Object::ReturnValue {value: Box::new(Object::Integer {value: 20})}),
            ("return 5 + 2 * 10;", Object::ReturnValue {value: Box::new(Object::Integer {value: 25})}),
            ("return 20 + 2 * -10;", Object::ReturnValue {value: Box::new(Object::Integer {value: 0})}),
            ("return 50 / 2 * 2 + 10;", Object::ReturnValue {value: Box::new(Object::Integer {value: 60})}),
            ("return 2 * (5 + 10);", Object::ReturnValue {value: Box::new(Object::Integer {value: 30})}),
            ("return 3 * 3 * 3 + 10;", Object::ReturnValue {value: Box::new(Object::Integer {value: 37})}),
            ("return 3 * (3 * 3 + 10);", Object::ReturnValue {value: Box::new(Object::Integer {value: 57})}),
            ("return (5 + 10 * 2 + 15 / 3) * 2 + -10;", Object::ReturnValue {value: Box::new(Object::Integer {value: 50})}),
            ("return 1 < 2;", Object::ReturnValue {value: Box::new(Object::Boolean { value: true })}),
            ("return 1 > 2;", Object::ReturnValue {value: Box::new(Object::Boolean { value: false })}),
            ("return 1 < 1;", Object::ReturnValue {value: Box::new(Object::Boolean { value: false })}),
            ("return 1 > 1;", Object::ReturnValue {value: Box::new(Object::Boolean { value: false })}),
            ("return 1 == 1;", Object::ReturnValue {value: Box::new(Object::Boolean { value: true })}),
            ("return 1 != 1;", Object::ReturnValue {value: Box::new(Object::Boolean { value: false })}),
            ("return 1 == 2;", Object::ReturnValue {value: Box::new(Object::Boolean { value: false })}),
            ("return 1 != 2;", Object::ReturnValue {value: Box::new(Object::Boolean { value: true })}),
        ];

        do_test(&tests);
    }

    fn test_eval(input: &str) -> Object {
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        Eval::eval_program(&program.expect("fail parse program."))
    }

    fn do_test(tests: &[(&str, Object)]) {
        for (input, expected) in tests.to_vec() {
            let evaluated = test_eval(input);
            assert_eq!(evaluated, expected);
        }
    }
}

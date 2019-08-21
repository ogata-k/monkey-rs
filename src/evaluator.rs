use crate::ast::{Expression, Program, Statement};
use crate::object::{Object, ObjectType};

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
            stmt @ Statement::ReturnStatement {
                token: _,
                return_value: _,
            } => unimplemented!(),
            stmt @ Statement::BlockStatement {
                token: _,
                statements: _,
            } => unimplemented!(),
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
                condition: _,
                consequence: _,
                alternative: _,
            } => unimplemented!(),
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
        } else {
            // TODO others
            Object::NULL
        }
    }

    fn eval_integer_infix_expression(operator: &str, left: &Object, right: &Object) -> Object {
        match operator {
            "+" => Object::Integer { value: left.inspect().parse::<i64>().unwrap() + right.inspect().parse::<i64>().unwrap()},
            "-" => Object::Integer { value: left.inspect().parse::<i64>().unwrap() - right.inspect().parse::<i64>().unwrap()},
            "*" => Object::Integer { value: left.inspect().parse::<i64>().unwrap() * right.inspect().parse::<i64>().unwrap()},
            "/" => Object::Integer { value: left.inspect().parse::<i64>().unwrap() / right.inspect().parse::<i64>().unwrap()},
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
        ];

        do_test(&tests);
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = [
            ("true;", Object::Boolean { value: true }),
            ("false;", Object::Boolean { value: false }),
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

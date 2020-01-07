use crate::ast::{BlockStatement, Expression, Program, Statement};
use crate::object::Object;

pub fn eval(program: Program) -> Object {
    let mut result = Object::Null;
    for statement in program.statements {
        result = eval_statement(statement);
    }
    result
}

fn eval_statement(statement: Statement) -> Object {
    match statement {
        Statement::Expression(expr) => eval_expression(expr),
        _ => Object::Null,
    }
}

fn eval_expression(expression: Expression) -> Object {
    match expression {
        Expression::IntegerLiteral(i) => Object::Integer(i),
        Expression::Boolean(b) => Object::Boolean(b),
        Expression::Prefix(operator, right) => {
            let r = eval_expression(*right);
            eval_prefix_expression(&operator, r)
        }
        _ => Object::Null,
    }
}

fn eval_block_statement(block_statement: BlockStatement) -> Object {
    Object::Null
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Object::Null,
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean(true) => Object::Boolean(false),
        Object::Boolean(false) => Object::Boolean(true),
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(i) => Object::Integer(-i),
        _ => Object::Null,
    }
}

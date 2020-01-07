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
        _ => Object::Null,
    }
}

fn eval_block_statement(block_statement: BlockStatement) -> Object {
    Object::Null
}

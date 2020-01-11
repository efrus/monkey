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
        Expression::Infix(left, operator, right) => {
            let l = eval_expression(*left);
            let r = eval_expression(*right);
            eval_infix_expression(&operator, l, r)
        }
        Expression::IfExpression(condition, consequence, alt) => {
            let c = eval_expression(*condition);
            if is_truthy(c) {
                return eval_block_statement(consequence);
            }
            match alt {
                Some(val) => eval_block_statement(val),
                None => Object::Null,
            }
        }
        _ => Object::Null,
    }
}

fn eval_block_statement(block_statement: BlockStatement) -> Object {
    for statement in block_statement.statements {
        let result = eval_statement(statement);
        return result;
        /*match result {
            Object::
        }*/
    }
    Object::Null
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Object::Null,
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    if let Object::Integer(_) = left {
        if let Object::Integer(_) = right {
            return eval_integer_infix_expression(operator, left, right);
        }
    }
    match operator {
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Null,
    }
}

fn eval_integer_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    if let Object::Integer(left_value) = left {
        if let Object::Integer(right_value) = right {
            return match operator {
                "+" => Object::Integer(left_value + right_value),
                "-" => Object::Integer(left_value - right_value),
                "*" => Object::Integer(left_value * right_value),
                "/" => Object::Integer(left_value / right_value),
                "<" => Object::Boolean(left_value < right_value),
                ">" => Object::Boolean(left_value > right_value),
                "==" => Object::Boolean(left_value == right_value),
                "!=" => Object::Boolean(left_value != right_value),
                _ => Object::Null,
            };
        };
    };
    Object::Null
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

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(true) => true,
        Object::Boolean(false) => false,
        _ => true,
    }
}

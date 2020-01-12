use crate::ast::{BlockStatement, Expression, Program, Statement};
use crate::object::{Object, ObjectType};

pub fn eval(program: Program) -> Object {
    let mut result = Object::Null;
    for statement in program.statements {
        result = eval_statement(statement);
        match result {
            Object::ReturnValue(val) => {
                return *val;
            }
            Object::Error(_) => {
                return result;
            }
            _ => (),
        }
    }
    result
}

fn eval_statement(statement: Statement) -> Object {
    match statement {
        Statement::Expression(expr) => eval_expression(expr),
        Statement::Return(expr) => {
            let val = eval_expression(expr);
            if is_error(&val) {
                return val;
            }
            Object::ReturnValue(Box::new(val))
        }
        _ => Object::Null,
    }
}

fn eval_expression(expression: Expression) -> Object {
    match expression {
        Expression::IntegerLiteral(i) => Object::Integer(i),
        Expression::Boolean(b) => Object::Boolean(b),
        Expression::Prefix(operator, right) => {
            let r = eval_expression(*right);
            if is_error(&r) {
                return r;
            }
            eval_prefix_expression(&operator, r)
        }
        Expression::Infix(left, operator, right) => {
            let l = eval_expression(*left);
            if is_error(&l) {
                return l;
            }
            let r = eval_expression(*right);
            if is_error(&r) {
                return r;
            }
            eval_infix_expression(&operator, l, r)
        }
        Expression::IfExpression(condition, consequence, alt) => {
            let c = eval_expression(*condition);
            if is_error(&c) {
                return c;
            }
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
    let mut result = Object::Null;
    for statement in block_statement.statements {
        result = eval_statement(statement);
        if result.obj_type() == ObjectType::ReturnValue || result.obj_type() == ObjectType::Error {
            return result;
        }
    }
    result
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => {
            let msg = format!("unknown operator: {}{}", operator, right.obj_type());
            Object::Error(msg)
        }
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
        _ => {
            if left.obj_type() != right.obj_type() {
                let msg = format!(
                    "type mismatch: {} {} {}",
                    left.obj_type(),
                    operator,
                    right.obj_type()
                );
                return Object::Error(msg);
            } else {
                let msg = format!(
                    "unknown operator: {} {} {}",
                    left.obj_type(),
                    operator,
                    right.obj_type()
                );
                return Object::Error(msg);
            }
        }
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
                _ => {
                    let msg = format!(
                        "unknown operator: {} {} {}",
                        left.obj_type(),
                        operator,
                        right.obj_type()
                    );
                    Object::Error(msg)
                }
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
        _ => {
            dbg!(&right);
            let msg = format!("unknown operator: -{}", right.obj_type());
            Object::Error(msg)
        }
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

fn is_error(obj: &Object) -> bool {
    obj.obj_type() == ObjectType::Error
}

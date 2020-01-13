use crate::ast::{BlockStatement, Expression, Program, Statement};
use crate::object::{Environment, Object, ObjectType};
use std::cell::RefCell;
use std::rc::Rc;

pub fn eval(program: Program, env: Rc<RefCell<Environment>>) -> Object {
    let mut result = Object::Null;
    for statement in program.statements {
        result = eval_statement(statement, env.clone());
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

fn eval_statement(statement: Statement, env: Rc<RefCell<Environment>>) -> Object {
    match statement {
        Statement::Expression(expr) => eval_expression(expr, env),
        Statement::Return(expr) => {
            let val = eval_expression(expr, env);
            if is_error(&val) {
                return val;
            }
            Object::ReturnValue(Box::new(val))
        }
        Statement::Let(ident, expr) => {
            let val = eval_expression(expr.clone(), env.clone());
            if !is_error(&val) {
                env.borrow_mut().set(ident, val.clone());
            }
            val
        }
        _ => Object::Null,
    }
}

fn eval_expression(expression: Expression, env: Rc<RefCell<Environment>>) -> Object {
    match expression {
        Expression::IntegerLiteral(i) => Object::Integer(i),
        Expression::Boolean(b) => Object::Boolean(b),
        Expression::Prefix(operator, right) => {
            let r = eval_expression(*right, env);
            if is_error(&r) {
                return r;
            }
            eval_prefix_expression(&operator, r)
        }
        Expression::Infix(left, operator, right) => {
            let l = eval_expression(*left, env.clone());
            if is_error(&l) {
                return l;
            }
            let r = eval_expression(*right, env.clone());
            if is_error(&r) {
                return r;
            }
            eval_infix_expression(&operator, l, r)
        }
        Expression::IfExpression(condition, consequence, alt) => {
            let c = eval_expression(*condition, env.clone());
            if is_error(&c) {
                return c;
            }
            if is_truthy(c) {
                return eval_block_statement(consequence, env.clone());
            }
            match alt {
                Some(val) => eval_block_statement(val, env),
                None => Object::Null,
            }
        }
        Expression::Ident(ident) => eval_identifier(ident, env),
        Expression::FunctionLiteral(parms, body) => Object::Function(parms, body, env.clone()),
        Expression::CallExpression(function, arguments) => {
            let function = eval_expression(*function, env.clone());
            if is_error(&function) {
                function
            } else {
                let args = eval_expressions(arguments, env);
                if args.len() == 1 && is_error(&args[0]) {
                    return args[0].clone();
                }
                Object::Null
            }
        }
        _ => Object::Null,
    }
}

fn eval_block_statement(block_statement: BlockStatement, env: Rc<RefCell<Environment>>) -> Object {
    let mut result = Object::Null;
    for statement in block_statement.statements {
        result = eval_statement(statement, env.clone());
        if result.obj_type() == ObjectType::ReturnValue || result.obj_type() == ObjectType::Error {
            return result;
        }
    }
    result
}

fn eval_identifier(ident: String, env: Rc<RefCell<Environment>>) -> Object {
    match env.borrow().get(ident.clone()) {
        Some(val) => val.clone(),
        None => {
            let msg = format!("identifier not found: {}", ident);
            Object::Error(msg)
        }
    }
}

fn eval_expressions(expressions: Vec<Expression>, env: Rc<RefCell<Environment>>) -> Vec<Object> {
    let mut result = vec![];
    for exp in expressions {
        let eval = eval_expression(exp, env.clone());
        if is_error(&eval) {
            return vec![eval];
        }
        result.push(eval);
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

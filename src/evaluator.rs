use crate::ast::{BlockStatement, Expression, Program, Statement};
use crate::environment::Environment;
use crate::object;
use crate::object::{BuiltIn, Object, ObjectType};
use std::cell::RefCell;
use std::collections::HashMap;
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
                return function;
            }
            let args = eval_expressions(arguments, env);
            if args.len() == 1 && is_error(&args[0]) {
                return args[0].clone();
            }
            apply_function(function, args)
        }
        Expression::ArrayLiteral(elements) => {
            let elements = eval_expressions(elements, env);
            if elements.len() == 1 && is_error(&elements[0]) {
                return elements[0].clone();
            }
            Object::Array(elements)
        }
        Expression::IndexExpression(left, index) => {
            let left = eval_expression(*left, env.clone());
            if is_error(&left) {
                return left;
            }
            let index = eval_expression(*index, env);
            if is_error(&index) {
                return index;
            }
            eval_index_expression(left, index)
        }
        Expression::HashLiteral(pairs) => eval_hash_literal(pairs, env),
        Expression::StringLiteral(s) => Object::String(s),
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
        None => match BuiltIn::lookup_builtin(&ident) {
            Some(built_in) => Object::BuiltIn(built_in),
            None => {
                let msg = format!("identifier not found: {}", ident);
                Object::Error(msg)
            }
        },
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

fn eval_hash_literal(
    pairs: Vec<(Expression, Expression)>,
    env: Rc<RefCell<Environment>>,
) -> Object {
    let mut map = HashMap::new();

    let env = env.clone();

    for (k, v) in pairs {
        let key = eval_expression(k, env.clone());
        let key_clone = key.clone();
        if is_error(&key) {
            return key;
        }

        match object::create_hash_key(key) {
            Some(hash_key) => {
                let value = eval_expression(v, env.clone());
                if is_error(&value) {
                    return value;
                }
                let hash_pair = object::HashPair {
                    key: key_clone,
                    value,
                };
                map.insert(hash_key, hash_pair);
            }
            None => {
                let msg = format!("unusable as hash key: {}", key_clone.obj_type().to_string());
                return Object::Error(msg);
            }
        }
    }

    Object::Hash(map)
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

fn eval_index_expression(left: Object, index: Object) -> Object {
    if left.obj_type() == ObjectType::Array && index.obj_type() == ObjectType::Integer {
        return eval_array_index_expression(left, index);
    } else if left.obj_type() == ObjectType::Hash {
        return eval_hash_index_expression(left, index);
    }
    let msg = format!("index operator not supported: {}", left.obj_type());
    Object::Error(msg)
}

fn eval_array_index_expression(array: Object, index: Object) -> Object {
    match array {
        Object::Array(elements) => match index {
            Object::Integer(i) => {
                let idx = i as usize;
                let max = elements.len() - 1;
                if idx > max {
                    return Object::Null;
                }
                return elements[idx].clone();
            }
            _ => Object::Null,
        },
        _ => Object::Null,
    }
}

fn eval_hash_index_expression(hash: Object, index: Object) -> Object {
    match hash {
        Object::Hash(hash_object) => match object::create_hash_key(index) {
            Some(key) => match hash_object.get(&key) {
                Some(pair) => pair.value.clone(),
                None => Object::Null,
            },
            _ => Object::Error("unusable as hash key.".to_string()),
        },
        _ => Object::Error("expected hash.".to_string()),
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
            } else if left.obj_type() == ObjectType::String
                && right.obj_type() == ObjectType::String
            {
                return eval_string_infix_expression(operator.to_string(), left, right);
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

fn eval_string_infix_expression(operator: String, left: Object, right: Object) -> Object {
    if operator != "+" {
        let msg = format!(
            "unknown operator: {} {} {}",
            left.obj_type(),
            operator,
            right.obj_type()
        );
        return Object::Error(msg);
    }

    let left_val = match left {
        Object::String(s) => s,
        _ => String::from(""),
    };

    let right_val = match right {
        Object::String(s) => s,
        _ => String::from(""),
    };

    Object::String(format!("{}{}", left_val, right_val))
}

fn apply_function(function: Object, args: Vec<Object>) -> Object {
    match &function {
        Object::Function(_parms, body, _env) => {
            let extended_env = extend_function_env(&function, args);
            match extended_env {
                Some(extended_env) => {
                    let evaluated = eval_block_statement(body.clone(), extended_env);
                    unwrap_return_value(evaluated)
                }
                _ => Object::Error("extended env error".to_string()),
            }
        }
        Object::BuiltIn(built_in) => built_in.get_fn(args),
        _ => Object::Error("not a function".to_string()),
    }
}

fn extend_function_env(function: &Object, args: Vec<Object>) -> Option<Rc<RefCell<Environment>>> {
    match function {
        Object::Function(parms, _body, env) => {
            let mut env = Environment::new_enclosed_environment(env.clone());
            let items: Vec<_> = parms.iter().zip(args.iter()).collect();
            for (parm, arg) in items {
                env.set(parm.to_string(), arg.clone());
            }
            Some(Rc::new(RefCell::new(env)))
        }
        _ => None,
    }
}

fn unwrap_return_value(obj: Object) -> Object {
    match obj {
        Object::ReturnValue(val) => *val,
        _ => obj,
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
        _ => {
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

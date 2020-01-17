use crate::ast::{BlockStatement, Identifier};
use crate::environment::Environment;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Error(String),
    Function(Vec<Identifier>, BlockStatement, Rc<RefCell<Environment>>),
    String(String),
    BuiltIn,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Null,
    Error,
    Integer,
    Boolean,
    ReturnValue,
    Function,
    String,
    BuiltIn,
}

impl Object {
    pub fn inspect(&self) -> String {
        match &self {
            Object::Integer(i) => i.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Null => String::from(""),
            Object::ReturnValue(value) => String::from(&*value.inspect()),
            Object::Error(msg) => format!("ERROR: {}", msg),
            Object::Function(parms, body, _) => {
                format!("fn({}) {{\n{}\n}}", parms.join(", "), body.to_string())
            }
            Object::String(s) => s.to_string(),
            Object::BuiltIn => "builtin function".to_string(),
        }
    }

    pub fn obj_type<'a>(&self) -> ObjectType {
        match &self {
            Object::Integer(_) => ObjectType::Integer,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Null => ObjectType::Null,
            Object::ReturnValue(_) => ObjectType::ReturnValue,
            Object::Error(_) => ObjectType::Error,
            Object::Function(_, _, _) => ObjectType::Function,
            Object::String(_) => ObjectType::String,
            Object::BuiltIn => ObjectType::BuiltIn,
        }
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self {
            ObjectType::Null => "NULL",
            ObjectType::Error => "ERROR",
            ObjectType::Integer => "INTEGER",
            ObjectType::Boolean => "BOOLEAN",
            ObjectType::ReturnValue => "RETURN_VALUE",
            ObjectType::Function => "FUNCTION",
            ObjectType::String => "STRING",
            ObjectType::BuiltIn => "BUILTIN",
        };
        write!(f, "{}", output)
    }
}

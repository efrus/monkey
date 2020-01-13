use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Error(String),
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
}

impl Object {
    pub fn inspect(&self) -> String {
        match &self {
            Object::Integer(i) => i.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Null => String::from(""),
            Object::ReturnValue(value) => String::from(&*value.inspect()),
            Object::Error(msg) => format!("ERROR: {}", msg),
        }
    }

    pub fn obj_type<'a>(&self) -> ObjectType {
        match &self {
            Object::Integer(_) => ObjectType::Integer,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Null => ObjectType::Null,
            Object::ReturnValue(_) => ObjectType::ReturnValue,
            Object::Error(_) => ObjectType::Error,
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
        };
        write!(f, "{}", output)
    }
}

pub struct Environment {
    pub store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        let store = HashMap::new();
        Environment { store }
    }

    pub fn get(&self, name: String) -> Option<&Object> {
        self.store.get(&name)
    }

    pub fn set(&mut self, name: String, obj: Object) -> Option<Object> {
        self.store.insert(name, obj)
    }
}

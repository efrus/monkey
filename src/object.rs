use crate::ast::{BlockStatement, Identifier};
use crate::environment::Environment;
use std::cell::RefCell;
use std::collections::HashMap;
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
    BuiltIn(BuiltIn),
    Array(Vec<Object>),
    Hash(HashMap<HashKey, HashPair>),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectType {
    Null,
    Error,
    Integer,
    Boolean,
    ReturnValue,
    Function,
    String,
    BuiltIn,
    Array,
    Hash,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltIn {
    Len,
    First,
    Last,
    Rest,
    Push,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HashKey {
    obj_type: ObjectType,
    value: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashPair {
    pub key: Object,
    pub value: Object,
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
            Object::BuiltIn(bi) => bi.to_string(),
            Object::Array(elements) => {
                let mut s = vec![];
                for element in elements {
                    s.push(element.inspect());
                }
                format!("[{}]", s.join(", "))
            }
            Object::Hash(map) => {
                let mut pairs = vec![];
                for pair in map.values() {
                    pairs.push(format!("{}: {}", pair.key.inspect(), pair.value.inspect()));
                }
                format!("{{{}}}", pairs.join(", "))
            }
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
            Object::BuiltIn(_) => ObjectType::BuiltIn,
            Object::Array(_) => ObjectType::Array,
            Object::Hash(_) => ObjectType::Hash,
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
            ObjectType::Array => "ARRAY",
            ObjectType::Hash => "HASH",
        };
        write!(f, "{}", output)
    }
}

impl fmt::Display for BuiltIn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self {
            BuiltIn::Len => "Len",
            BuiltIn::First => "First",
            BuiltIn::Last => "Last",
            BuiltIn::Rest => "Rest",
            BuiltIn::Push => "Push",
        };
        write!(f, "{}", output)
    }
}

impl BuiltIn {
    pub fn get_fn(&self, args: Vec<Object>) -> Object {
        match self {
            BuiltIn::Len => builtin_len(args),
            BuiltIn::First => builtin_first(args),
            BuiltIn::Last => builtin_last(args),
            BuiltIn::Rest => builtin_rest(args),
            BuiltIn::Push => builtin_push(args),
        }
    }

    pub fn lookup_builtin(s: &str) -> Option<BuiltIn> {
        match s {
            "len" => Some(BuiltIn::Len),
            "first" => Some(BuiltIn::First),
            "last" => Some(BuiltIn::Last),
            "rest" => Some(BuiltIn::Rest),
            "push" => Some(BuiltIn::Push),
            _ => None,
        }
    }
}

fn builtin_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        let msg = format!("wrong number of arguments. got={}, want=1", args.len());
        return Object::Error(msg);
    }
    match &args[0] {
        Object::String(s) => {
            let l = s.len() as i64;
            Object::Integer(l)
        }
        Object::Array(elements) => {
            let l = elements.len() as i64;
            Object::Integer(l)
        }
        _ => {
            let msg = "argument to 'len' not supported.".to_string();
            Object::Error(msg)
        }
    }
}

fn builtin_first(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        let msg = format!("wrong number of arguments. got={}, want=1", args.len());
        return Object::Error(msg);
    }
    match &args[0] {
        Object::Array(elements) => {
            if elements.len() == 0 {
                return Object::Null;
            }
            elements[0].clone()
        }
        _ => {
            let msg = "argument to 'first' must be ARRAY".to_string();
            Object::Error(msg)
        }
    }
}

fn builtin_last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        let msg = format!("wrong number of arguments. got={}, want=1", args.len());
        return Object::Error(msg);
    }
    match &args[0] {
        Object::Array(elements) => {
            if elements.len() == 0 {
                return Object::Null;
            }

            elements[elements.len() - 1].clone()
        }
        _ => {
            let msg = "argument to 'last' must be ARRAY".to_string();
            Object::Error(msg)
        }
    }
}

fn builtin_rest(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        let msg = format!("wrong number of arguments. got={}, want=1", args.len());
        return Object::Error(msg);
    }
    match &args[0] {
        Object::Array(elements) => {
            if elements.len() == 0 {
                return Object::Null;
            }

            let (_head, tail) = elements.split_at(1);
            Object::Array(tail.to_vec())
        }
        _ => {
            let msg = "argument to 'rest' must be ARRAY".to_string();
            Object::Error(msg)
        }
    }
}

fn builtin_push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        let msg = format!("wrong number of arguments. got={}, want=2", args.len());
        return Object::Error(msg);
    }
    match &args[0] {
        Object::Array(elements) => {
            if elements.len() == 0 {
                return Object::Null;
            }

            let mut push = vec![args[1].clone()];
            let mut new_elements = elements.clone();
            new_elements.append(&mut push);
            Object::Array(new_elements)
        }
        _ => {
            let msg = "argument to 'rest' must be ARRAY".to_string();
            Object::Error(msg)
        }
    }
}

pub fn create_hash_key(obj: Object) -> Option<HashKey> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let obj_type = obj.obj_type();
    match obj {
        Object::Boolean(b) => {
            let value = if b { 1 } else { 0 };
            Some(HashKey { obj_type, value })
        }
        Object::Integer(i) => Some(HashKey {
            obj_type,
            value: i as u64,
        }),
        Object::String(str) => {
            let mut s = DefaultHasher::new();
            str.hash(&mut s);
            let value = s.finish();
            Some(HashKey { obj_type, value })
        }
        _ => None,
    }
}

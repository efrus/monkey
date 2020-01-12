#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
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
            Object::Null => String::from("null"),
            Object::ReturnValue(value) => String::from(&*value.inspect()),
        }
    }

    pub fn obj_type<'a>(&self) -> ObjectType {
        match &self {
            Object::Integer(_) => ObjectType::Integer,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Null => ObjectType::Null,
            Object::ReturnValue(_) => ObjectType::ReturnValue,
        }
    }
}

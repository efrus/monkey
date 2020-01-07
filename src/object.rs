#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

static INTEGER_OBJ: &str = "INTEGER";
static BOOLEAN_OBJ: &str = "BOOLEAN";
static NULL_OBJ: &str = "NULL";

impl Object {
    pub fn inspect(&self) -> String {
        match &self {
            Object::Integer(i) => i.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Null => "null".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn obj_type<'a>(&self) -> &'a str {
        match &self {
            Object::Integer(_) => INTEGER_OBJ,
            Object::Boolean(_) => BOOLEAN_OBJ,
            Object::Null => NULL_OBJ,
            _ => "",
        }
    }
}

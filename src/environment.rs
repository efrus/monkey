use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Environment {
        let store = HashMap::new();
        Environment { store, outer: None }
    }
}

impl Environment {
    pub fn new_enclosed_environment(outer: Rc<RefCell<Environment>>) -> Environment {
        let mut env = Environment::default();
        env.outer = Some(outer);
        env
    }

    pub fn get(&self, name: String) -> Option<Object> {
        match self.store.get(&name) {
            Some(item) => Some(item.clone()),
            None => self.outer.as_ref().and_then(|item| item.borrow().get(name)),
        }
    }

    pub fn set(&mut self, name: String, obj: Object) -> Option<Object> {
        self.store.insert(name, obj)
    }
}

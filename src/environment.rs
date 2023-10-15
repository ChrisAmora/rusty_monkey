use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

#[derive(Debug)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub outer: Option<GlobalEnv>,
}

pub type GlobalEnv = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: GlobalEnv, store: HashMap<String, Object>) -> GlobalEnv {
        Rc::new(RefCell::new(Environment {
            store,
            outer: Some(outer),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(obj) = self.store.get(name) {
            return Some(obj.clone());
        } else if let Some(outer) = self.outer.clone() {
            let outer = outer;
            return outer.borrow().get(name);
        }
        None
    }

    pub fn set(&mut self, name: String, val: &Object) {
        self.store.insert(name, val.clone());
    }
}

use std::{fmt::Display, io::Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Nil,
    Int(i64),
    Bool(bool),
    Return(Box<Object>),
}

impl Object {
    pub fn bang(&self) -> Object {
        match self {
            Object::Nil => Object::Bool(true),
            Object::Bool(value) => {
                if value == &true {
                    Object::Bool(false)
                } else {
                    Object::Bool(true)
                }
            }
            Object::Int(_) => Object::Bool(false),
            Object::Return(_) => Object::Bool(false),
        }
    }

    pub fn minus(&self) -> Object {
        match self {
            Object::Int(value) => Object::Int(-value),
            _ => Object::Nil,
        }
    }

    pub fn add(&self, right: Object) -> Object {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Object::Int(left + right),
            _ => todo!(),
        }
    }

    pub fn sub(&self, right: Object) -> Object {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Object::Int(left - right),
            _ => todo!(),
        }
    }

    pub fn mul(&self, right: Object) -> Object {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Object::Int(left * right),
            _ => todo!(),
        }
    }

    pub fn div(&self, right: Object) -> Object {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Object::Int(left / right),
            _ => todo!(),
        }
    }

    pub fn eq(&self, right: Object) -> Object {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Object::Bool(left == &right),
            (Object::Bool(left), Object::Bool(right)) => Object::Bool(left == &right),
            _ => todo!(),
        }
    }
    pub fn not_eq(&self, right: Object) -> Object {
        self.eq(right).bang()
    }
    pub fn gt(&self, right: Object) -> Object {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Object::Bool(left > &right),
            _ => todo!(),
        }
    }

    pub fn lt(&self, right: Object) -> Object {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Object::Bool(left < &right),
            _ => todo!(),
        }
    }

    pub fn lte(&self, right: Object) -> Object {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Object::Bool(left <= &right),
            _ => todo!(),
        }
    }

    pub fn gte(&self, right: Object) -> Object {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Object::Bool(left >= &right),
            _ => todo!(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Nil => write!(f, "nil"),
            Object::Int(value) => write!(f, "{value}"),
            Object::Bool(value) => {
                if value == &true {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Object::Return(ret) => write!(f, "return {ret}"),
        }
    }
}

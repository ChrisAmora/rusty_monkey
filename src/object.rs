use anyhow::{anyhow, Result};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Nil,
    Int(i64),
    Bool(bool),
    Return(Box<Object>),
}

pub struct Environment {
    pub map: HashMap<String, Object>,
}

pub struct Stack {
    pub object: Object,
    pub env: Environment,
}

impl Stack {
    pub fn new(object: Object, env: Environment) -> Self {
        Self { object, env }
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl Object {
    pub fn bang(&self) -> Result<Object> {
        match self {
            Object::Nil => Ok(Object::Bool(true)),
            Object::Bool(value) => {
                if value == &true {
                    Ok(Object::Bool(false))
                } else {
                    Ok(Object::Bool(true))
                }
            }
            Object::Int(_) => Ok(Object::Bool(false)),
            Object::Return(_) => Ok(Object::Bool(false)),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Object::Nil => "nil",
            Object::Int(_) => "int",
            Object::Bool(_) => "bool",
            Object::Return(_) => "return",
        }
    }

    pub fn minus(&self) -> Result<Object> {
        match self {
            Object::Int(value) => Ok(Object::Int(-value)),
            object => Err(anyhow!("unknown operator -{}", object)),
        }
    }

    pub fn add(&self, right: Object) -> Result<Object> {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Ok(Object::Int(left + right)),
            (x, y) => Err(anyhow!("type mismatch: {x} + {y}")),
        }
    }

    pub fn sub(&self, right: Object) -> Result<Object> {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Ok(Object::Int(left - right)),
            _ => todo!(),
        }
    }

    pub fn mul(&self, right: Object) -> Result<Object> {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Ok(Object::Int(left * right)),
            _ => todo!(),
        }
    }

    pub fn div(&self, right: Object) -> Result<Object> {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Ok(Object::Int(left / right)),
            _ => todo!(),
        }
    }

    pub fn eq(&self, right: Object) -> Result<Object> {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Ok(Object::Bool(left == &right)),
            (Object::Bool(left), Object::Bool(right)) => Ok(Object::Bool(left == &right)),
            _ => todo!(),
        }
    }
    pub fn not_eq(&self, right: Object) -> Result<Object> {
        self.eq(right)?.bang()
    }
    pub fn gt(&self, right: Object) -> Result<Object> {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Ok(Object::Bool(left > &right)),
            _ => todo!(),
        }
    }

    pub fn lt(&self, right: Object) -> Result<Object> {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Ok(Object::Bool(left < &right)),
            _ => todo!(),
        }
    }

    pub fn lte(&self, right: Object) -> Result<Object> {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Ok(Object::Bool(left <= &right)),
            _ => todo!(),
        }
    }

    pub fn gte(&self, right: Object) -> Result<Object> {
        match (self, right) {
            (Object::Int(left), Object::Int(right)) => Ok(Object::Bool(left >= &right)),
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

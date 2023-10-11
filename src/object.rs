use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Nil,
    Int(i64),
    Bool(bool),
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
        }
    }
}

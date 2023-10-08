use crate::token::Identifier;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement {
    Let {
        identifier: Identifier,
        expression: Expression,
    },
    Return,
    Expression(Expression),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
    Plus,
    Identifier(Identifier),
    Literal(Literal),
    Prefix(Prefix),
    Infix(Infix),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal {
    Int(i64),
    String(String),
    True,
    False,
    Nil,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PrefixOperation {
    Bang,
    Minus,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InfixOperation {
    Add,
    Sub,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Prefix {
    pub expression: Box<Expression>,
    pub operation: PrefixOperation,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Infix {
    pub left_expression: Box<Expression>,
    pub right_expression: Box<Expression>,
    pub operation: InfixOperation,
}

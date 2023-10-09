use core::fmt;
use std::mem::uninitialized;

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
    Eq,
    NotEq,
    Lt,
    Lte,
    Gt,
    Gte,
    Mul,
    Div,
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

impl Expression {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Nil => f.write_str("nil"),
            Literal::True => f.write_str("true"),
            Literal::False => f.write_str("false"),
            Literal::String(str) => f.write_str(str.as_str()),
            Literal::Int(int) => {
                let fmt_str = int.to_string();
                f.write_str(fmt_str.as_str())
            }
        }
    }
}

impl fmt::Display for PrefixOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrefixOperation::Bang => f.write_str("!"),
            PrefixOperation::Minus => f.write_str("-"),
        }
    }
}

impl fmt::Display for InfixOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfixOperation::Eq => f.write_str("="),
            InfixOperation::Lt => f.write_str("<"),
            InfixOperation::Gt => f.write_str(">"),
            InfixOperation::Gte => f.write_str(">="),
            InfixOperation::Add => f.write_str("+"),
            InfixOperation::Sub => f.write_str("-"),
            InfixOperation::Lte => f.write_str("<="),
            InfixOperation::NotEq => f.write_str("!="),
            InfixOperation::Mul => f.write_str("*"),
            InfixOperation::Div => f.write_str("/"),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(literal) => f.write_str(literal.to_string().as_str()),
            Expression::Prefix(prefix) => f.write_str(prefix.to_string().as_str()),
            Expression::Infix(infix) => f.write_str(infix.to_string().as_str()),

            Expression::Identifier(identifier) => {
                let formatted = identifier.to_string();
                f.write_str(formatted.as_str())
            }
        }
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_expression = self.expression.to_string();
        let str_operator = self.operation.to_string();
        write!(f, "({}{})", str_operator, str_expression)
    }
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left = self.left_expression.to_string();
        let right = self.right_expression.to_string();
        let operator = self.operation.to_string();
        write!(f, "({}{}{})", left, operator, right)
    }
}

use core::fmt;

use anyhow::{anyhow, Result};

use crate::ast::InfixOperation;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Identifier(Identifier),
    Int(i64),
    True,
    False,
    Nil,
    Dot,
    Eof,
    Eq,
    NotEq,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Lte,
    Gt,
    Gte,
    Comma,
    LineBreak,
    Colon,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Function,
    Let,
    If,
    Else,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn new_str(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl TryFrom<TokenType> for Identifier {
    type Error = anyhow::Error;

    fn try_from(token: TokenType) -> Result<Self, Self::Error> {
        match token {
            TokenType::Identifier(name) => Ok(name),
            _ => Err(anyhow!("error")),
        }
    }
}

impl TokenType {
    pub fn precedence(&self) -> usize {
        match self {
            TokenType::Eq | TokenType::NotEq => 2,
            TokenType::Gt | TokenType::Gte | TokenType::Lt | TokenType::Lte => 3,
            TokenType::Plus | TokenType::Minus => 4,
            TokenType::Slash | TokenType::Asterisk => 5,
            TokenType::LParen => 7,
            TokenType::LBracket => 8,
            _ => 0,
        }
    }
    pub fn operation(&self) -> Option<InfixOperation> {
        match self {
            TokenType::Plus => Some(InfixOperation::Add),
            TokenType::Minus => Some(InfixOperation::Sub),
            TokenType::Asterisk => Some(InfixOperation::Mul),
            TokenType::Slash => Some(InfixOperation::Div),
            TokenType::Eq => Some(InfixOperation::Eq),
            TokenType::NotEq => Some(InfixOperation::NotEq),
            TokenType::Lt => Some(InfixOperation::Lt),
            TokenType::Lte => Some(InfixOperation::Lte),
            TokenType::Gt => Some(InfixOperation::Gt),
            TokenType::Gte => Some(InfixOperation::Gte),
            _ => None,
        }
    }
}

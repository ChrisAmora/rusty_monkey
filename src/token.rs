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

impl Identifier {
    #[inline]
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MyError;

impl TryFrom<TokenType> for Identifier {
    type Error = anyhow::Error;

    #[inline]
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
            _ => None,
        }
    }
}

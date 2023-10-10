use core::fmt;
use std::fmt::Display;

use crate::token::Identifier;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement {
    Let {
        identifier: Identifier,
        expression: Expression,
    },
    Return(Expression),
    Expression(Expression),
    Block(Block),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix(Prefix),
    Infix(Infix),
    If(If),
    Function(Function),
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
pub struct Function {
    pub params: Vec<Identifier>,
    pub body: Block,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Block(pub Vec<Statement>);

impl Block {
    pub fn new(value: Vec<Statement>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Infix {
    pub left_expression: Box<Expression>,
    pub right_expression: Box<Expression>,
    pub operation: InfixOperation,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct If {
    pub condition: Box<Expression>,
    pub alternative: Option<Block>,
    pub consequence: Block,
}

impl Expression {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Display for Literal {
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

impl Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}", TokenType::Function);
        write!(f, "fn (")?;

        for (index, param) in self.params.iter().enumerate() {
            write!(f, "{}", param)?;
            if index != self.params.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")?;

        if self.body.len() > 0 {
            write!(f, " ")?;
        }

        write!(f, "{}", self.body)
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let {
                identifier,
                expression,
            } => {
                write!(
                    f,
                    "let {} = {}",
                    identifier.to_string().as_str(),
                    expression.to_string().as_str()
                )
            }
            Statement::Return(expression) => write!(f, "return {expression}"),
            Statement::Expression(expression) => write!(f, "return {expression}"),
            Statement::Block(block) => write!(f, "{block}"),
        }
    }
}

impl Display for PrefixOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrefixOperation::Bang => f.write_str("!"),
            PrefixOperation::Minus => f.write_str("-"),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.0 {
            write!(f, "{statement}")?
        }
        write!(f, "")
    }
}

impl Block {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for InfixOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfixOperation::Eq => f.write_str("=="),
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

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(literal) => write!(f, "{literal}"),
            Expression::Prefix(prefix) => write!(f, "{prefix}"),
            Expression::Infix(infix) => write!(f, "{infix}"),
            Expression::If(if_expression) => write!(f, "{if_expression}"),
            Expression::Identifier(identifier) => write!(f, "{identifier}"),
            Expression::Function(function) => write!(f, "{function}"),
        }
    }
}

impl Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_expression = self.expression.to_string();
        let str_operator = self.operation.to_string();
        write!(f, "({}{})", str_operator, str_expression)
    }
}

impl Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left = self.left_expression.to_string();
        let right = self.right_expression.to_string();
        let operator = self.operation.to_string();
        write!(f, "({}{}{})", left, operator, right)
    }
}

impl Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let condition = self.condition.to_string();
        let consequence = self.consequence.to_string();
        write!(f, "if {} {}", condition, consequence)?;
        match &self.alternative {
            Some(alt) => {
                write!(f, " else {}", alt)?;
            }
            None => {}
        }
        write!(f, "")
    }
}

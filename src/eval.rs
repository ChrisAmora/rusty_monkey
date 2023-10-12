use std::fmt::Display;

use crate::{
    ast::{Block, Expression, If, InfixOperation, Literal, PrefixOperation, Statement},
    object::Object,
    parser::Parser,
};

use anyhow::Result;

pub struct Program {}

impl Program {
    pub fn eval(&mut self, parser: &mut Parser) -> Result<Object> {
        let mut result = Object::Nil;
        for statement in parser.collect_statements() {
            result = statement.eval()?;
            if let Object::Return(expression) = result {
                return Ok(*expression);
            }
        }
        Ok(result)
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Statement {
    pub fn eval(&self) -> Result<Object> {
        match self {
            Statement::Return(expression) => Ok(Object::Return(Box::new(expression.eval()?))),
            Statement::Expression(expression) => expression.eval(),
            Statement::Block(block) => block.eval(),
            Statement::Let {
                identifier,
                expression,
            } => todo!(),
        }
    }
}

impl Block {
    pub fn eval(&self) -> Result<Object> {
        let mut result = Object::Nil;
        for statement in &self.0 {
            result = statement.eval()?;
            if let Object::Return(_) = result {
                break;
            }
        }
        Ok(result)
    }
}

impl If {
    pub fn eval(&self) -> Result<Object> {
        match self.condition.eval()? {
            Object::Nil => self
                .alternative
                .as_ref()
                .map_or(Ok(Object::Nil), |block| block.eval()),
            Object::Int(_) => self.consequence.eval(),
            Object::Bool(b) => {
                if b {
                    return self.consequence.eval();
                }
                self.alternative
                    .as_ref()
                    .map_or(Ok(Object::Nil), |block| block.eval())
            }
            Object::Return(_) => todo!(),
        }
    }
}

impl Expression {
    pub fn eval(&self) -> Result<Object> {
        match self {
            Expression::Literal(literal) => literal.eval(),
            Expression::Prefix(prefix) => {
                let right = prefix.expression.eval()?;
                match prefix.operation {
                    PrefixOperation::Bang => right.bang(),
                    PrefixOperation::Minus => right.minus(),
                }
            }
            Expression::If(if_expression) => if_expression.eval(),
            Expression::Infix(infix) => {
                let left = infix.left_expression.eval()?;
                let right = infix.right_expression.eval()?;

                match infix.operation {
                    InfixOperation::Add => left.add(right),
                    InfixOperation::Sub => left.sub(right),
                    InfixOperation::Mul => left.mul(right),
                    InfixOperation::Div => left.div(right),
                    InfixOperation::Eq => left.eq(right),
                    InfixOperation::NotEq => left.not_eq(right),
                    InfixOperation::Gt => left.gt(right),
                    InfixOperation::Gte => left.gte(right),
                    InfixOperation::Lt => left.lt(right),
                    InfixOperation::Lte => left.lte(right),
                    _ => Ok(Object::Nil),
                }
            }

            _ => todo!(),
        }
    }
}

impl PrefixOperation {
    pub fn eval(&self) -> Object {
        match self {
            PrefixOperation::Bang => todo!(),
            PrefixOperation::Minus => todo!(),
        }
    }
}

impl Literal {
    pub fn eval(&self) -> Result<Object> {
        match self {
            Literal::Int(int) => Ok(Object::Int(*int)),
            Literal::True => Ok(Object::Bool(true)),
            Literal::False => Ok(Object::Bool(false)),
            Literal::Nil => Ok(Object::Nil),
            Literal::String(_) => Ok(Object::Bool(false)),
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use crate::{lexer, object::Object, parser::Parser};

    use super::Program;

    fn generate_eval(text: &str) -> Object {
        let mut lexer = lexer::Lexer::new_from_str(text);
        let mut parser = Parser::new(lexer.peekable_iter());
        let mut program = Program::new();
        let eval = program.eval(&mut parser);
        match eval {
            Ok(expr) => {
                println!("{}", expr);
                expr
            }
            Err(err) => {
                println!("error: {}", err);
                Object::Nil
            }
        }
    }

    fn generate_eval_err(text: &str, expected: &str) -> Object {
        let mut lexer = lexer::Lexer::new_from_str(text);
        let mut parser = Parser::new(lexer.peekable_iter());
        let mut program = Program::new();
        let eval = program.eval(&mut parser);
        match eval {
            Ok(_) => {
                panic!("should not happen");
            }
            Err(err) => {
                assert_eq!(err.to_string(), expected);
                println!("error: {}", err);
                Object::Nil
            }
        }
    }

    #[test]
    fn ev() {
        assert_eq!(generate_eval("5"), Object::Int(5));
        assert_eq!(generate_eval("true"), Object::Bool(true));
        assert_eq!(generate_eval("false"), Object::Bool(false));
        assert_eq!(generate_eval("!false"), Object::Bool(true));
        assert_eq!(generate_eval("!true"), Object::Bool(false));
        assert_eq!(generate_eval("!5"), Object::Bool(false));
        assert_eq!(generate_eval("!!true"), Object::Bool(true));
        assert_eq!(generate_eval("!!false"), Object::Bool(false));
        assert_eq!(generate_eval("!!5"), Object::Bool(true));
        assert_eq!(generate_eval("-5"), Object::Int(-5));
        assert_eq!(generate_eval("-10"), Object::Int(-10));
        assert_eq!(generate_eval("-false"), Object::Nil);
        assert_eq!(generate_eval("5 + 10"), Object::Int(15));
        assert_eq!(generate_eval("2 * 2 * 2 * 2 * 2"), Object::Int(32));
        assert_eq!(generate_eval("50 / 2 * 2 + 10"), Object::Int(60));
        assert_eq!(generate_eval("2 * (5 + 10)"), Object::Int(30));
        assert_eq!(generate_eval("3 * 3 * 3 + 10"), Object::Int(37));
        assert_eq!(generate_eval("3 * (3 * 3) + 10"), Object::Int(37));
        assert_eq!(
            generate_eval("(5 + 10 * 2 + 15 / 3) * 2 + -10"),
            Object::Int(50)
        );
        assert_eq!(generate_eval("1 < 2"), Object::Bool(true));
        assert_eq!(generate_eval("1 > 2"), Object::Bool(false));
        assert_eq!(generate_eval("1 < 1"), Object::Bool(false));
        assert_eq!(generate_eval("1 > 1"), Object::Bool(false));
        assert_eq!(generate_eval("1 == 1"), Object::Bool(true));
        assert_eq!(generate_eval("1 != 1"), Object::Bool(false));
        assert_eq!(generate_eval("1 == 2"), Object::Bool(false));
        assert_eq!(generate_eval("1 != 2"), Object::Bool(true));
        assert_eq!(generate_eval("(1 < 2) == true"), Object::Bool(true));
        assert_eq!(generate_eval("(1 > 2) == true"), Object::Bool(false));
        assert_eq!(generate_eval("if (true) { 10 }"), Object::Int(10));
        assert_eq!(generate_eval("if (false) { 10 }"), Object::Nil);
        assert_eq!(generate_eval("if (1) { 10 }"), Object::Int(10));
        assert_eq!(generate_eval("if (1 > 2) { 10 }"), Object::Nil);
        assert_eq!(
            generate_eval("if (1 > 2) { 10 } else { 20 }"),
            Object::Int(20)
        );
        assert_eq!(
            generate_eval("if (1 < 2) { 10 } else { 20 }"),
            Object::Int(10)
        );
        assert_eq!(generate_eval("return 10;"), Object::Int(10));
        assert_eq!(generate_eval("return 10; 9;"), Object::Int(10));
        assert_eq!(generate_eval("return 2 * 5; 9;"), Object::Int(10));
        assert_eq!(generate_eval("9; return 2 * 5; 9;"), Object::Int(10));
        assert_eq!(
            generate_eval("if (10 > 1) {if (10 > 1) {return 10;}return 1;}"),
            Object::Int(10)
        );
        generate_eval_err("5 + true;", "type mismatch: 5 + true");
        generate_eval_err("5 + true; 5;", "type mismatch: 5 + true");
        generate_eval_err("-true", "unknown operator -true");
        generate_eval_err("true + false;", "type mismatch: true + false");
        generate_eval_err("5; true + false; 5", "type mismatch: true + false");
        generate_eval_err(
            "if (10 > 1) { true + false; }",
            "type mismatch: true + false",
        );
    }
}

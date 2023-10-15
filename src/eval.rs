use crate::{
    ast::{
        Block, Call, Expression, Function, If, InfixOperation, Literal, PrefixOperation, Statement,
    },
    environment::{Environment, GlobalEnv},
    object::Object,
    parser::Parser,
};

use anyhow::{bail, Ok, Result};

#[derive(Default)]
pub struct Program {}

impl Program {
    pub fn eval(&mut self, parser: &mut Parser, env: GlobalEnv) -> Result<Object> {
        let mut result = Object::Nil;

        for statement in parser {
            result = statement.eval(env.clone())?;

            if let Object::Return(expression) = result {
                return Ok(*expression);
            }
        }
        Ok(result)
    }
}

impl Statement {
    pub fn eval(self, env: GlobalEnv) -> Result<Object> {
        match self {
            Statement::Return(expression) => {
                let result = expression.eval(env)?;
                Ok(Object::Return(Box::new(result)))
            }
            Statement::Expression(expression) => expression.eval(env),
            Statement::Block(block) => block.eval(env),

            Statement::Let {
                identifier,
                expression,
            } => {
                let stack = expression.eval(env.clone())?;

                env.borrow_mut().set(identifier.get_name(), &stack.clone());
                Ok(Object::Nil)
            }
        }
    }
}

impl Block {
    pub fn eval(self, env: GlobalEnv) -> Result<Object> {
        let mut result = Object::Nil;
        for statement in self.0 {
            result = statement.eval(env.clone())?;
            if let Object::Return(_) = result {
                break;
            }
        }
        Ok(result)
    }
}

impl If {
    pub fn eval(self, env: GlobalEnv) -> Result<Object> {
        let result = self.condition.eval(env.clone())?;
        match result {
            Object::Nil => {
                let mut result = Object::Nil;
                if let Some(block) = self.alternative {
                    result = block.eval(env.clone())?;
                }
                Ok(result)
            }
            Object::Int(_) => self.consequence.eval(env),
            Object::Bool(b) => {
                if b {
                    let result = self.consequence.eval(env)?;
                    return Ok(result);
                }
                let mut result = Object::Nil;
                if let Some(block) = self.alternative {
                    result = block.eval(env.clone())?;
                }
                Ok(result)
            }
            _ => todo!(),
        }
    }
}

impl Call {
    pub fn eval(self, env: GlobalEnv) -> Result<Object> {
        let function = self.function.eval(env.clone())?;

        match function {
            Object::Function(f) => {
                let resolved_args_map = f
                    .parameters
                    .into_iter()
                    .map(|id| id.get_name())
                    .zip(
                        self.arguments
                            .into_iter()
                            .flat_map(|exp| exp.eval(env.clone())),
                    )
                    .collect();
                let env = Environment::new_enclosed(env, resolved_args_map);
                f.body.eval(env)
            }
            _ => todo!(),
        }
    }
}

impl Function {
    pub fn eval(self, env: GlobalEnv) -> Result<Object> {
        Ok(Object::Function(crate::object::Function {
            parameters: self.params,
            body: self.body,
            env,
        }))
    }
}

impl Expression {
    pub fn eval(self, env: GlobalEnv) -> Result<Object> {
        match self {
            Expression::Literal(literal) => Ok(literal.eval()?),
            Expression::Prefix(prefix) => {
                let right = prefix.expression.eval(env)?;
                match prefix.operation {
                    PrefixOperation::Bang => Ok(right.bang()?),
                    PrefixOperation::Minus => Ok(right.minus()?),
                }
            }
            Expression::If(if_expression) => if_expression.eval(env),
            Expression::Identifier(id) => {
                let result = env.borrow().get(&id.get_name());
                match result {
                    Some(value) => Ok(value.clone()),
                    None => {
                        bail!("identifier not found: {}", &id.get_name())
                    }
                }
            }

            Expression::Call(call) => call.eval(env),
            Expression::Infix(infix) => {
                let left = infix.left_expression.eval(env.clone())?;
                let right = infix.right_expression.eval(env)?;

                match infix.operation {
                    InfixOperation::Add => Ok(left.add(right)?),
                    InfixOperation::Sub => Ok(left.sub(right)?),
                    InfixOperation::Mul => Ok(left.mul(right)?),
                    InfixOperation::Div => Ok(left.div(right)?),
                    InfixOperation::Eq => Ok(left.eq(right)?),
                    InfixOperation::NotEq => Ok(left.not_eq(right)?),
                    InfixOperation::Gt => Ok(left.gt(right)?),
                    InfixOperation::Gte => Ok(left.gte(right)?),
                    InfixOperation::Lt => Ok(left.lt(right)?),
                    InfixOperation::Lte => Ok(left.lte(right)?),
                    _ => Ok(Object::Nil),
                }
            }
            Expression::Function(f) => Ok(f.eval(env)?),
        }
    }
}

impl Literal {
    pub fn eval(self) -> Result<Object> {
        match self {
            Literal::Int(int) => Ok(Object::Int(int)),
            Literal::True => Ok(Object::Bool(true)),
            Literal::False => Ok(Object::Bool(false)),
            Literal::Nil => Ok(Object::Nil),
            Literal::String(_) => Ok(Object::Bool(false)),
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{environment::Environment, lexer, object::Object, parser::Parser};
    use anyhow::Result;

    use super::Program;

    fn eval(text: &str) -> Result<Object> {
        let lexer = lexer::Lexer::new(text);
        let mut parser = Parser::new(lexer.peekable());
        let mut program = Program::default();
        let env = Environment::default();
        program.eval(&mut parser, Rc::new(RefCell::new(env)))
    }

    fn generate_eval(text: &str) -> Object {
        let eval = eval(text);
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
        let eval = eval(text);
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
        assert_eq!(generate_eval("let a = 5; a;"), Object::Int(5));
        assert_eq!(generate_eval("let a = 5 * 5; a"), Object::Int(25));
        assert_eq!(generate_eval("let a = 5; let b = a; b;"), Object::Int(5));
        assert_eq!(
            generate_eval("let a = 5; let b = a; let c = a + b + 5; c;"),
            Object::Int(15)
        );
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
        generate_eval_err("foobar", "identifier not found: foobar");
        assert_eq!(
            generate_eval("let identity = fn(x) { x; }; identity(5);"),
            Object::Int(5)
        );
        assert_eq!(
            generate_eval("let identity = fn(x) { return x; }; identity(5);"),
            Object::Int(5)
        );
        assert_eq!(
            generate_eval("let double = fn(x) { x * 2; }; double(5);"),
            Object::Int(10)
        );
        assert_eq!(
            generate_eval("let add = fn(x, y) { x + y; }; add(5, 5);"),
            Object::Int(10)
        );
        assert_eq!(
            generate_eval("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));"),
            Object::Int(20)
        );
        assert_eq!(generate_eval("fn(x) { x; }(5)"), Object::Int(5));
        assert_eq!(generate_eval("let add = fn(x, y) { x + y; };"), Object::Nil);
    }
}

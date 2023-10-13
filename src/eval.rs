use crate::{
    ast::{Block, Expression, If, InfixOperation, Literal, PrefixOperation, Statement},
    object::{Environment, Object, Stack},
    parser::Parser,
};

use anyhow::{bail, Result};

pub struct Program {}

impl Program {
    pub fn eval(&mut self, parser: &mut Parser, env: Environment) -> Result<Stack> {
        let mut result = Object::Nil;
        let mut current_env = env;

        for statement in parser.collect_statements() {
            let stack = statement.eval(current_env)?;
            result = stack.object;
            current_env = stack.env;
            if let Object::Return(expression) = result {
                return Ok(Stack {
                    object: *expression,
                    env: current_env,
                });
            }
        }
        Ok(Stack {
            object: result,
            env: current_env,
        })
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Statement {
    pub fn eval(&self, env: Environment) -> Result<Stack> {
        let mut current_env = env;
        match self {
            Statement::Return(expression) => {
                let stack = expression.eval(current_env)?;
                let current_env = stack.env;
                Ok(Stack {
                    object: Object::Return(Box::new(stack.object)),
                    env: current_env,
                })
            }
            Statement::Expression(expression) => expression.eval(current_env),
            Statement::Block(block) => block.eval(current_env),

            Statement::Let {
                identifier,
                expression,
            } => {
                let stack = expression.eval(current_env)?;
                current_env = stack.env;
                current_env
                    .map
                    .insert(identifier.get_name(), stack.object.clone());
                Ok(Stack::new(stack.object, current_env))
            }
        }
    }
}

impl Block {
    pub fn eval(&self, env: Environment) -> Result<Stack> {
        let mut result = Object::Nil;
        let mut current_env = env;
        for statement in &self.0 {
            let stack = statement.eval(current_env)?;
            result = stack.object;
            current_env = stack.env;
            if let Object::Return(_) = result {
                break;
            }
        }
        Ok(Stack {
            object: result,
            env: current_env,
        })
    }
}

impl If {
    pub fn eval(&self, env: Environment) -> Result<Stack> {
        let stack = self.condition.eval(env)?;
        match stack.object {
            Object::Nil => {
                let mut current_env = stack.env;
                let mut result = Object::Nil;
                for block in self.alternative.as_ref() {
                    let stack = block.eval(current_env)?;
                    result = stack.object;
                    current_env = stack.env;
                }
                Ok(Stack {
                    object: result,
                    env: current_env,
                })
            }
            Object::Int(_) => self.consequence.eval(stack.env),
            Object::Bool(b) => {
                if b {
                    let stack = self.consequence.eval(stack.env)?;
                    return Ok(Stack::new(stack.object, stack.env));
                }
                let mut current_env = stack.env;
                let mut result = Object::Nil;
                for block in self.alternative.as_ref() {
                    let stack = block.eval(current_env)?;
                    result = stack.object;
                    current_env = stack.env;
                }
                Ok(Stack {
                    object: result,
                    env: current_env,
                })
            }
            // Object::Return(_) => todo!(),
            _ => todo!(),
        }
    }
}

impl Expression {
    pub fn eval(&self, env: Environment) -> Result<Stack> {
        match self {
            Expression::Literal(literal) => Ok(Stack::new(literal.eval()?, env)),
            Expression::Prefix(prefix) => {
                let right = prefix.expression.eval(env)?;
                match prefix.operation {
                    PrefixOperation::Bang => Ok(Stack::new(right.object.bang()?, right.env)),
                    PrefixOperation::Minus => Ok(Stack::new(right.object.minus()?, right.env)),
                }
            }
            Expression::If(if_expression) => if_expression.eval(env),
            Expression::Identifier(id) => {
                let result = env.map.get(&id.get_name());
                match result {
                    Some(value) => Ok(Stack::new(value.clone(), env)),
                    None => {
                        bail!("identifier not found: {}", &id.get_name())
                    }
                }
            }
            Expression::Infix(infix) => {
                let left = infix.left_expression.eval(env)?;
                let right = infix.right_expression.eval(left.env)?;

                match infix.operation {
                    InfixOperation::Add => {
                        Ok(Stack::new(left.object.add(right.object)?, right.env))
                    }
                    InfixOperation::Sub => {
                        Ok(Stack::new(left.object.sub(right.object)?, right.env))
                    }
                    InfixOperation::Mul => {
                        Ok(Stack::new(left.object.mul(right.object)?, right.env))
                    }
                    InfixOperation::Div => {
                        Ok(Stack::new(left.object.div(right.object)?, right.env))
                    }
                    InfixOperation::Eq => Ok(Stack::new(left.object.eq(right.object)?, right.env)),
                    InfixOperation::NotEq => {
                        Ok(Stack::new(left.object.not_eq(right.object)?, right.env))
                    }
                    InfixOperation::Gt => Ok(Stack::new(left.object.gt(right.object)?, right.env)),
                    InfixOperation::Gte => {
                        Ok(Stack::new(left.object.gte(right.object)?, right.env))
                    }
                    InfixOperation::Lt => Ok(Stack::new(left.object.lt(right.object)?, right.env)),
                    InfixOperation::Lte => {
                        Ok(Stack::new(left.object.lte(right.object)?, right.env))
                    }
                    _ => Ok(Stack::new(Object::Nil, right.env)),
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
    use crate::{
        lexer,
        object::{Environment, Object},
        parser::Parser,
    };

    use super::Program;

    fn generate_eval(text: &str) -> Object {
        let mut lexer = lexer::Lexer::new_from_str(text);
        let mut parser = Parser::new(lexer.peekable_iter());
        let mut program = Program::new();
        let env = Environment::new();
        let eval = program.eval(&mut parser, env);
        match eval {
            Ok(expr) => {
                println!("{}", expr.object);
                expr.object
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
        let env = Environment::new();
        let eval = program.eval(&mut parser, env);

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
    }
}

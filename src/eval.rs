use crate::{
    ast::{Expression, Literal, Statement},
    object::Object,
    parser::Parser,
};

pub struct Program {}

impl Program {
    pub fn eval(&mut self, parser: &mut Parser) -> Object {
        parser
            .collect_statements()
            .iter()
            .fold(Object::Nil, |_, item| item.eval())
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Statement {
    pub fn eval(&self) -> Object {
        match self {
            Statement::Return(_) => todo!(),
            Statement::Expression(expression) => expression.eval(),
            _ => todo!(),
        }
    }
}

impl Expression {
    pub fn eval(&self) -> Object {
        match self {
            Expression::Literal(literal) => literal.eval(),
            _ => todo!(),
        }
    }
}

impl Literal {
    pub fn eval(&self) -> Object {
        match self {
            Literal::Int(int) => Object::Int(*int),
            Literal::True => Object::Bool(true),
            Literal::False => Object::Bool(false),
            Literal::Nil => Object::Nil,
            Literal::String(s) => Object::Bool(false),
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use crate::{
        lexer,
        object::Object,
        parser::{self, Parser},
    };

    use super::Program;

    fn generate_eval(text: &str) -> Object {
        let mut lexer = lexer::Lexer::new_from_str(text);
        let mut parser = Parser::new(lexer.peekable_iter());
        let mut program = Program::new();
        program.eval(&mut parser)
    }

    #[test]
    fn ev() {
        let eval = generate_eval(r#"5"#);
        assert_eq!(eval, Object::Int(5));
        assert_eq!(generate_eval("true"), Object::Bool(true));
        assert_eq!(generate_eval("false"), Object::Bool(false));
        println!("eval is {eval}");
    }
}

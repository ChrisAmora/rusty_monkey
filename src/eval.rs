use crate::{
    ast::{Block, Expression, If, InfixOperation, Literal, PrefixOperation, Statement},
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
            Statement::Block(block) => block.eval(),
            _ => todo!(),
        }
    }
}

impl Block {
    pub fn eval(&self) -> Object {
        self.0.iter().fold(Object::Nil, |_, item| item.eval())
    }
}

impl If {
    pub fn eval(&self) -> Object {
        match self.condition.eval() {
            Object::Nil => self
                .alternative
                .as_ref()
                .map_or(Object::Nil, |block| block.eval()),
            Object::Int(_) => self.consequence.eval(),
            Object::Bool(b) => {
                if b {
                    return self.consequence.eval();
                }
                self.alternative
                    .as_ref()
                    .map_or(Object::Nil, |block| block.eval())
            }
        }
    }
}

impl Expression {
    pub fn eval(&self) -> Object {
        match self {
            Expression::Literal(literal) => literal.eval(),
            Expression::Prefix(prefix) => {
                let right = prefix.expression.eval();
                match prefix.operation {
                    PrefixOperation::Bang => right.bang(),
                    PrefixOperation::Minus => right.minus(),
                }
            }
            Expression::If(if_expression) => if_expression.eval(),
            Expression::Infix(infix) => {
                let left = infix.left_expression.eval();
                let right = infix.right_expression.eval();

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
                    _ => Object::Nil,
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
    pub fn eval(&self) -> Object {
        match self {
            Literal::Int(int) => Object::Int(*int),
            Literal::True => Object::Bool(true),
            Literal::False => Object::Bool(false),
            Literal::Nil => Object::Nil,
            Literal::String(_) => Object::Bool(false),
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
        println!("eval {text} is {eval}");
        eval
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
    }
}

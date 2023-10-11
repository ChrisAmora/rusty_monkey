use crate::ast::{
    Block, Call, Expression, Function, If, Infix, Literal, Prefix, PrefixOperation, Statement,
};
use crate::token::{Identifier, TokenType};
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<TokenType>>,
}

impl Parser {
    pub fn new(tokens: Peekable<IntoIter<TokenType>>) -> Self {
        Parser { tokens }
    }

    pub fn parse_next_statement(&mut self) -> Option<Statement> {
        self.tokens
            .next()
            .and_then(|token| self.parse_statement(token))
    }

    fn parse_statement(&mut self, token: TokenType) -> Option<Statement> {
        match token {
            TokenType::Let => {
                let statement = self.parse_let();
                Some(statement)
            }
            TokenType::Return => {
                let statement = self.parse_expr_statement();
                Some(statement)
            }
            token => {
                let expression = self.parse_expression(0, token);
                self.assert_next_and_advance(TokenType::Semicolon);
                Some(Statement::Expression(expression))
            }
        }
    }

    pub fn assert_next_and_advance(&mut self, token: TokenType) -> Option<TokenType> {
        self.tokens.next_if_eq(&token)
    }

    pub fn try_next_token(&mut self) -> TokenType {
        self.tokens.next().unwrap()
    }

    pub fn assert_next_ident(&mut self) -> Identifier {
        self.try_next_token().try_into().unwrap()
    }

    pub fn parse_expr_statement(&mut self) -> Statement {
        let new_token = self.try_next_token();
        let left = self.parse_expression(0, new_token);
        self.tokens.next_if_eq(&TokenType::Semicolon);
        Statement::Return(left)
    }

    pub fn parse_let(&mut self) -> Statement {
        let token = self.try_next_token();
        let identifier = Identifier(token.to_string());
        self.assert_next_and_advance(TokenType::Assign);
        let current_token = self.try_next_token();
        let expression = self.parse_expression(0, current_token);
        self.tokens.next_if_eq(&TokenType::Semicolon);
        Statement::Let {
            identifier,
            expression,
        }
    }

    pub fn parse_expression(&mut self, precedente: usize, current_token: TokenType) -> Expression {
        let mut left = self.parse_prefix(current_token).unwrap();

        while let Some(next) = self.tokens.next_if(|peek| {
            peek != &TokenType::Semicolon
                && precedente < peek.precedence()
                && peek.operation().is_some()
        }) {
            match next {
                TokenType::LParen => left = self.parse_call_expression(left),
                _ => left = self.parse_infix_expression(left, next),
            }
        }
        left
    }

    fn parse_prefix_expression(&mut self, operation: PrefixOperation) -> Expression {
        let token = self.try_next_token();
        let expression = self.parse_expression(6, token);
        Expression::Prefix(Prefix {
            operation,
            expression: expression.boxed(),
        })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        let previous_token = self.try_next_token();
        let expression = self.parse_expression(0, previous_token);
        self.assert_next_and_advance(TokenType::RParen)?;
        Some(expression)
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        self.assert_next_and_advance(TokenType::LParen)?;
        let current_token = self.try_next_token();
        let condition = self.parse_expression(0, current_token).boxed();
        self.assert_next_and_advance(TokenType::RParen)?;
        self.assert_next_and_advance(TokenType::LBrace)?;
        let consequence = self.parse_block();
        let mut alternative: Option<Block> = None;
        if self.assert_peek(&TokenType::Else) {
            self.try_next_token();
            self.assert_next_and_advance(TokenType::LBrace);
            alternative = Some(self.parse_block());
        }
        Some(Expression::If(If {
            condition,
            alternative,
            consequence,
        }))
    }

    fn parse_block(&mut self) -> Block {
        let mut current_token = self.try_next_token();
        let mut statements = vec![];
        while current_token != TokenType::RBrace {
            let statement = self.parse_statement(current_token);
            statements.push(statement);
            current_token = self.try_next_token();
        }
        Block(statements.into_iter().flatten().collect())
    }

    fn parse_function(&mut self) -> Expression {
        self.assert_next_and_advance(TokenType::LParen);
        let params = self.parse_function_params();
        self.assert_next_and_advance(TokenType::LBrace);
        let body = self.parse_block();
        Expression::Function(Function { body, params })
    }

    fn parse_function_params(&mut self) -> Vec<Identifier> {
        let mut identifiers = vec![];
        if self.tokens.next_if_eq(&TokenType::RParen).is_some() {
            return identifiers;
        };
        let token = self.try_next_token();
        identifiers.push(Identifier::new(token.to_string()));
        while self.tokens.peek().unwrap() == &TokenType::Comma {
            self.try_next_token();
            let current_token = self.try_next_token();
            identifiers.push(Identifier::new(current_token.to_string()));
        }
        self.assert_next_and_advance(TokenType::RParen);
        identifiers
    }

    fn parse_call_arguments(&mut self) -> Vec<Expression> {
        let mut args = vec![];
        if self.tokens.next_if_eq(&TokenType::RParen).is_some() {
            return args;
        };
        let current_token = self.try_next_token();
        args.push(self.parse_expression(0, current_token));
        while self.tokens.peek().unwrap() == &TokenType::Comma {
            self.try_next_token();
            let current_token = self.try_next_token();
            args.push(self.parse_expression(0, current_token));
        }
        self.assert_next_and_advance(TokenType::RParen);
        args
    }

    fn parse_call_expression(&mut self, function: Expression) -> Expression {
        let arguments = self.parse_call_arguments();
        Expression::Call(Call {
            function: function.boxed(),
            arguments,
        })
    }

    fn assert_peek(&mut self, token: &TokenType) -> bool {
        self.tokens.peek().unwrap() == token
    }

    fn parse_infix_expression(
        &mut self,
        left_expression: Expression,
        token: TokenType,
    ) -> Expression {
        let precedence = token.precedence();
        let operation = token.operation().unwrap();

        let token_new = self.try_next_token();
        let right_expression = self.parse_expression(precedence, token_new);
        Expression::Infix(Infix {
            right_expression: right_expression.boxed(),
            operation,
            left_expression: left_expression.boxed(),
        })
    }

    pub fn parse_prefix(&mut self, token: TokenType) -> Option<Expression> {
        match token {
            TokenType::Identifier(name) => Some(Expression::Identifier(name.to_owned())),
            TokenType::Int(num) => Some(Expression::Literal(Literal::Int(num.to_owned()))),
            TokenType::True => Some(Expression::Literal(Literal::True)),
            TokenType::False => Some(Expression::Literal(Literal::False)),
            TokenType::Nil => Some(Expression::Literal(Literal::Nil)),
            TokenType::Bang => Some(self.parse_prefix_expression(PrefixOperation::Bang)),
            TokenType::Minus => Some(self.parse_prefix_expression(PrefixOperation::Minus)),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::If => self.parse_if_expression(),
            TokenType::Function => Some(self.parse_function()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Expression, Infix, InfixOperation, Literal, Prefix, PrefixOperation, Statement},
        parser::Parser,
        token::Identifier,
    };

    #[test]
    fn let_expression() {
        use crate::lexer;
        let program = r#"
        let five = 5;
        let ten = 10 + 2;
        "#;

        let mut lexer = lexer::Lexer::new(program.chars().peekable());
        let peek = lexer.peekable_iter();
        let mut parser = Parser::new(peek);

        let expected_vec = vec![
            Identifier::new("five".to_string()),
            Identifier::new("ten".to_string()),
        ];

        let mut expected = expected_vec.iter();

        while let Some(statement) = parser.parse_next_statement() {
            match statement {
                Statement::Let {
                    identifier,
                    expression,
                } => {
                    assert_eq!(&identifier, expected.next().unwrap());
                }
                _ => {}
            }
        }
    }

    #[test]
    fn return_expression() {
        use crate::lexer;
        let program = r#"
        return 10;
        "#;

        let mut lexer = lexer::Lexer::new(program.chars().peekable());
        let peek = lexer.peekable_iter();
        let mut parser = Parser::new(peek);

        while let Some(statement) = parser.parse_next_statement() {
            assert!(matches!(
                statement,
                Statement::Return(Expression::Literal(Literal::Int(10)))
            ));
        }
    }

    #[test]
    fn ident_expression() {
        use crate::lexer;

        let program = r#"
        foobar;
        baafoo;
        lasagna;
        5;
        6;
        !5;
        -8;
        potato;
        5 + 5;
        3 - 9;
        3 * 9;
        foo * bar;
        88 + 2 * 3;
        "#;

        let mut lexer = lexer::Lexer::new(program.chars().peekable());
        let peek = lexer.peekable_iter();
        let mut parser = Parser::new(peek);

        let expected_vec = vec![
            Expression::Identifier(Identifier("foobar".to_string())),
            Expression::Identifier(Identifier("baafoo".to_string())),
            Expression::Identifier(Identifier("lasagna".to_string())),
            Expression::Literal(Literal::Int(5)),
            Expression::Literal(Literal::Int(6)),
            Expression::Prefix(Prefix {
                operation: PrefixOperation::Bang,
                expression: Box::new(Expression::Literal(Literal::Int(5))),
            }),
            Expression::Prefix(Prefix {
                operation: PrefixOperation::Minus,
                expression: Box::new(Expression::Literal(Literal::Int(8))),
            }),
            Expression::Identifier(Identifier("potato".to_string())),
            Expression::Infix(Infix {
                left_expression: Box::new(Expression::Literal(Literal::Int(5))),
                right_expression: Box::new(Expression::Literal(Literal::Int(5))),
                operation: InfixOperation::Add,
            }),
            Expression::Infix(Infix {
                left_expression: Box::new(Expression::Literal(Literal::Int(3))),
                right_expression: Box::new(Expression::Literal(Literal::Int(9))),
                operation: InfixOperation::Sub,
            }),
            Expression::Infix(Infix {
                left_expression: Box::new(Expression::Literal(Literal::Int(3))),
                right_expression: Box::new(Expression::Literal(Literal::Int(9))),
                operation: InfixOperation::Mul,
            }),
            Expression::Infix(Infix {
                left_expression: Expression::Identifier(Identifier::new_str("foo")).boxed(),
                right_expression: Expression::Identifier(Identifier::new_str("bar")).boxed(),
                operation: InfixOperation::Mul,
            }),
            Expression::Infix(Infix {
                left_expression: Expression::Literal(Literal::Int(88)).boxed(),
                right_expression: Expression::Infix(Infix {
                    left_expression: Expression::Literal(Literal::Int(2)).boxed(),
                    right_expression: Expression::Literal(Literal::Int(3)).boxed(),
                    operation: InfixOperation::Mul,
                })
                .boxed(),
                operation: InfixOperation::Add,
            }),
        ];

        let mut expected = expected_vec.iter();

        while let Some(statement) = parser.parse_next_statement() {
            match statement {
                Statement::Expression(expression) => {
                    assert_eq!(&expression, expected.next().unwrap());
                }
                _ => {
                    println!("here");
                    panic!("here");
                }
            }
        }
    }

    #[test]
    fn test_as_str() {
        use crate::lexer;

        let program = r#"
        foobar;
        baafoo;
        lasagna;
        5;
        6;
        !5;
        -8;
        potato;
        5 + 5;
        3 - 9;
        3 * 9;
        foo * bar;
        88 + 2 * 3;
        1 + 2 + 3;
        false == false;
        false <= true;
        1 + (2 + 3) + 4;
        !(true == true);
        -(5 + 5);
        2 / (5 + 5);
        (5 + 5) * 2;
        if (5 + 10) {10 + 3};
        if (!true) {!5} else {99 + 2};
        fn(x, y) { x + y; };
        fn() {};
        fn(x) {};
        fn(x, y, z) {};
        a + add(b * c) + d;
        let x = fn(x) {5 + 3};
        return x(5+3);
        "#;

        // let program = r#"
        // let foobar = y;
        // "#;

        let mut lexer = lexer::Lexer::new(program.chars().peekable());
        let peek = lexer.peekable_iter();
        let mut parser = Parser::new(peek);

        let expected_vec = vec![
            String::from("foobar"),
            String::from("baafoo"),
            String::from("lasagna"),
            String::from("5"),
            String::from("6"),
            String::from("(!5)"),
            String::from("(-8)"),
            String::from("potato"),
            String::from("(5+5)"),
            String::from("(3-9)"),
            String::from("(3*9)"),
            String::from("(foo*bar)"),
            String::from("(88+(2*3))"),
            String::from("((1+2)+3)"),
            String::from("(false==false)"),
            String::from("(false<=true)"),
            String::from("((1+(2+3))+4)"),
            String::from("(!(true==true))"),
            String::from("(-(5+5))"),
            String::from("(2/(5+5))"),
            String::from("((5+5)*2)"),
            String::from("if (5+10) (10+3)"),
            String::from("if (!true) (!5) else (99+2)"),
            String::from("fn (x, y) (x+y)"),
            String::from("fn ()"),
            String::from("fn (x)"),
            String::from("fn (x, y, z)"),
            String::from("((a+add ((b*c)))+d)"),
            String::from("let x = fn (x) (5+3)"),
            String::from("return x ((5+3))"),
        ];

        // let expected_vec = vec![String::from("(mock!)")];

        let mut expected = expected_vec.iter();
        while let Some(statement) = parser.parse_next_statement() {
            let formatted = format!("{statement}");
            println!("{formatted}");
            assert_eq!(&formatted, expected.next().unwrap());
        }
    }
}

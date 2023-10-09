#![allow(clippy::unused_unit, clippy::single_match, clippy::needless_return)]
use crate::ast::{Expression, Infix, Literal, Prefix, PrefixOperation, Statement};
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
        if let Some(token) = self.tokens.next() {
            return match token {
                TokenType::Let => {
                    let identifier = self.assert_next_ident();
                    self.assert_next_and_advance(TokenType::Assign).unwrap();
                    let statement = self.parse_expr_statement();
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
            };
        }
        None
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
        while self.assert_next_and_advance(TokenType::Semicolon).is_some() {
            continue;
        }

        Statement::Return(left)
    }

    pub fn parse_expression(&mut self, precedente: usize, current_token: TokenType) -> Expression {
        let mut left = self.parse_prefix(current_token).unwrap();
        while !self.assert_peek(&TokenType::Semicolon) && precedente < self.peek_precedence() {
            let next = self.tokens.next_if(|token| token.operation().is_some());
            match next {
                Some(next) => left = self.parse_infix_expression(left, next),
                None => break,
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

    fn peek_precedence(&mut self) -> usize {
        self.tokens.peek().unwrap().precedence()
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
        "#;

        // let program = r#"
        // !(true);
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
        ];

        // let expected_vec = vec![String::from("(!true)")];

        let mut expected = expected_vec.iter();

        while let Some(statement) = parser.parse_next_statement() {
            match statement {
                Statement::Expression(expression) => {
                    let formatted = format!("{}", expression);
                    println!("{}", expression);
                    assert_eq!(&formatted, expected.next().unwrap());
                }
                _ => {
                    println!("here");
                    panic!("here");
                }
            }
        }
    }
}

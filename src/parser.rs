#![allow(dead_code, unused_variables)]
#![allow(clippy::unused_unit, clippy::single_match, clippy::needless_return)]
use crate::ast::{Expression, Infix, Literal, Prefix, PrefixOperation, Statement};
use crate::token::{Identifier, TokenType};
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<TokenType>>,
    current_token: Option<TokenType>,
}

impl Parser {
    pub fn new(tokens: Peekable<IntoIter<TokenType>>) -> Self {
        Parser {
            tokens,
            current_token: None,
        }
    }

    pub fn parse_next_statement(&mut self) -> Option<Statement> {
        if let Some(token) = self.tokens.next() {
            self.current_token = Some(token.clone());
            return match token {
                TokenType::Let => {
                    let identifier = self.assert_next_ident();
                    self.assert_next(TokenType::Assign).unwrap();
                    let expression = self.parse_expr_statement();
                    // self.assert_next(TokenType::Semicolon);
                    Some(Statement::Let {
                        identifier,
                        expression,
                    })
                }
                TokenType::Return => {
                    let expression = self.parse_expr_statement();
                    Some(Statement::Return)
                }
                token => {
                    let expression = self.parse_expression(0)?;
                    self.assert_next(TokenType::Semicolon);
                    Some(Statement::Expression(expression))
                }
            };
        }
        None
    }

    pub fn assert_next(&mut self, token: TokenType) -> Option<TokenType> {
        self.tokens.next_if_eq(&token)
    }

    pub fn try_next_token(&mut self) -> TokenType {
        let next_token = self.tokens.next();
        self.current_token = next_token.clone();
        next_token.unwrap()
    }

    pub fn assert_next_ident(&mut self) -> Identifier {
        self.try_next_token().try_into().unwrap()
    }

    pub fn parse_expr_statement(&mut self) -> Expression {
        while self.try_next_token() != TokenType::Semicolon {
            continue;
        }

        Expression::Identifier(Identifier::new("lol".to_string()))
    }

    pub fn parse_expression(&mut self, precedente: usize) -> Option<Expression> {
        let mut left = self.parse_prefix().unwrap();

        while !self.assert_peek(TokenType::Semicolon) && precedente < self.peek_precedence() {
            let token = self.peek();
            match token.operation() {
                Some(_) => {
                    self.try_next_token();
                    left = self.parse_infix_expression(left);
                }
                None => break,
            }
        }
        Some(left)
    }

    fn parse_prefix_expression(&mut self, operation: crate::ast::PrefixOperation) -> Expression {
        let token = self.try_next_token();
        let expression = self.parse_expression(6).unwrap();
        Expression::Prefix(Prefix {
            operation,
            expression: Box::new(expression),
        })
    }

    fn peek_precedence(&mut self) -> usize {
        let current_token = self.tokens.peek().unwrap();
        current_token.precedence()
    }

    fn peek(&mut self) -> &TokenType {
        self.tokens.peek().unwrap()
    }

    fn assert_peek(&mut self, token: TokenType) -> bool {
        self.peek().clone() == token
    }

    fn parse_infix_expression(&mut self, left_expression: Expression) -> Expression {
        let precedence = self.peek_precedence();
        let token = self.current_token.clone().unwrap();
        let operation = token.operation().unwrap();
        self.try_next_token();
        let right_expression = self.parse_expression(precedence).unwrap();
        Expression::Infix(Infix {
            right_expression: Box::new(right_expression),
            operation,
            left_expression: Box::new(left_expression),
        })
    }

    pub fn parse_prefix(&mut self) -> Option<Expression> {
        let token = self.current_token.clone().unwrap();
        match token {
            TokenType::Identifier(name) => Some(Expression::Identifier(name.to_owned())),
            TokenType::Int(num) => Some(Expression::Literal(Literal::Int(num.to_owned()))),
            TokenType::Bang => Some(self.parse_prefix_expression(PrefixOperation::Bang)),
            TokenType::Minus => Some(self.parse_prefix_expression(PrefixOperation::Minus)),
            _ => None,
        }
    }
}

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

        let mut lexer = lexer::Lexer::new(program.chars());
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
        return 5;
        return 10;
        return 993322;
        "#;

        let mut lexer = lexer::Lexer::new(program.chars());
        let peek = lexer.peekable_iter();
        let mut parser = Parser::new(peek);

        while let Some(statement) = parser.parse_next_statement() {
            assert!(matches!(statement, Statement::Return));
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

        let mut lexer = lexer::Lexer::new(program.chars());
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
                    println!("{}", expression);
                    assert_eq!(&expression, expected.next().unwrap());
                }
                _ => {
                    println!("here");
                    panic!("here");
                }
            }
        }
    }
}

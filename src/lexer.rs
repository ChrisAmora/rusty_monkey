#![allow(dead_code)]
use crate::token::{Identifier, TokenType};
use std::iter::{self, Peekable};
use std::str::Chars;

pub struct Lexer<'a> {
    chars_iter: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = TokenType;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(char) = self.chars_iter.next() {
            return match char {
                ' ' => self.next(),
                '\n' => self.next(),
                ',' => Some(TokenType::Comma),
                ':' => Some(TokenType::Colon),
                ';' => Some(TokenType::Semicolon),
                '(' => Some(TokenType::LParen),
                ')' => Some(TokenType::RParen),
                '[' => Some(TokenType::LBracket),
                ']' => Some(TokenType::RBracket),
                '{' => Some(TokenType::LBrace),
                '}' => Some(TokenType::RBrace),
                '-' => Some(TokenType::Minus),
                '+' => Some(TokenType::Plus),
                '*' => Some(TokenType::Asterisk),
                '.' => Some(TokenType::Dot),
                '/' => Some(TokenType::Slash),
                '=' => self
                    .chars_iter
                    .next_if_eq(&'=')
                    .map_or(Some(TokenType::Assign), |_| Some(TokenType::Eq)),
                '!' => self
                    .chars_iter
                    .next_if_eq(&'=')
                    .map_or(Some(TokenType::Bang), |_| Some(TokenType::NotEq)),
                '<' => self
                    .chars_iter
                    .next_if_eq(&'=')
                    .map_or(Some(TokenType::Lt), |_| Some(TokenType::Lte)),
                '>' => self
                    .chars_iter
                    .next_if_eq(&'=')
                    .map_or(Some(TokenType::Gt), |_| Some(TokenType::Gte)),
                num if num.is_ascii_digit() => iter::once(num)
                    .chain(iter::from_fn(|| {
                        self.chars_iter.next_if(|char| char.is_ascii_digit())
                    }))
                    .collect::<String>()
                    .parse::<i64>()
                    .map_or(Some(TokenType::Illegal), |x| Some(TokenType::Int(x))),
                ch if ch.is_alphabetic() => {
                    let result = iter::once(ch)
                        .chain(iter::from_fn(|| {
                            self.chars_iter.next_if(|char| char.is_alphabetic())
                        }))
                        .collect::<String>();

                    match result.as_str() {
                        "fn" => Some(TokenType::Function),
                        "let" => Some(TokenType::Let),
                        "false" => Some(TokenType::False),
                        "true" => Some(TokenType::True),
                        "if" => Some(TokenType::If),
                        "else" => Some(TokenType::Else),
                        "return" => Some(TokenType::Return),
                        "nil" => Some(TokenType::Nil),
                        _ => Some(TokenType::Identifier(Identifier::new(result))),
                    }
                }
                _ => Some(TokenType::Illegal),
            };
        } else {
            None
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Lexer {
            chars_iter: text.chars().peekable(),
        }
    }
}

mod test {
    #[test]
    fn parse() {
        use crate::lexer;
        let program = r#"
let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
return true;
} else {
return false;
}
10 == 10;
10 != 9;
"#;

        let mut lexer = lexer::Lexer::new(program);

        while let Some(l) = lexer.next() {
            println!("{:?}", l);
        }
    }
}

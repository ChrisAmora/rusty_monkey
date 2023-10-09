#![allow(dead_code)]
use crate::token::{Identifier, TokenType};
use std::iter::{self, Peekable};
use std::str::Chars;
use std::vec::IntoIter;

pub struct Lexer {
    chars_iter: Peekable<Chars<'static>>,
}

impl Lexer {
    pub fn new(chars_iter: Peekable<Chars<'static>>) -> Self {
        Lexer { chars_iter }
    }

    pub fn peekable_iter(&mut self) -> Peekable<IntoIter<TokenType>> {
        let mut all_collected = vec![];
        while let Some(token) = self.get_next_token() {
            all_collected.push(token);
        }
        all_collected.into_iter().peekable()
    }

    pub fn get_next_token(&mut self) -> Option<TokenType> {
        if let Some(char) = self.chars_iter.next() {
            return match char {
                ' ' => self.get_next_token(),
                // '\n' => Some(TokenType::LineBreak),
                '\n' => self.get_next_token(),
                ',' => Some(TokenType::Comma),
                ':' => Some(TokenType::Colon),
                ';' => Some(TokenType::Semicolon),
                '(' => Some(TokenType::LParen),
                ')' => Some(TokenType::RParen),
                '[' => Some(TokenType::LBracket),
                ']' => Some(TokenType::RBracket),
                '{' => Some(TokenType::LBracket),
                '}' => Some(TokenType::RBracket),
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

        let mut lexer = lexer::Lexer::new(program.chars().peekable());
    }
}

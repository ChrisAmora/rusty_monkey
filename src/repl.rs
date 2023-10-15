use std::{cell::RefCell, io, rc::Rc};

use crate::{environment::Environment, eval::Program, lexer::Lexer, parser::Parser};
pub struct Repl {}

impl Repl {
    pub fn start(&self) {
        println!("Hello This is the Monkey programming language!");
        println!("Feel free to type in commands");
        let new_env = Environment::new();
        let env = Rc::new(RefCell::new(new_env));

        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            input = input.trim().to_string();

            if input == "exit" {
                return;
            }
            if input.is_empty() {
                continue;
            }

            let mut lexer = Lexer::new_from_str(input.as_str());
            let mut parser = Parser::new(lexer.peekable_iter());
            let mut program = Program::new();
            let eval = program.eval(&mut parser, env.clone());
            match eval {
                Ok(stack) => {
                    println!("{}", stack)
                }
                Err(err) => {
                    println!("error: {:?}", err)
                }
            }
        }
    }
}

use std::io;

use crate::{eval::Program, lexer::Lexer, object::Environment, parser::Parser};
pub struct Repl {}

impl Repl {
    pub fn start(&self) {
        println!("Hello This is the Monkey programming language!");
        println!("Feel free to type in commands");
        let mut env = Environment::new();

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
            let eval = program.eval(&mut parser, env);
            match eval {
                Ok(stack) => {
                    env = stack.env;
                    println!("{}", stack.object)
                }
                Err(err) => {
                    env = Environment::new();
                    println!("error: {:?}", err)
                }
            }
        }
    }
}

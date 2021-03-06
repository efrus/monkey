use crate::environment::Environment;
use crate::evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::cell::RefCell;
use std::error::Error;
use std::io::{self, Write};
use std::rc::Rc;

const PROMPT: &str = ">> ";

pub fn start() -> Result<(), Box<dyn Error>> {
    let env = Rc::new(RefCell::new(Environment::default()));
    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if !parser.errors().is_empty() {
            println!("Woops! We ran into some monkey business here!");
            println!(" parser errors: ");
            for s in parser.errors() {
                println!("{}", s);
            }
            continue;
        }

        let evaluated = evaluator::eval(program, env.clone());
        println!("{}", evaluated.inspect());
    }
}

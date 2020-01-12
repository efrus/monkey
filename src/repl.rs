use crate::evaluator;
use crate::lexer::Lexer;
use crate::object::Environment;
use crate::parser::Parser;
use std::error::Error;
use std::io::{self, Write};
use std::rc::Rc;

const PROMPT: &str = ">> ";

pub fn start() -> Result<(), Box<dyn Error>> {
    let env = Rc::new(Environment::new());
    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if parser.errors().len() > 0 {
            println!("Woops! We ran into some monkey business here!");
            println!(" parser errors: ");
            for s in parser.errors() {
                println!("{}", s);
            }
            continue;
        }

        let evaluated = evaluator::eval(program, env.clone());
        println!("{}", evaluated.inspect());

        //println!("{}", program.to_string());
    }
}

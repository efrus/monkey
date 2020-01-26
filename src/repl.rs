use crate::environment::Environment;
use crate::evaluator::evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::cell::RefCell;
use std::error::Error;
use std::io::{self, Write};
use std::rc::Rc;

const PROMPT: &str = ">> ";

pub fn start() -> Result<(), Box<dyn Error>> {
    let env = Rc::new(RefCell::new(Environment::new()));
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
    }
}

pub fn interpret_text(input: &String) -> String {
    let env = Rc::new(RefCell::new(Environment::new()));
    interpret_text_env(input, env)
}

pub fn interpret_text_env(input: &String, env: Rc<RefCell<Environment>>) -> String {
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if parser.errors().len() > 0 {
        let mut output = vec![];
        output.push("Woops! We ran into some monkey business here!");
        output.push(" parser errors: ");
        for s in parser.errors() {
            output.push(s);
        }
        return output.join("\n");
    }

    let evaluated = evaluator::eval(program, env);
    evaluated.inspect()
}

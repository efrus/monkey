mod ast;
pub mod environment;
mod evaluator;
mod lexer;
mod object;
mod parser;
pub mod repl;
mod tests;
mod token;

use crate::environment::Environment;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;

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

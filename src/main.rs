mod ast;
mod ast_test;
mod environment;
mod evaluator;
mod lexer;
mod lexer_test;
mod object;
mod object_test;
mod parser;
mod parser_test;
mod repl;
mod token;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello!  This is the Monkey programming Language.");
    repl::start()?;

    Ok(())
}

mod ast;
mod lexer;
mod repl;
mod token;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello!  This is the Monkey programming Language.");
    repl::start()?;

    Ok(())
}

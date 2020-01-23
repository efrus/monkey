use monkey::repl;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello!  This is the Monkey programming Language.");
    repl::start()?;

    Ok(())
}

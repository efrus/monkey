use monkey;
use monkey::repl;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Hello!  This is the Monkey programming Language.");
        repl::start()?;
    } else {
        //load file
        let file = &args[1];
        let contents = fs::read_to_string(file)?;
        let output = monkey::interpret_text(&contents);
        println!("{}", output);
    }

    Ok(())
}

use crate::lexer::Lexer;
use std::error::Error;
use std::io::{self, Write};

const PROMPT: &str = ">> ";

pub fn start() -> Result<(), Box<dyn Error>> {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut l = Lexer::new(&input);
        while let Some(tok) = l.next_token() {
            println!("{:?}", tok);
        }
    }
}

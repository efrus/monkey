use crate::ast::{Expression, Identifier, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Parser<'a> {
        let current_token = None;
        let peek_token = lexer.next_token();

        Parser {
            lexer: lexer,
            current_token,
            peek_token,
        }
    }

    pub fn parse_program(&self) -> Option<Program> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Identifier, Program, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    //use crate::token::Token;

    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        if let Some(program) = parser.parse_program() {
            assert_eq!(3, program.statements.len());

            let tests = ["x", "y", "foobar"];

            let mut statements = program.statements.into_iter();
            for test in tests.iter() {
                let s = statements.next().unwrap();
                test_let_statement(s, test);
            }
        } else {
            println!("Parse Program returned None!");
            assert!(false);
        }
    }

    fn test_let_statement(s: Statement, name: &str) {
        match s {
            Statement::Let(ident, _) => {
                assert_eq!(ident, name);
            }
            _ => assert!(false),
        };
    }
}

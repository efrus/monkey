#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Identifier, Program, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    //use crate::token::Token;

    fn check_parser_errors(parser: &Parser) {
        for error in parser.errors() {
            println!("{}", error);
        }
        assert_eq!(parser.errors().len(), 0);
    }

    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(3, program.statements.len());
        let tests = ["x", "y", "foobar"];

        let mut statements = program.statements.into_iter();
        for test in tests.iter() {
            let s = statements.next().unwrap();
            test_let_statement(s, test);
        }
    }

    fn test_let_statement(s: Statement, name: &str) {
        match s {
            Statement::Let(ident, _) => {
                assert_eq!(ident, name);
            }
            _ => {
                println!("did not get let");
                assert!(false);
            }
        };
    }
    #[test]
    fn test_return_statements() {
        let input = "
            return 5;
            return 10;
            return 838383;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(3, program.statements.len());
        for s in program.statements {
            match s {
                Statement::Return(_) => {
                    continue;
                }
                _ => {
                    println!("did not get return");
                    assert!(false);
                }
            }
        }
    }
}
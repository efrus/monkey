#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Statement};
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

    #[test]
    fn test_identifier_expression() {
        let input = "foobar";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::Ident(ident) if ident == "foobar" => (),
                    _ => {
                        println!("Expected ident, got something else.");
                        assert!(false);
                    }
                },
                _ => {
                    println!("Expected Statement expr, got something else.");
                    assert!(false);
                }
            }
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::IntegerLiteral(int) if int == 5 => (),
                    _ => {
                        println!("Expected 5, got something else.");
                        assert!(false);
                    }
                },
                _ => {
                    println!("Expected Statement expr, got something else.");
                    assert!(false);
                }
            }
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let tests = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for test in tests {
            let lexer = Lexer::new(test.0);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parser_errors(&parser);
            assert_eq!(program.statements.len(), 1);

            if let Some(statement) = program.statements.into_iter().next() {
                match statement {
                    Statement::Expression(expr) => match expr {
                        Expression::Prefix(operator, right) => {
                            if operator != test.1 {
                                println!("operator is not {}, got {}", test.1, operator);
                                assert!(false);
                            }

                            if !test_integer_literal(*right, test.2) {
                                println!("right not equal to integer test");
                                assert!(false);
                            }
                        }
                        _ => {
                            println!("Not a prefix expression");
                            assert!(false);
                        }
                    },
                    _ => {
                        println!("Expected Statement expr, got something else.");
                        assert!(false);
                    }
                }
            }
        }
    }

    fn test_integer_literal(expression: Expression, value: i64) -> bool {
        match expression {
            Expression::IntegerLiteral(i) => {
                if i != value {
                    println!("integer value not {}, got {} ", value, i);
                    false
                } else {
                    true
                }
            }
            _ => {
                println!("Expression not integer literal");
                false
            }
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let tests = vec![
            ("5 + 5;", 5, "+", 5),
            //("5 - 5;", 5, "-", 5),
            //("5 * 5;", 5, "*", 5),
            //("5 / 5;", 5, "/", 5),
            //("5 > 5;", 5, ">", 5),
            //("5 < 5;", 5, "<", 5),
            //("5 == 5;", 5, "==", 5),
            //("5 != 5;", 5, "!=", 5),
        ];
        for test in tests {
            let lexer = Lexer::new(test.0);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parser_errors(&parser);
            assert_eq!(program.statements.len(), 1);

            if let Some(statement) = program.statements.into_iter().next() {
                match statement {
                    Statement::Expression(expr) => match expr {
                        Expression::Infix(left, operator, right) => {
                            if !test_integer_literal(*left, test.1) {
                                println!("left not equal to integer test");
                                assert!(false);
                            }

                            if operator != test.2 {
                                println!("operator is not {}, got {}", test.1, operator);
                                assert!(false);
                            }

                            if !test_integer_literal(*right, test.3) {
                                println!("right not equal to integer test");
                                assert!(false);
                            }
                        }
                        _ => {
                            println!("Not a prefix expression");
                            assert!(false);
                        }
                    },
                    _ => {
                        println!("Expected Statement expr, got something else.");
                        assert!(false);
                    }
                }
            }
        }
    }
}

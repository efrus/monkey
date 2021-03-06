#[cfg(test)]
mod tests {
    use crate::ast;
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
    fn test_let_statements_int() {
        let (input, expected_identifier, expected_value) = ("let x = 5;", "x", 5);

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(1, program.statements.len());
        let statement = &program.statements[0];
        test_let_statement(statement, expected_identifier);
        match statement {
            Statement::Let(_ident, expr) => {
                assert_eq!(test_integer_literal(expr, expected_value), true);
            }
            _ => {
                println!("Expected Let stmt, got somethingn else");
                assert!(false);
            }
        }
    }

    fn test_let_statements_bool() {
        let (input, expected_identifier, expected_value) = ("let y = true;", "y", true);

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(1, program.statements.len());
        let statement = &program.statements[0];
        test_let_statement(statement, expected_identifier);
        match statement {
            Statement::Let(_ident, expr) => {
                assert_eq!(test_bool_literal(expr, expected_value), true);
            }
            _ => {
                println!("Expected Let stmt, got somethingn else");
                assert!(false);
            }
        }
    }

    fn test_let_statements_string() {
        let (input, expected_identifier, expected_value) = ("let foobar = y;", "foobar", "y");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(1, program.statements.len());
        let statement = &program.statements[0];
        test_let_statement(statement, expected_identifier);
        match statement {
            Statement::Let(_ident, expr) => {
                assert_eq!(test_identifier(expr, expected_value), true);
            }
            _ => {
                println!("Expected Let stmt, got somethingn else");
                assert!(false);
            }
        }
    }

    fn test_let_statement(s: &Statement, name: &str) {
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
        let (input, expected_value) = ("return foobar;", "foobar");

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(1, program.statements.len());
        for s in program.statements {
            match s {
                Statement::Return(expr) => {
                    test_identifier(&expr, expected_value);
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
    fn test_boolean_expression() {
        let input = "false;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::Boolean(b) if b == false => (),
                    _ => {
                        println!("Expected false, got something else.");
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
    fn test_if_expression() {
        let input = "if (x < y) { x }";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::IfExpression(condition, consequence, alt) => {
                        test_infix_expression(&condition.to_string(), "x", "<", "y");

                        assert_eq!(consequence.statements.len(), 1);
                        let s = &consequence.statements[0];
                        //if let Some(c) = consequence.statements.iter().next() {
                        match s {
                            Statement::Expression(ex) => {
                                if !test_identifier(ex, "x") {
                                    println!("identifier was not x");
                                    assert!(false);
                                }
                            }
                            _ => {
                                println!("Expected Statement expr, got something else.");
                                assert!(false);
                            }
                        }

                        if let Some(_) = alt {
                            println!("alt sttements was not None");
                            assert!(false);
                        }
                    }
                    _ => {
                        println!("Expected If condition, got something else.");
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
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::IfExpression(condition, consequence, alt) => {
                        test_infix_expression(&condition.to_string(), "x", "<", "y");

                        assert_eq!(consequence.statements.len(), 1);
                        let s = &consequence.statements[0];
                        //if let Some(c) = consequence.statements.iter().next() {
                        match s {
                            Statement::Expression(ex) => {
                                if !test_identifier(ex, "x") {
                                    println!("identifier was not x");
                                    assert!(false);
                                }
                            }
                            _ => {
                                println!("Expected Statement expr, got something else.");
                                assert!(false);
                            }
                        }

                        if let Some(alternate) = alt {
                            assert_eq!(alternate.statements.len(), 1);
                            let s = &alternate.statements[0];
                            match s {
                                Statement::Expression(ex) => {
                                    if !test_identifier(ex, "y") {
                                        println!("identifier was not x");
                                        assert!(false);
                                    }
                                }
                                _ => {
                                    println!("Expected Statement expr, got something else.");
                                    assert!(false);
                                }
                            }
                        } else {
                            println!("alt sttements was None");
                            assert!(false);
                        }
                    }
                    _ => {
                        println!("Expected If condition, got something else.");
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
    fn test_function_literal_parsing() {
        let input = "fn(x, y) { x + y; }";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::FunctionLiteral(parms, body) => {
                        assert_eq!(parms.len(), 2);
                        assert_eq!(parms[0], "x");
                        assert_eq!(parms[1], "y");
                        assert_eq!(body.statements.len(), 1);
                        let s = &body.statements[0];
                        match s {
                            Statement::Expression(ex) => {
                                test_infix_expression(&ex.to_string(), "x", "+", "y");
                            }
                            _ => {
                                println!("Expected Statement expr, got something else.");
                                assert!(false);
                            }
                        }
                    }
                    _ => {
                        println!("Expected function literal, got something else.");
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
    fn test_function_parameter_parsing() {
        let tests = vec![
            ("fn() {};", vec![]),
            ("fn(x) {};", vec!["x"]),
            ("fn(x, y, z) {};", vec!["x", "y", "z"]),
        ];

        for (input, expected_parms) in tests {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parser_errors(&parser);

            if let Some(statement) = program.statements.into_iter().next() {
                match statement {
                    Statement::Expression(expr) => match expr {
                        Expression::FunctionLiteral(parms, body) => {
                            assert_eq!(parms.len(), expected_parms.len());
                            let mut i = 0;
                            for p in expected_parms {
                                assert_eq!(p, parms[i]);
                                i += 1;
                            }
                        }
                        _ => {
                            println!("Expected function literal, got something else.");
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

    #[test]
    fn test_call_expression_parsing() {
        let input = "add(1, 2 * 3, 4 + 5);";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::CallExpression(function, args) => {
                        assert_eq!(test_identifier(&*function, "add"), true);
                        assert_eq!(args.len(), 3);
                        assert_eq!(test_integer_literal(&args[0], 1), true);
                        test_infix_expression_int(&args[1].to_string(), 2, "*", 3);
                        test_infix_expression_int(&args[2].to_string(), 4, "+", 5);
                    }
                    _ => {
                        println!("Expected call expression, got something else.");
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
    fn test_parsing_prefix_expressions_int() {
        let tests = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for (test_expr, test_operator, test_int) in tests {
            let lexer = Lexer::new(test_expr);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parser_errors(&parser);
            assert_eq!(program.statements.len(), 1);

            if let Some(statement) = program.statements.into_iter().next() {
                match statement {
                    Statement::Expression(expr) => match expr {
                        Expression::Prefix(operator, right) => {
                            if operator != test_operator {
                                println!("operator is not {}, got {}", test_operator, operator);
                                assert!(false);
                            }

                            if !test_integer_literal(&*right, test_int) {
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

    #[test]
    fn test_parsing_prefix_expressions_bool() {
        let tests = vec![("!true", "!", true), ("!false", "!", false)];

        for (test_expr, test_operator, test_bool) in tests {
            let lexer = Lexer::new(test_expr);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parser_errors(&parser);
            assert_eq!(program.statements.len(), 1);

            if let Some(statement) = program.statements.into_iter().next() {
                match statement {
                    Statement::Expression(expr) => match expr {
                        Expression::Prefix(operator, right) => {
                            if operator != test_operator {
                                println!("operator is not {}, got {}", test_operator, operator);
                                assert!(false);
                            }

                            if !test_bool_literal(&*right, test_bool) {
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

    fn test_integer_literal(expression: &Expression, value: i64) -> bool {
        match expression {
            Expression::IntegerLiteral(i) if i == &value => true,
            Expression::IntegerLiteral(i) => {
                println!("integer value not {}, got {} ", value, i);
                false
            }
            _ => {
                println!("Expression not integer literal");
                false
            }
        }
    }

    fn test_bool_literal(expression: &Expression, value: bool) -> bool {
        match expression {
            Expression::Boolean(b) if b == &value => true,
            _ => {
                println!("bool literal not correct");
                false
            }
        }
    }

    fn test_identifier(expression: &Expression, value: &str) -> bool {
        match expression {
            Expression::Ident(s) if s == value => true,
            Expression::Ident(s) => {
                println!("identifier value not {}, got {}", value, s);
                false
            }
            Expression::IntegerLiteral(i) => {
                println!("got an integer here for some reason: {}", i);
                false
            }
            _ => {
                println!("Expression not identifier");
                false
            }
        }
    }

    fn test_infix_expression_int(
        test_expr: &str,
        test_left: i64,
        test_operator: &str,
        test_right: i64,
    ) {
        let lexer = Lexer::new(test_expr);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::Infix(left, operator, right) => {
                        if !test_integer_literal(&*left, test_left) {
                            println!("left not equal to integer test");
                            assert!(false);
                        }

                        if operator != test_operator {
                            println!("operator is not {}, got {}", test_operator, operator);
                            assert!(false);
                        }

                        if !test_integer_literal(&*right, test_right) {
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

    fn test_infix_expression(
        test_expr: &str,
        test_left: &str,
        test_operator: &str,
        test_right: &str,
    ) {
        let lexer = Lexer::new(test_expr);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::Infix(left, operator, right) => {
                        if !test_identifier(&*left, test_left) {
                            println!("left not equal to identifier test");
                            assert!(false);
                        }

                        if operator != test_operator {
                            println!("operator is not {}, got {}", test_operator, operator);
                            assert!(false);
                        }

                        if !test_identifier(&*right, test_right) {
                            println!("right not equal to identifier test");
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

    #[test]
    fn test_parsing_infix_expressions() {
        let tests = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];
        for (test_expr, test_left, test_operator, test_right) in tests {
            test_infix_expression_int(test_expr, test_left, test_operator, test_right);
        }
    }

    #[test]
    fn test_parsing_infix_expressions_bool() {
        let tests = vec![
            ("true == true", true, "==", true),
            ("true != false", true, "!=", false),
            ("false == false", false, "==", false),
        ];
        for (test_expr, test_left, test_operator, test_right) in tests {
            let lexer = Lexer::new(test_expr);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parser_errors(&parser);
            assert_eq!(program.statements.len(), 1);

            if let Some(statement) = program.statements.into_iter().next() {
                match statement {
                    Statement::Expression(expr) => match expr {
                        Expression::Infix(left, operator, right) => {
                            if !test_bool_literal(&*left, test_left) {
                                println!("left not equal to bool test");
                                assert!(false);
                            }

                            if operator != test_operator {
                                println!("operator is not {}, got {}", test_operator, operator);
                                assert!(false);
                            }

                            if !test_bool_literal(&*right, test_right) {
                                println!("right not equal to bool test");
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

    #[test]
    fn test_operator_precedence() {
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("add(1, 2 * 3, 4 + 5)", "add(1, (2 * 3), (4 + 5))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
            (
                "a * [1, 2, 3, 4][b * c] * d",
                "((a * ([1, 2, 3, 4][(b * c)])) * d)",
            ),
            (
                "add(a * b[2], b[1], 2 * [1, 2][1])",
                "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
            ),
            ("return x;", "return x;"),
            ("return x return 2 * 3", "return x;return (2 * 3);"),
            ("return 2 * 4 + 5;", "return ((2 * 4) + 5);"),
            ("let x = 3", "let x = 3;"),
            ("let x = 3 + f * 8;", "let x = (3 + (f * 8));"),
        ];
        for (input, expected) in tests {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parser_errors(&parser);

            let actual = program.to_string();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_string_literal_expression() {
        let input = "\"hello world\"";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::StringLiteral(s) => {
                        if s != "hello world" {
                            println!("literal value not hello world, got={}", s);
                            assert!(false);
                        }
                    }
                    Expression::Ident(i) => {
                        dbg!("got ident and wanted string!");
                        assert!(false);
                    }
                    _ => {
                        println!("Expected string, got something else.");
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
    fn test_parsing_array_literals() {
        let input = "[1, 2 * 2, 3 + 3]";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::ArrayLiteral(elements) => {
                        assert_eq!(elements.len(), 3);
                        test_integer_literal(&elements[0], 1);
                        test_infix_expression_int(&elements[1].to_string(), 2, "*", 2);
                        test_infix_expression_int(&elements[2].to_string(), 3, "+", 3);
                    }
                    _ => {
                        println!("Expected array literal, got something else.");
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
    fn test_parsing_index_expression() {
        let input = "myArray[1 + 1]";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::IndexExpression(left, index) => {
                        if !test_identifier(&*left, "myArray") {
                            println!("left != myArray");
                            assert!(false);
                        }

                        test_infix_expression_int(&*index.to_string(), 1, "+", 1);
                    }
                    _ => {
                        println!("Expected index expression, got something else.");
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
    fn test_parsing_hash_literals_string_keys() {
        let input = "{\"one\": 1, \"two\": 2, \"three\": 3}";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::HashLiteral(pairs) => {
                        assert_eq!(pairs.len(), 3);
                        let expected = vec![
                            (
                                Expression::StringLiteral("one".to_string()),
                                Expression::IntegerLiteral(1),
                            ),
                            (
                                Expression::StringLiteral("two".to_string()),
                                Expression::IntegerLiteral(2),
                            ),
                            (
                                Expression::StringLiteral("three".to_string()),
                                Expression::IntegerLiteral(3),
                            ),
                        ];
                        for (k, v) in pairs {
                            match &k {
                                Expression::StringLiteral(_literal) => {
                                    match hash_get(&expected, &k) {
                                        Expression::IntegerLiteral(i) => {
                                            test_integer_literal(&v, i);
                                        }
                                        _ => {
                                            println!("expected int, got something else.");
                                            assert!(false);
                                        }
                                    }
                                }
                                _ => {
                                    println!("expected string, got something else.");
                                    assert!(false);
                                }
                            }
                        }
                    }
                    _ => {
                        println!("Expected hash literal, got something else.");
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

    fn hash_get(map: &Vec<(Expression, Expression)>, key: &Expression) -> Expression {
        for (k, v) in map {
            if k == key {
                return v.clone();
            }
        }
        Expression::None
    }

    #[test]
    fn test_parsing_empty_hash_literal() {
        let input = "{}";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::HashLiteral(pairs) => assert_eq!(pairs.len(), 0),
                    _ => {
                        println!("Expected hash literal, got something else.");
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
    fn test_parsing_hash_literals_with_expressions() {
        let input = "{\"one\": 0 + 1, \"two\": 10 - 8, \"three\": 15 / 5}";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        if let Some(statement) = program.statements.into_iter().next() {
            match statement {
                Statement::Expression(expr) => match expr {
                    Expression::HashLiteral(pairs) => {
                        assert_eq!(pairs.len(), 3);
                        let expected = vec![
                            (Expression::StringLiteral("one".to_string()), (0, "+", 1)),
                            (Expression::StringLiteral("two".to_string()), (10, "-", 8)),
                            (Expression::StringLiteral("three".to_string()), (15, "/", 5)),
                        ];
                        let mut i = 0;
                        for (k, v) in pairs {
                            let (_, e) = expected[i];
                            match &k {
                                Expression::StringLiteral(_literal) => {
                                    test_infix_expression_int(&v.to_string(), e.0, e.1, e.2);
                                }
                                _ => {
                                    println!("expected string literal for key.");
                                    assert!(false);
                                }
                            }
                            i += 1;
                        }
                    }
                    _ => {
                        println!("Expected hash literal, got something else.");
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

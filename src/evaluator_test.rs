#[cfg(test)]
mod tests {
    use crate::evaluator;
    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        return evaluator::eval(program);
    }
    //#[test]
    fn test_eval_integer_expression() {
        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
        }
    }

    fn test_integer_object(obj: Object, expected: i64) {
        match obj {
            Object::Integer(i) if i == expected => (),
            Object::Integer(i) => {
                println!("object has wrong int value. got={}, want={}", i, expected);
                assert!(false);
            }
            _ => {
                println!("object is not integer.");
                assert!(false);
            }
        }
    }

    #[test]
    fn test_boolean_expression() {
        let tests = vec![
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 == 1", true),
            ("1 == 2", false),
            ("1 != 1", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_boolean_object(evaluated, expected);
        }
    }

    fn test_boolean_object(obj: Object, expected: bool) {
        match obj {
            Object::Boolean(b) if b == expected => (),
            Object::Boolean(b) => {
                println!("object has wrong bool value. got={}, want={}", b, expected);
                assert!(false);
            }
            _ => {
                println!("object is not bool.");
                assert!(false);
            }
        }
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_boolean_object(evaluated, expected);
        }
    }

    #[test]
    fn test_if_else_expressions() {
        let tests = vec![
            ("if (true) { 10 }", Some(10)),
            ("if (false) { 10 }", None),
            ("if (1) { 10 } ", Some(10)),
            ("if (1 < 2) { 10 }", Some(10)),
            ("if (1 > 2) { 10 }", None),
            ("if (1 > 2) { 10 } else { 20 }", Some(20)),
            ("if (1 < 2) { 10 } else { 20 }", Some(10)),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            match expected {
                Some(i) => test_integer_object(evaluated, i),
                None => test_null_object(evaluated),
            }
        }
    }

    fn test_null_object(obj: Object) {
        if obj != Object::Null {
            println!("Expected Null");
            assert!(false);
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            ("if (10 > 1) { return 10; }", 10),
            (
                "if (10 > 1) {
                if (10 > 1) {
                  return 10;
                }
              
                return 1;
              }",
                10,
            ),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            dbg!(&evaluated);
            test_integer_object(evaluated, expected);
        }
    }
}

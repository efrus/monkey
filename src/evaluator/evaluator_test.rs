#[cfg(test)]
mod tests {
    use crate::environment::Environment;
    use crate::evaluator::evaluator;
    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let env = Rc::new(RefCell::new(Environment::new()));
        return evaluator::eval(program, env);
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
            test_integer_object(&evaluated, expected);
        }
    }

    fn test_integer_object(obj: &Object, expected: i64) {
        match obj {
            Object::Integer(i) if i == &expected => (),
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
                Some(i) => test_integer_object(&evaluated, i),
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
            test_integer_object(&evaluated, expected);
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) { 
                    if (10 > 1) {
                        return true + false; 
                    }
                    return 1;
                }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "identifier not found: foobar"),
            ("\"Hello\" - \"World", "unknown operator: STRING - STRING"),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            match evaluated {
                Object::Error(msg) => {
                    if msg != expected {
                        println!("wrong error message, expected={}, got={}", expected, msg);
                        assert!(false);
                    }
                }
                _ => {
                    println!("no error object reteurned");
                    assert!(false);
                }
            }
        }
    }

    #[test]
    fn test_let_statement() {
        let tests = vec![
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];

        for (input, expected) in tests {
            let obj = test_eval(input);
            dbg!(&obj);
            test_integer_object(&obj, expected);
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2; };";
        let evaluated = test_eval(input);
        match evaluated {
            Object::Function(parms, body, env) => {
                assert_eq!(parms.len(), 1);
                if parms[0].to_string() != "x" {
                    println!("paramater is not 'x'. got={}", parms[0].to_string());
                    assert!(false);
                }

                let expected_body = String::from("(x + 2)");

                if body.to_string() != expected_body {
                    println!("body is not {}. got={}", expected_body, body);
                    assert!(false);
                }
            }
            _ => {
                println!("object is not function");
                assert!(false);
            }
        }
    }

    #[test]
    fn test_function_application() {
        let tests = vec![
            ("let identity = fn(x) { x; }; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
            ("fn(x) { x; }(5)", 5),
        ];

        for (input, expected) in tests {
            let obj = test_eval(input);
            dbg!(&obj);
            test_integer_object(&obj, expected);
        }
    }

    #[test]
    fn test_string_literal() {
        let input = "\"Hello World!\"".to_string();
        test_string(input);
    }

    #[test]
    fn test_string_concatenation() {
        let input = "\"Hello\" + \" \" + \"World!\"".to_string();
        test_string(input);
    }

    fn test_string(input: String) {
        let evaluated = test_eval(&input);
        match evaluated {
            Object::String(s) => {
                if s != "Hello World!" {
                    println!("string has wrong value. got={}", s);
                    assert!(false);
                }
            }
            _ => {
                println!("object is not String. got={}", evaluated.obj_type());
                assert!(false);
            }
        }
    }

    #[test]
    fn test_builtin_functions() {
        enum Mixed {
            Text(String),
            Int(i64),
            Array(Vec<i64>),
        }
        let tests = vec![
            ("len(\"\")", Mixed::Int(0)),
            ("len(\"four\")", Mixed::Int(4)),
            ("len(\"hello world\")", Mixed::Int(11)),
            (
                "len(1)",
                Mixed::Text("argument to 'len' not supported.".to_string()),
            ),
            (
                "len(\"one\", \"two\")",
                Mixed::Text("wrong number of arguments. got=2, want=1".to_string()),
            ),
            ("len([1,2,3,4])", Mixed::Int(4)),
            ("first([1,2,3,4])", Mixed::Int(1)),
            ("last([1,2,3,4])", Mixed::Int(4)),
            ("rest([1,2,3,4])", Mixed::Array(vec![2, 3, 4])),
            ("push([1,2,3,4],5)", Mixed::Array(vec![1, 2, 3, 4, 5])),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(&input);
            match expected {
                Mixed::Int(i) => {
                    test_integer_object(&evaluated, i);
                }
                Mixed::Text(s) => match evaluated {
                    Object::Error(err) => {
                        if err != s {
                            println!("wrong error message. expected={}, got={}", s, err);
                            assert!(false);
                        }
                    }
                    _ => {
                        println!("object is not error.");
                        assert!(false);
                    }
                },
                Mixed::Array(vec) => match evaluated {
                    Object::Array(elements) => {
                        assert_eq!(elements.len(), vec.len());
                        let mut i = 0;
                        for item in vec {
                            test_integer_object(&elements[i], item);
                            i += 1;
                        }
                    }
                    _ => {
                        println!("object is not array.");
                        assert!(false);
                    }
                },
            }
        }
    }

    #[test]
    fn test_array_literals() {
        let input = "[1, 2 * 2, 3 + 3]";
        let evaluated = test_eval(input);
        match evaluated {
            Object::Array(elements) => {
                assert_eq!(elements.len(), 3);
                test_integer_object(&elements[0], 1);
                test_integer_object(&elements[1], 4);
                test_integer_object(&elements[2], 6);
            }
            _ => {
                println!("object is not array");
                assert!(false);
            }
        }
    }

    #[test]
    fn test_array_index_expressions() {
        let tests = vec![
            ("[1, 2, 3][0]", Some(1)),
            ("[1, 2, 3][1]", Some(2)),
            ("[1, 2, 3][2]", Some(3)),
            ("let i = 0; [1][i];", Some(1)),
            ("[1, 2, 3][1 + 1];", Some(3)),
            ("let myArray = [1, 2, 3]; myArray[2];", Some(3)),
            (
                "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
                Some(6),
            ),
            (
                "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
                Some(2),
            ),
            ("[1, 2, 3][3]", None),
            ("[1, 2, 3][-1]", None),
        ];
        for (input, expected) in tests {
            let evaluated = test_eval(input);
            match expected {
                Some(i) => test_integer_object(&evaluated, i),
                None => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator;
    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![("5", 5), ("10", 10)];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
        }
    }

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        return evaluator::eval(program);
    }

    fn test_integer_object(obj: Object, expected: i64) {
        match obj {
            Object::Integer(i) if i == expected => (),
            Object::Integer(i) => {
                println!("object has wrong int value. got={}, want={}", i, expected);
            }
            _ => {
                println!("object is not integer.");
                assert!(false);
            }
        }
    }
}

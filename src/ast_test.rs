#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Identifier, Program, Statement};
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn test_strings() {
        let output = "let myVar = anotherVar;";

        let expr = Expression::Ident("anotherVar".to_string());
        let let_statement = Statement::Let("myVar".to_string(), expr);
        let statements = vec![let_statement];
        let program = Program { statements };
        assert_eq!(output, program.to_string());
    }
}

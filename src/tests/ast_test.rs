#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Program, Statement};
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

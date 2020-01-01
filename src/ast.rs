use std::fmt;

pub type Identifier = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expression(Expression),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Ident(Identifier),
    IntegerLiteral(i64),
    Prefix(Identifier, Box<Expression>),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("");
        for s in &self.statements {
            output.push_str(&s.to_string());
        }
        write!(f, "{}", output)
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self {
            Statement::Let(ident, expr) => format!("let {} = {};", ident, expr),
            Statement::Return(expr) => format!("return {};", expr),
            Statement::Expression(expr) => expr.to_string(),
            _ => String::from(""),
        };
        write!(f, "{}", output)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self {
            Expression::Ident(ident) => ident.to_string(),
            Expression::IntegerLiteral(int) => int.to_string(),
            Expression::Prefix(operator, expr) => format!("({}{})", operator, expr),
            _ => String::from(""),
        };
        write!(f, "{}", output)
    }
}

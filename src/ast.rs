use std::fmt;

pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expression(Expression),
    Block(Vec<Statement>),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Ident(Identifier),
    IntegerLiteral(i64),
    Prefix(Identifier, Box<Expression>),
    Infix(Box<Expression>, Identifier, Box<Expression>),
    Boolean(bool),
    IfExpression(Box<Expression>, BlockStatement, Option<BlockStatement>),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
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
            Statement::Block(statements) => {
                let mut s = String::from("");
                for statement in statements {
                    s.push_str(&statement.to_string());
                }
                s
            }
            Statement::None => String::from(""),
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
            Expression::Infix(left, operator, right) => {
                format!("({} {} {})", left, operator, right)
            }
            Expression::Boolean(b) => format!("{}", b.to_string()),
            Expression::IfExpression(condition, consequence, alternative) => {
                let s = format!("if{} {}", condition.to_string(), consequence.to_string());
                if let Some(alt) = alternative {
                    format!("{}else {}", s, alt.to_string())
                } else {
                    s
                }
            }
            Expression::None => String::from(""),
        };
        write!(f, "{}", output)
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("");
        for statement in &self.statements {
            output.push_str(&statement.to_string());
        }
        write!(f, "{}", output)
    }
}

use std::fmt;

pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Ident(Identifier),
    IntegerLiteral(i64),
    StringLiteral(String),
    Prefix(Identifier, Box<Expression>),
    Infix(Box<Expression>, Identifier, Box<Expression>),
    Boolean(bool),
    IfExpression(Box<Expression>, BlockStatement, Option<BlockStatement>),
    FunctionLiteral(Vec<Identifier>, BlockStatement),
    CallExpression(Box<Expression>, Vec<Expression>),
    ArrayLiteral(Vec<Expression>),
    IndexExpression(Box<Expression>, Box<Expression>),
    HashLiteral(Vec<(Expression, Expression)>),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
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
        };
        write!(f, "{}", output)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self {
            Expression::Ident(ident) => ident.to_string(),
            Expression::IntegerLiteral(int) => int.to_string(),
            Expression::StringLiteral(s) => s.to_string(),
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
            Expression::CallExpression(function, arguments) => {
                let mut args = vec![];
                for arg in arguments {
                    args.push(arg.to_string());
                }

                format!("{}({})", function, args.join(", "))
            }
            Expression::FunctionLiteral(parms, body) => format!("{}({})", parms.join(", "), body),
            Expression::ArrayLiteral(elements) => {
                let mut e = vec![];
                for element in elements {
                    e.push(element.to_string());
                }
                format!("[{}]", e.join(", "))
            }
            Expression::IndexExpression(left, index) => {
                format!("({}[{}])", left.to_string(), index.to_string())
            }
            Expression::HashLiteral(pairs) => {
                let mut s = vec![];
                for (k, v) in pairs {
                    s.push(format!("{}:{}", k.to_string(), v.to_string()));
                }
                format!("{{{}}}", s.join(", "))
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

pub fn hash_put(
    map: &mut Vec<(Expression, Expression)>,
    key: Expression,
    value: Expression,
) -> Expression {
    map.push((key, value.clone()));
    value
}

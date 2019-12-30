use crate::token::Token;

pub type Identifier = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Let(Identifier, Expression),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

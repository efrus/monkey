#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,

    // Operators
    Assign,
    Asterisk,
    Bang,
    Minus,
    Plus,
    Slash,

    // Comparison
    Gt,
    Lt,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,

    // Grouping
    LBrace,
    LParen,
    RBrace,
    RParen,

    // Keywords
    Else,
    False,
    Function,
    If,
    Let,
    Return,
    True,
}

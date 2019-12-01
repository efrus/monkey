#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    //Eof,

    // Identifiers + literals
    Ident(String),
    Int(String),

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

impl Token {
    pub fn lookup_ident(ident: String) -> Token {
        match ident.as_str() {
            "else" => Token::Else,
            "false" => Token::False,
            "fn" => Token::Function,
            "if" => Token::If,
            "let" => Token::Let,
            "return" => Token::Return,
            "true" => Token::True,
            _ => Token::Ident(ident),
        }
    }
}

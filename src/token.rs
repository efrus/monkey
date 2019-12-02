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

/*impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self {
            Token::Ident(s) => s,
            Token::Int(s) => s,
            Token::Assign => "=",
            Token::Asterisk => "*",
            Token::Bang => "!",
            Token::Minus => "-",
            Token::Plus => "+",
            Token::Slash => "/",
            Token::Gt => ">",
            Token::Lt => "<",
            Token::Eq => "==",
            Token::NotEq => "!=",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::LBrace => "{",
            Token::LParen => "(",
            Token::RBrace => "}",
            Token::RParen => ")",
            Token::Else => "else",
            Token::False => "false",
            Token::Function => "fn",
            Token::If => "if",
            Token::Let => "let",
            Token::Return => "return",
            Token::True => "true",
            _ => "ILLEGAL",
        };
        write!(f, "{}", output)
    }
}*/

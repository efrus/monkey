pub type TokenType = String;

pub struct Token {
    Type: TokenType,
    Literal: String,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT"; // add, foobar, x, y, ...
pub const INT: &str = "INT"; // 1343456

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINU: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLAS: &str = "/";

pub const LT: &str = "<";
pub const GT: &str = ">";

pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALS: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";

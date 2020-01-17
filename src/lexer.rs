use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn read_identifier(&mut self, c: char) -> String {
        dbg!("read ident");
        let mut s = String::new();
        s.push(c);
        while let Some(&c) = self.peek_char() {
            if c.is_alphabetic() {
                s.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        s
    }

    fn read_number(&mut self, c: char) -> String {
        let mut num = String::new();
        num.push(c);
        while let Some(&c) = self.peek_char() {
            if c.is_digit(10) {
                num.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        num
    }

    fn read_string(&mut self) -> String {
        dbg!("read string");
        let mut s = String::new();
        while let Some(&c) = self.peek_char() {
            let ch = self.read_char().unwrap();
            if c == '"' || c == '\u{0}' || c == '\"' {
                break;
            } else if c != '\n' {
                s.push(ch);
            }
        }
        s
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() || c == '\n' {
                self.read_char().unwrap();
            } else {
                break;
            }
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(c) = self.read_char() {
            match c {
                '=' => {
                    if let Some(&'=') = self.peek_char() {
                        self.read_char();
                        Some(Token::Eq)
                    } else {
                        Some(Token::Assign)
                    }
                }
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '!' => {
                    if let Some(&'=') = self.peek_char() {
                        self.read_char();
                        Some(Token::NotEq)
                    } else {
                        Some(Token::Bang)
                    }
                }
                '/' => Some(Token::Slash),
                '*' => Some(Token::Asterisk),
                '<' => Some(Token::Lt),
                '>' => Some(Token::Gt),
                ';' => Some(Token::Semicolon),
                ',' => Some(Token::Comma),
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                '{' => Some(Token::LBrace),
                '}' => Some(Token::RBrace),
                '"' => {
                    let s = self.read_string();
                    Some(Token::String(s))
                }
                _ => {
                    if is_letter(c) {
                        Some(Token::lookup_ident(self.read_identifier(c)))
                    } else if c.is_digit(10) {
                        Some(Token::Int(self.read_number(c)))
                    } else {
                        Some(Token::Illegal)
                    }
                }
            }
        } else {
            None
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

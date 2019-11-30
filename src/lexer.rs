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

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
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
                '=' => Some(Token::Assign),
                ';' => Some(Token::Semicolon),
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                ',' => Some(Token::Comma),
                '+' => Some(Token::Plus),
                '{' => Some(Token::LBrace),
                '}' => Some(Token::RBrace),
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

/*impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}*/

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = "
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
";
        let mut tests = Vec::new();
        tests.push(Some(Token::Let));
        tests.push(Some(Token::Ident("five".to_string())));
        tests.push(Some(Token::Assign));
        tests.push(Some(Token::Int("5".to_string())));
        tests.push(Some(Token::Semicolon));
        tests.push(Some(Token::Let));
        tests.push(Some(Token::Ident("ten".to_string())));
        tests.push(Some(Token::Assign));
        tests.push(Some(Token::Int("10".to_string())));
        tests.push(Some(Token::Semicolon));
        tests.push(Some(Token::Let));
        tests.push(Some(Token::Ident("add".to_string())));
        tests.push(Some(Token::Assign));
        tests.push(Some(Token::Function));
        tests.push(Some(Token::LParen));
        tests.push(Some(Token::Ident("x".to_string())));
        tests.push(Some(Token::Comma));
        tests.push(Some(Token::Ident("y".to_string())));
        tests.push(Some(Token::RParen));
        tests.push(Some(Token::LBrace));
        tests.push(Some(Token::Ident("x".to_string())));
        tests.push(Some(Token::Plus));
        tests.push(Some(Token::Ident("y".to_string())));
        tests.push(Some(Token::Semicolon));
        tests.push(Some(Token::RBrace));
        tests.push(Some(Token::Semicolon));
        tests.push(Some(Token::Let));
        tests.push(Some(Token::Ident("result".to_string())));
        tests.push(Some(Token::Assign));
        tests.push(Some(Token::Ident("add".to_string())));
        tests.push(Some(Token::LParen));
        tests.push(Some(Token::Ident("five".to_string())));
        tests.push(Some(Token::Comma));
        tests.push(Some(Token::Ident("ten".to_string())));
        tests.push(Some(Token::RParen));
        tests.push(Some(Token::Semicolon));
        tests.push(None);

        let mut l = Lexer::new(input);

        for test in tests {
            let tok = l.next_token();
            println!("expected {:?}, lexed {:?} ", test, tok);
            assert_eq!(tok, test);
        }
    }
}

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

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn next_token(&mut self) -> Option<Token> {
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
                _ => Some(Token::Eof),
            }
        } else {
            None
        }
    }
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
        let input = "=+(){},;";

        let mut tests = Vec::new();
        tests.push(Some(Token::Assign));
        tests.push(Some(Token::Plus));
        tests.push(Some(Token::LParen));
        tests.push(Some(Token::RParen));
        tests.push(Some(Token::LBrace));
        tests.push(Some(Token::RBrace));
        tests.push(Some(Token::Comma));
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

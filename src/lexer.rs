use crate::token;
use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer {
    input: Peekable<Chars>,
}

impl Lexer {
    pub fn New(input: String) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn NextToken(&mut self) -> Option<Token> {
        if let Some(c) = self.read_char() {
            match c {
                '=' => new_token(token::ASSIGN, c),
                ';' => new_token(token::SEMICOLON, c),
                '(' => new_token(token::LPAREN, c),
                ')' => new_token(token::RPAREN, c),
                ',' => new_token(token::COMMA, c),
                '+' => new_token(token::PLUS, c),
                '{' => new_token(token::LBRACE, c),
                '}' => new_token(token::RBRACE, c),
                _ => new_token(token::EOF, ""),
            }
        } else {
            None
        }
    }
}

fn new_token(token_type: token::TokenType, c: char) -> Option<Token> {
    Some(Token {
        Type: token_type,
        Literal: c.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::token;

    fn TestNextToken() {
        let input = "=+(){},;";

        let mut tests: Vec<(&str, &str)> = Vec::new();
        tests.push((token::ASSIGN, "="));
        tests.push((token::PLUS, "+"));
        tests.push((token::LPAREN, "("));
        tests.push((token::RPAREN, ")"));
        tests.push((token::LBRACE, "{"));
        tests.push((token::RBRACE, "}"));
        tests.push((token::COMMA, ","));
        tests.push((token::SEMICOLON, ";"));
        tests.push((token::EOF, ""));

        let l = Lexer::new(input);

        for t in tests {
            let tok = l.NextToken();

            assert_eq!(tok.Type, t.0);
            assert_eq!(tok.Literal, t.1);
        }
    }
}

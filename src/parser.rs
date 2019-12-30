use crate::ast::{Expression, Identifier, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            lexer,
            current_token: None,
            peek_token: None,
        };

        p.next_token();
        p.next_token();

        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = vec![];
        let mut not_eof = true;

        while not_eof {
            match &self.current_token {
                Some(_token) => {
                    if let Some(statement) = self.parse_statement() {
                        statements.push(statement);
                    }
                    self.next_token();
                }
                None => not_eof = false,
            };
        }
        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if let Some(token) = &self.current_token {
            let statement = match token {
                Token::Let => self.parse_let_statement(),
                _ => None,
            };
            return statement;
        }
        None
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if let Some(Token::Ident(identifier)) = self.peek_token.clone() {
            self.next_token();

            if !self.expect_peek(Token::Assign) {
                return None;
            }

            while !self.current_token_is(&Token::Semicolon) {
                self.next_token();
            }

            let expression = Expression::None;
            return Some(Statement::Let(identifier, expression));
        }
        None
    }

    fn current_token_is(&self, t: &Token) -> bool {
        match &self.current_token {
            Some(token) if token == t => true,
            _ => false,
        }
    }

    fn peek_token_is(&self, t: &Token) -> bool {
        match &self.peek_token {
            Some(token) if token == t => true,
            _ => false,
        }
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Identifier, Program, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    //use crate::token::Token;

    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(3, program.statements.len());

        let tests = ["x", "y", "foobar"];

        let mut statements = program.statements.into_iter();
        for test in tests.iter() {
            let s = statements.next().unwrap();
            test_let_statement(s, test);
        }
    }

    fn test_let_statement(s: Statement, name: &str) {
        match s {
            Statement::Let(ident, _) => {
                assert_eq!(ident, name);
            }
            _ => assert!(false),
        };
    }
}

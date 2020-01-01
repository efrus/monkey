use crate::ast::{Expression, Identifier, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

pub enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            lexer,
            current_token: None,
            peek_token: None,
            errors: vec![],
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
                Token::Return => self.parse_return_statement(),
                _ => self.parse_expression_statement(),
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

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        while !self.current_token_is(&Token::Semicolon) {
            self.next_token();
        }

        let expression = Expression::None;
        return Some(Statement::Return(expression));
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::LOWEST);
        let statement = Statement::Expression(expr);

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        return Some(statement);
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Expression {
        let expr = self.prefix_parse();
        if expr == Expression::None {
            if let Some(token) = &self.current_token {
                self.no_prefix_parse_error(token.clone());
            }
        }
        expr
    }

    fn parse_identifier(&self) -> Expression {
        match &self.current_token {
            Some(Token::Ident(ident)) => Expression::Ident(ident.to_string()),
            _ => Expression::None,
        }
    }

    fn parse_integer_literal(&self) -> Expression {
        match &self.current_token {
            Some(Token::Int(int_string)) => {
                if let Ok(int) = int_string.parse::<i64>() {
                    Expression::IntegerLiteral(int)
                } else {
                    Expression::None
                }
            }
            _ => Expression::None,
        }
    }

    fn parse_prefix_expression(&mut self) -> Expression {
        let operator = self.get_token();
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX);
        return Expression::Prefix(operator.to_string(), Box::new(right));
    }

    //convenience method to retrieve token
    fn get_token(&self) -> Token {
        match &self.current_token {
            Some(token) => token.clone(),
            None => Token::Illegal,
        }
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
        self.peek_error(&t);
        false
    }

    fn peek_error(&mut self, t: &Token) {
        let token = match &self.peek_token {
            Some(val) => val,
            None => &Token::Illegal,
        };
        let msg = format!("expected next token to be {}, got {} instead", t, token);
        self.errors.push(msg);
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn prefix_parse(&mut self) -> Expression {
        match &self.current_token {
            Some(Token::Ident(_)) => self.parse_identifier(),
            Some(Token::Int(_)) => self.parse_integer_literal(),
            Some(Token::Bang) => self.parse_prefix_expression(),
            Some(Token::Minus) => self.parse_prefix_expression(),
            _ => Expression::None,
        }
    }

    fn no_prefix_parse_error(&mut self, token: Token) {
        let msg = format!("no prefix parse function for {} found", token);
        self.errors.push(msg);
    }

    pub fn infix_parse(&self, left_expression: Expression) -> Expression {
        Expression::None
    }
}

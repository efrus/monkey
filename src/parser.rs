use crate::ast::{BlockStatement, Expression, Identifier, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
}

fn precedences(token: Token) -> Precedence {
    match token {
        Token::Eq => Precedence::EQUALS,
        Token::NotEq => Precedence::EQUALS,
        Token::Lt => Precedence::LESSGREATER,
        Token::Gt => Precedence::LESSGREATER,
        Token::Plus => Precedence::SUM,
        Token::Minus => Precedence::SUM,
        Token::Slash => Precedence::PRODUCT,
        Token::Asterisk => Precedence::PRODUCT,
        Token::LParen => Precedence::CALL,
        _ => Precedence::LOWEST,
    }
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
        let prefix = self.prefix_parse();
        //println!("Prefix:{}", prefix);
        if prefix == Expression::None {
            println!("prefix == Expression::None");
            let token = &self.get_current_token();
            self.no_prefix_parse_error(token.clone());
            return Expression::None;
        }
        let mut peek_precedence = self.peek_precedence();
        let mut peek_token_is_semi_colon = self.peek_token_is(&Token::Semicolon);
        let mut left_expr = prefix.clone();
        while !peek_token_is_semi_colon && precedence < peek_precedence {
            if left_expr == Expression::None {
                return left_expr;
            }
            self.next_token();
            left_expr = self.infix_parse(Box::new(left_expr));
            peek_precedence = self.peek_precedence();
            peek_token_is_semi_colon = self.peek_token_is(&Token::Semicolon);
        }
        left_expr
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
        let operator = self.get_current_token().to_string();
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX);
        Expression::Prefix(operator, Box::new(right))
    }

    fn parse_infix_expression(&mut self, left: Box<Expression>) -> Expression {
        let operator = self.get_current_token().to_string();
        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence);
        Expression::Infix(left, operator, Box::new(right))
    }

    fn parse_call_expression(&mut self, function: Box<Expression>) -> Expression {
        let args = self.parse_call_arguments();
        Expression::CallExpression(function, args)
    }

    fn parse_call_arguments(&mut self) -> Vec<Expression> {
        let mut args = vec![];
        if self.peek_token_is(&Token::RParen) {
            self.next_token();
            return args;
        }

        self.next_token();
        args.push(self.parse_expression(Precedence::LOWEST));

        while self.peek_token_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::LOWEST));
        }

        if !self.expect_peek(Token::RParen) {
            return vec![];
        }
        args
    }

    fn parse_boolean(&mut self) -> Expression {
        Expression::Boolean(self.current_token_is(&Token::True))
    }

    fn parse_grouped_expression(&mut self) -> Expression {
        self.next_token();
        let exp = self.parse_expression(Precedence::LOWEST);
        if !self.expect_peek(Token::RParen) {
            return Expression::None;
        }
        exp
    }

    fn parse_if_expression(&mut self) -> Expression {
        if !self.expect_peek(Token::LParen) {
            return Expression::None;
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST);
        if !self.expect_peek(Token::RParen) {
            return Expression::None;
        }

        if !self.expect_peek(Token::LBrace) {
            return Expression::None;
        }
        let consequence = self.parse_block_statement();
        let mut alt = None;
        if self.peek_token_is(&Token::Else) {
            self.next_token();

            if !self.expect_peek(Token::LBrace) {
                return Expression::None;
            }

            alt = Some(self.parse_block_statement());
        }
        Expression::IfExpression(Box::new(condition), consequence, alt)
    }

    fn parse_function_literal(&mut self) -> Expression {
        if !self.expect_peek(Token::LParen) {
            return Expression::None;
        }

        let parms = self.parse_function_parameters();

        if !self.expect_peek(Token::LBrace) {
            return Expression::None;
        }

        let body = self.parse_block_statement();

        Expression::FunctionLiteral(parms, body)
    }

    fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        let mut identifiers = vec![];

        if self.peek_token_is(&Token::RParen) {
            self.next_token();
            return identifiers;
        }

        self.next_token();

        identifiers.push(self.get_current_token().to_string());

        while self.peek_token_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(self.get_current_token().to_string());
        }

        if !self.expect_peek(Token::RParen) {
            return vec![];
        }
        identifiers
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut statements = vec![];
        self.next_token();

        let mut eof = false;

        while !self.current_token_is(&Token::RBrace) && !eof {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
            if self.current_token == None {
                eof = true;
            }
        }

        BlockStatement { statements }
    }

    //convenience method to retrieve token
    fn get_current_token(&self) -> Token {
        match &self.current_token {
            Some(token) => token.clone(),
            None => Token::Illegal,
        }
    }

    //convenience method to retrieve token
    fn get_peek_token(&self) -> Token {
        match &self.peek_token {
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
            Some(Token::True) => self.parse_boolean(),
            Some(Token::False) => self.parse_boolean(),
            Some(Token::LParen) => self.parse_grouped_expression(),
            Some(Token::If) => self.parse_if_expression(),
            Some(Token::Function) => self.parse_function_literal(),
            _ => Expression::None,
        }
    }

    fn no_prefix_parse_error(&mut self, token: Token) {
        let msg = format!("no prefix parse function for {} found", token);
        self.errors.push(msg);
    }

    pub fn infix_parse(&mut self, left_expression: Box<Expression>) -> Expression {
        match &self.current_token {
            Some(Token::Plus) => self.parse_infix_expression(left_expression),
            Some(Token::Minus) => self.parse_infix_expression(left_expression),
            Some(Token::Slash) => self.parse_infix_expression(left_expression),
            Some(Token::Asterisk) => self.parse_infix_expression(left_expression),
            Some(Token::Eq) => self.parse_infix_expression(left_expression),
            Some(Token::NotEq) => self.parse_infix_expression(left_expression),
            Some(Token::Lt) => self.parse_infix_expression(left_expression),
            Some(Token::Gt) => self.parse_infix_expression(left_expression),
            Some(Token::LParen) => self.parse_call_expression(left_expression),
            _ => Expression::None,
        }
    }

    fn peek_precedence(&self) -> Precedence {
        precedences(self.get_peek_token())
    }

    fn current_precedence(&self) -> Precedence {
        precedences(self.get_current_token())
    }
}

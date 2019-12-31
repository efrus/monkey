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
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;

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

        tests.push(Some(Token::Bang));
        tests.push(Some(Token::Minus));
        tests.push(Some(Token::Slash));
        tests.push(Some(Token::Asterisk));
        tests.push(Some(Token::Int("5".to_string())));
        tests.push(Some(Token::Semicolon));
        tests.push(Some(Token::Int("5".to_string())));
        tests.push(Some(Token::Lt));
        tests.push(Some(Token::Int("10".to_string())));
        tests.push(Some(Token::Gt));
        tests.push(Some(Token::Int("5".to_string())));
        tests.push(Some(Token::Semicolon));
        tests.push(Some(Token::If));
        tests.push(Some(Token::LParen));
        tests.push(Some(Token::Int("5".to_string())));
        tests.push(Some(Token::Lt));
        tests.push(Some(Token::Int("10".to_string())));
        tests.push(Some(Token::RParen));
        tests.push(Some(Token::LBrace));
        tests.push(Some(Token::Return));
        tests.push(Some(Token::True));
        tests.push(Some(Token::Semicolon));
        tests.push(Some(Token::RBrace));
        tests.push(Some(Token::Else));
        tests.push(Some(Token::LBrace));
        tests.push(Some(Token::Return));
        tests.push(Some(Token::False));
        tests.push(Some(Token::Semicolon));
        tests.push(Some(Token::RBrace));

        tests.push(Some(Token::Int("10".to_string())));
        tests.push(Some(Token::Eq));
        tests.push(Some(Token::Int("10".to_string())));
        tests.push(Some(Token::Semicolon));
        tests.push(Some(Token::Int("10".to_string())));
        tests.push(Some(Token::NotEq));
        tests.push(Some(Token::Int("9".to_string())));
        tests.push(Some(Token::Semicolon));
        tests.push(None);
        let mut l = Lexer::new(input);

        for test in tests {
            let tok = l.next_token();
            //println!("expected {:?}, lexed {:?} ", test, tok);
            assert_eq!(tok, test);
        }
    }
}

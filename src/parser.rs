use std::mem::take;

use crate::ast::{Expression, Program, Statement};
use crate::lexer::{Lexer, LimiterToken, Token};

struct Parser {
    lexer: Lexer,
    curr_token: Option<Token>,
    peek_token: Option<Token>,
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Self {
            lexer,
            curr_token: None,
            peek_token: None,
        };
        p.next_token();
        p.next_token();

        return p;
    }

    pub fn parse_program(&mut self) -> Program {
        let mut prog = Program::new();

        while self.curr_token.is_some() {
            let stmt = self.parse_statement();
            if let Some(s) = stmt {
                prog.statements.push(s);
            }
            self.next_token();
        }

        return prog;
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        return match &self.curr_token {
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };
    }

    fn next_token(&mut self) {
        self.curr_token = take(&mut self.peek_token);
        self.peek_token = self.lexer.next();
    }

    fn curr_token_is(&self, other: Token) -> bool {
        match &self.curr_token {
            Some(t) => *t == other,
            None => false,
        }
    }

    fn peek_token_is(&self, other: Token) -> bool {
        match &self.peek_token {
            Some(t) => *t == other,
            None => false,
        }
    }

    fn expect_peek(&mut self, other: Token) {
        match &self.peek_token {
            Some(t) => {
                assert_eq!(*t, other);
                self.next_token()
            }
            None => (),
        }
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        while !self.curr_token_is(Token::Limiter(LimiterToken::Semicolon)) {
            self.next_token();
        }
        // FIXME: this should be a proper expression
        let fixme = Expression::Identifier(Token::Assign);
        return Some(Statement::Return(fixme));
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let identifier: Expression = match &self.peek_token {
            Some(Token::Identifier(s)) => Expression::Identifier(Token::Identifier(s.clone())),
            _ => return None,
        };
        self.next_token(); // current is the identifier and peek the equal sign

        self.expect_peek(Token::Assign); // current is the equal sign

        while !self.curr_token_is(Token::Limiter(LimiterToken::Semicolon)) {
            self.next_token();
        }

        // FIXME: this should be a proper expression
        let fixme = Expression::Identifier(Token::Assign);
        return Some(Statement::Let(identifier, fixme));
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        // we know a token exists at the moment since we are in the middle of parsing a statment
        let express = self.parse_expression(Precedence::Lowest);
        return match express {
            Some(e) => Some(Statement::Expression(e)),
            None => None,
        }

    }

    fn parse_expression(&self, precedence: Precedence) -> Option<Expression> {
        // we know a token exists at the moment since we are in the middle of parsing a statment
        let token = self.curr_token.as_ref()?;
        return prefix_parsing_fn(&token);
    }

}

fn prefix_parsing_fn(token: &Token) -> Option<Expression> {
    match token {
        Token::Identifier(ident) => Some(Expression::Identifier(Token::Identifier(ident.clone()))),
        _ => None,
    }
}

// fn get_infix_parse_fn(token: Token) -> impl Fn(Expression) -> Option<Expression> {
//     todo!();
// }

#[test]
fn let_statement_test() {
    let input = "let five = 5;
    let ten = 10;
    let foobar = 8080;
    ";
    let lex = Lexer::from(String::from(input));
    let mut pars = Parser::new(lex);
    let prog = pars.parse_program();
    assert_eq!(prog.statements.len(), 3);

    let tests: [Statement; 3] = [
        Statement::Let(
            Expression::Identifier(Token::Identifier(String::from("five"))),
            Expression::Identifier(Token::Assign),
        ),
        Statement::Let(
            Expression::Identifier(Token::Identifier(String::from("ten"))),
            Expression::Identifier(Token::Assign),
        ),
        Statement::Let(
            Expression::Identifier(Token::Identifier(String::from("foobar"))),
            Expression::Identifier(Token::Assign),
        ),
    ];
    prog.statements
        .into_iter()
        .zip(tests.into_iter())
        .map(|(stmt, test_stmt)| {
            assert_eq!(stmt, test_stmt);
        })
        .for_each(drop);
}

#[test]
fn return_statement_test() {
    let input = "return 5;
    return 10;
    return 8080;
    ";
    let lex = Lexer::from(String::from(input));
    let mut pars = Parser::new(lex);
    let prog = pars.parse_program();
    assert_eq!(prog.statements.len(), 3);

    let tests: [Statement; 3] = [
        Statement::Return(Expression::Identifier(Token::Assign)),
        Statement::Return(Expression::Identifier(Token::Assign)),
        Statement::Return(Expression::Identifier(Token::Assign)),
    ];
    prog.statements
        .into_iter()
        .zip(tests.into_iter())
        .map(|(stmt, test_stmt)| {
            assert_eq!(stmt, test_stmt);
        })
        .for_each(drop);
}

#[test]
fn identifier_expression_test() {
    let input = "foobar;";

    let lex = Lexer::from(String::from(input));
    let mut pars = Parser::new(lex);
    let prog = pars.parse_program();
    assert_eq!(prog.statements.len(), 1);

    let s = prog.statements[0].clone();
    assert!(match s {
        Statement::Expression(e) => {
            if let Expression::Identifier(Token::Identifier(s)) = e {
                assert_eq!(s, String::from("foobar"));
                true
            } else { false }
        },
        _ => false,
    });
}

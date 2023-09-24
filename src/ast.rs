use crate::lexer::Token;

trait Node {
    // fn token_literal(&self) -> &Token;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Let(Expression, Expression),
    Return(Expression),
    Expression(Expression),
}

impl Statement {
    fn statement_node(&self) {
        todo!()
    }
}

impl Node for Statement {
    // fn token_literal(&self) -> &Token {
    //     return match self {
    //         Statement::Let(t, _, _) => t,
    //     };
    // }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Identifier(Token),
}

impl Expression {
    fn expression_node(&self) {
        todo!()
    }
}

impl Node for Expression {
    // fn token_literal(&self) -> &Token {
    //     return match self {
    //         Expression::Identifier(t) => t,
    //     };
    // }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        return Self { statements: vec![] };
    }
}

/*
Parser, turns tokens into AST
*/

use crate::lexer::{Token, TokenKind};

// An expression
#[derive(Debug, PartialEq)]
pub enum Expression {
    Num(u64),                               // A number
    Var(String),                            // A variable name
    Add(Box<Expression>, Box<Expression>),  // Left plus right
}

// A statement is just a expression with an action on it
#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign { name: String, value: Expression },         // Set variable values
    Loop { count: Expression, body: Vec<Statement> },   // Runs a list of statements a set number of times
}

// Errors 
#[derive(Debug, PartialEq)]
pub enum ParserError {
    Unexpected { expected: String, found: TokenKind, line: usize, column: usize },
}

// Parser that take tokens and turns into AST
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {

    // Create a new parser with the tokens and start at the first token
    pub fn new(tokens: Vec<Token>) -> Self {
        todo!()
    }

    // Looks at the current token without moving
    fn peek(&self) -> &Token {
        todo!()
    }

    // Moves on to the next token
    fn advance(&mut self) {
        todo!()
    }

    // BNF

    // <program> ::= <statement>* EOF
    pub fn parse_program(&mut self) -> Result<Vec<Statement>, ParserError> {
        todo!()
    }

    // <statement> ::= <assign> | <loop> | <write>
    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }

    // <assign> ::= "$" ID "=" <expression> "$"
    fn parse_assign(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }

    // <loop> ::= "\begin" "{" "loop" "}" "{" <expression> "}" <statement>* "\end" "{" "loop" "}"
    fn parse_loop(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }

    // <expression> ::= <term> ( "+" <term> )*
    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }

    // <term> ::= NUM | ID
    fn parse_term(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }
}


// Tests that lex a small string and check that the right tree comes out
#[cfg(test)]
mod tests {
    use super::*;

}

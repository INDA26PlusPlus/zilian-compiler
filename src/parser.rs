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
    Set { name: String, value: Expression },         // Set variable values
    Loop { count: Expression, content: Vec<Statement> },   // Runs a list of statements a set number of times
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
        Self {
            tokens,
            index: 0,
        }
    }

    // Looks at the current token without moving
    fn peek(&self) -> &Token {
        // Checks that we wont go out of range
        if self.index < self.tokens.len() {
            &self.tokens[self.index]
        }
        else {
            // Eof last
            &self.tokens[self.tokens.len() - 1]
        }
    }

    // Moves on to the next token
    fn advance(&mut self) {
        // Never move past Eof
        if self.peek().kind != TokenKind::Eof {
            self.index += 1;
        }
    }

    // Sends error if we got wrong token
    fn error(&self, expected: &str) -> ParserError {
        let token = self.peek();
        ParserError::Unexpected {
            expected: String::from(expected),
            found: token.kind.clone(),
            line: token.line,
            column: token.column,
        }
    }

    // Checks if what we got was expected
    fn expect(&mut self, expected_kind: TokenKind) -> Result<(), ParserError> {
        if self.peek().kind == expected_kind {
            self.advance();
            Ok(())
        }
        else {
            Err(self.error(&format!("{:?}", expected_kind)))
        }
    }

    // Checks if the contents of ID was expected
    fn expect_id(&mut self) -> Result<String, ParserError> {
        let id_name = match &self.peek().kind {
            TokenKind::Id(id_name) => id_name.clone(),
            _ => return Err(self.error("a variable name")),
        };
        self.advance();
        Ok(id_name)
    }

    // BNF

    // <program> ::= <statement> EOF
    pub fn parse_program(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();

        // Loops through statements until the file ends
        loop {
            if self.peek().kind == TokenKind::Eof {
                break;
            }

            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(statements)
    }

    // <statement> ::= <set> | <loop>
    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        // Determine the kind of statement
        match self.peek().kind {
            TokenKind::Dollar => self.parse_set(),
            TokenKind::Begin => self.parse_loop(),

            _ => {
                let token = self.peek();
                Err(ParserError::Unexpected {
                    expected: String::from("$ or \\begin"),
                    found: token.kind.clone(),
                    line: token.line,
                    column: token.column,
                })
            }
        }
    }

    // <set> ::= "$" ID "=" <expression> "$"
    fn parse_set(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenKind::Dollar)?;
        let name = self.expect_id()?;
        self.expect(TokenKind::Set)?;
        let value = self.parse_expression()?;
        self.expect(TokenKind::Dollar)?;

        Ok(Statement::Set { name, value })
    }

    // <loop> ::= "\begin" "{" "loop" "}" "{" <expression> "}" <statement>* "\end" "{" "loop" "}"
    fn parse_loop(&mut self) -> Result<Statement, ParserError> {

        // \begin{loop}{count}
        self.expect(TokenKind::Begin)?;
        self.expect(TokenKind::LeftBrace)?;
        self.expect(TokenKind::Loop)?;
        self.expect(TokenKind::RightBrace)?;
        self.expect(TokenKind::LeftBrace)?;
        let count = self.parse_expression()?;
        self.expect(TokenKind::RightBrace)?;

        // Parses statments on repeat untill end
        let mut loop_content = Vec::new();
        loop {

            // If we reached end of loop
            if self.peek().kind == TokenKind::End {
                break;
            }

            let statement = self.parse_statement()?;
            loop_content.push(statement);
        }

        // \end{loop}
        self.expect(TokenKind::End)?;
        self.expect(TokenKind::LeftBrace)?;
        self.expect(TokenKind::Loop)?;
        self.expect(TokenKind::RightBrace)?;

        Ok(Statement::Loop { count, content: loop_content })
    }

    // <expression> ::= <term> "+" <term>
    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        // First term
        let mut left_term = self.parse_term()?;

        loop {
            // Add all terms with plus to the left term
            if self.peek().kind == TokenKind::Plus {
                self.advance();
                let right_term = self.parse_term()?;
                left_term = Expression::Add(Box::new(left_term), Box::new(right_term));
            }
            else {
                break;
            }
        }

        Ok(left_term)
    }

    // <term> ::= NUM | ID
    fn parse_term(&mut self) -> Result<Expression, ParserError> {
        let token = self.peek();

        // Sorts expression into numbers or variables
        let expression = match &token.kind {
            TokenKind::Num(number) => Expression::Num(*number),
            TokenKind::Id(name) => Expression::Var(name.clone()),
            _ => {
                return Err(ParserError::Unexpected {
                    expected: String::from("a number or variable"),
                    found: token.kind.clone(),
                    line: token.line,
                    column: token.column,
                })
            }
        };

        self.advance();
        Ok(expression)
    }
}


// Tests that lex a small string and check that the right tree comes out
#[cfg(test)]
mod tests {
    use super::*;

}

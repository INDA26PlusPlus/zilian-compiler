/*
Lexer file used for handling input and turning it into tokens
*/

// Errors if something fails within the lexing process.
#[derive(Debug, PartialEq)]
pub enum LexerError {

}

// The types of tokens that our program can perceive
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Num(u64),
    Id(String),
    If(String),
    Loop(String),
    Eql(String),
}

// What makes up a token
// Line and column mainly for debugging
#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

// Lexer that takes input and "tokenizes"
// i.e divides chars into sections of tokens
pub struct Lexer {
    input: Vec<char>,
    index: usize,
    current_line: usize,
    current_column: usize,
}

impl Lexer {

    // Create new lexer with an input and starting line = 1
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            current_line: 1,
            current_column: 1,
        }
    }

    // Takes the saved input and "converts it" to a vector of Tokens
    // Will loop over the entire input and go through each next_token
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        todo!()
    }

    // For lexing a single token
    // Decides token type and how long it is
    fn next_token(&mut self) -> Result<Token, LexerError> {
        todo!()
    }

    // If token starts with a number
    // Send back token with value or error if failed
    fn read_number(&mut self) -> Result<TokenKind, LexerError> {
        todo!()
    }

    // If token starts with ascii char
    // Reads the rest of the word and sorts it further:
    // if, loop, function, write
    fn read_word(&mut self) -> TokenKind {
        todo!()
    }

    // Spaces can't be contained within a token so it must have ended
    fn whitespace(&mut self) {
        todo!()
    }

    // Looks ahead onto the next char index 
    fn peeking(&self) -> Option<char> {
        todo!()
    }

    // Unlike peeking, doesn't look but moves to the next char index
    fn advance(&mut self) -> Option<char> {
        todo!()
    }
}


// To test some strings to see if they get sorted into the right token vectors
#[cfg(test)]
mod tests {
    use super::*;

}
/*
Lexer file used for handling input and turning it into tokens
*/

// Errors if something fails within the lexing process.
#[derive(Debug, PartialEq)]
pub enum LexerError {
    NumberOverflow {number: String, line: usize, column: usize},
    UnknownCommand {text: String, line: usize, column: usize},
    UnknownChar {character: char, line: usize, column: usize}
}

// The types of tokens that our program can perceive
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Num(u64),                   // A unsigned 64 bit int
    Id(String),                 // A string that doesn't fit any other TokenKind
    Begin, End,                 // \begin and \end
    Loop,                       // For loops {loop}
    Dollar,                     // $
    Set, Plus,                  // =, +
    Equal,                      // ==
    LeftBrace, RightBrace,      // { }
    Eof,                        // End of file, aka we're done, nothing more to parse!
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
            index: 0,
            current_line: 1,
            current_column: 1,
        }
    }

    // Takes the saved input and "converts it" to a vector of Tokens
    // Will loop over the entire input and go through each next_token
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut lexed_tokens = Vec::new();

        loop {
            // Goes to next token
            let token = self.next_token()?;

            // Checks if its the final token
            if  token.kind == TokenKind::Eof {
                lexed_tokens.push(token);
                break;
            }

            lexed_tokens.push(token);            
        }

        Ok(lexed_tokens)
    }

    // For lexing a single token
    // Decides token type and how long it is
    fn next_token(&mut self) -> Result<Token, LexerError> {
        // Clear whitespaces and comments
        self.skip();

        // Check where the token starts
        let line = self.current_line;
        let column = self.current_column;

        // Determine the kind of token so we can categorize it
        let kind = match self.peeking() {
            // Nothing to peek into
            None => TokenKind::Eof,

            // Multi character token
            Some(character) if character.is_ascii_digit() => self.read_number()?,
            Some(character) if character.is_ascii_alphabetic() => self.read_word(),

            //  Command like \begin, \end
            Some('\\') => {
                self.advance();
                match self.read_word() {
                    TokenKind::Begin => TokenKind::Begin,
                    TokenKind::End => TokenKind::End,
                    _ => {
                        return Err(LexerError::UnknownCommand{text: String::from("?"), line, column})
                    }
                }
            }

            Some('$') => {
                self.advance(); 
                TokenKind::Dollar
            }
            Some('+') => {
                self.advance(); 
                TokenKind::Plus
            }
            Some('{') => {
                self.advance(); 
                TokenKind::LeftBrace
            }
            Some('}') => {
                self.advance(); 
                TokenKind::RightBrace
            }

            Some('=') => {
                if self.peek_next() == Some('=') {
                    self.advance();
                    self.advance();
                    TokenKind::Equal
                }
                else {
                    self.advance();
                    TokenKind::Set
                }
            }

            Some(character) => {
                return Err(LexerError::UnknownChar {character, line, column})
            }
        };

        Ok(Token {kind, line, column})
    }

    // If token starts with a number
    // Since we peek to decide what type of read method, we can assume self.peeking().is_some() == true
    // Send back token with value or error if failed
    fn read_number(&mut self) -> Result<TokenKind, LexerError> {

        // The token we want to read
        let mut number_string = String::new();
        
        loop {
            let current_char = match self.peeking() {
                Some(character) => character,
                _ => break
            };

            
            if current_char.is_ascii_digit() {
                number_string.push(current_char);
                self.advance();
            }
            else {
                break;
            }
        }

        match number_string.parse::<u64>() {
            Ok(number) => Ok(TokenKind::Num(number)),
            Err(_) => Err(LexerError::NumberOverflow { number: number_string, line: self.current_line, column: self.current_column })
        }
    }

    // If token starts with ascii char
    // Reads the rest of the word and sorts it further:
    // if, loop, function
    fn read_word(&mut self) -> TokenKind {
        // The token we want to read
        let mut word_string = String::new();
        
        loop {
            let current_char = match self.peeking() {
                Some(character) => character,
                _ => break
            };

            
            if current_char.is_ascii_alphanumeric() {
                word_string.push(current_char);
                self.advance();
            }
            else {
                break;
            }
        }

        match word_string.as_str() {
            "loop"  => TokenKind::Loop,
            "begin" => TokenKind::Begin,
            "end"   => TokenKind::End,
            _       => TokenKind::Id(word_string)
        }
            
    }

    // Skips whitespaces and comments "//" between tokens
    fn skip(&mut self) {
        
        loop {
            match self.peeking() {

                // Whitespace
                Some(character) if character.is_whitespace() => {
                    self.advance();
                }

                // Comments
                Some('/') if self.peek_next() == Some('/') => {
                    loop {
                        match self.peeking() {
                            Some('\n') | None => break,
                            Some(_) => {self.advance();}
                        }
                    }
                }

                // New token, stop skipping
                _ => return,
            }
        }
    }

    // Looks ahead onto the next char index 
    fn peeking(&self) -> Option<char> {
        // Checks that we wont go out of range
        if self.index < self.input.len() {
            Some(self.input[self.index])
        }
        else {
            None
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.index + 1 < self.input.len() {
            Some(self.input[self.index + 1])
        }
        else {
            None
        }
    }

    // Returns the current character and moves ahead
    fn advance(&mut self) -> Option<char> {
        if self.peeking().is_some() {
            let current_character = self.peeking().unwrap();
            self.index += 1;
            if current_character == '\n' {
                self.current_line += 1;
                self.current_column = 1;
            }
            else {
                self.current_column += 1;
            }
            Some(current_character)
        }
        else {
            None
        }
    }
}


// To test some strings to see if they get sorted into the right token vectors
#[cfg(test)]
mod tests {
    use super::*;

}
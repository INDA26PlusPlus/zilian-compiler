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
    use TokenKind::*;

    // To make custom token
    fn token(kind: TokenKind, line: usize, column: usize) -> Token {
        Token { kind, line, column }
    }

    // Checks if the test-code matches what we want
    #[test]
    fn testcode() {
        let input = 

r"// How many times to run the loop
// aka the fibonacci number you want
$ runTime = 0 $

// Set starting values
$ a = 0 $
$ b = 1 $

// Run through the fibonacci sequence a set number of times
\begin{loop}{runTime}

// Count up fibonacci numbers
$ temp = a$
$ a = b$
$ b = a + temp $

\end{loop}";

        let expected = vec![
            token(Dollar, 3, 1), token(Id("runTime".to_string()), 3, 3), token(Set, 3, 11), token(Num(0), 3, 13), token(Dollar, 3, 15),
            token(Dollar, 6, 1), token(Id("a".to_string()), 6, 3), token(Set, 6, 5), token(Num(0), 6, 7), token(Dollar, 6, 9),
            token(Dollar, 7, 1), token(Id("b".to_string()), 7, 3), token(Set, 7, 5), token(Num(1), 7, 7), token(Dollar, 7, 9),
            token(Begin, 10, 1), token(LeftBrace, 10, 7), token(Loop, 10, 8), token(RightBrace, 10, 12),
            token(LeftBrace, 10, 13), token(Id("runTime".to_string()), 10, 14), token(RightBrace, 10, 21),
            token(Dollar, 13, 1), token(Id("temp".to_string()), 13, 3), token(Set, 13, 8), token(Id("a".to_string()), 13, 10), token(Dollar, 13, 11),
            token(Dollar, 14, 1), token(Id("a".to_string()), 14, 3), token(Set, 14, 5), token(Id("b".to_string()), 14, 7), token(Dollar, 14, 8),
            token(Dollar, 15, 1), token(Id("b".to_string()), 15, 3), token(Set, 15, 5), token(Id("a".to_string()), 15, 7), token(Plus, 15, 9),
            token(Id("temp".to_string()), 15, 11), token(Dollar, 15, 16),
            token(End, 17, 1), token(LeftBrace, 17, 5), token(Loop, 17, 6), token(RightBrace, 17, 10),
            token(Eof, 17, 11),
        ];

        let result = Lexer::new(input.to_string()).tokenize();
        assert_eq!(result, Ok(expected));
    }
}
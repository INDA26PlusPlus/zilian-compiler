use std::fs;
use std::error::Error;
mod lexer;
mod parser;

// Takes contents of testcode and runs it through the lexer code
fn main() -> Result<(), Box<dyn Error>> {
    let message: String = fs::read_to_string("src/testcode.txt")?;
    println!("{}", message);
    let mut lexing_engine = lexer::Lexer::new(message);
    let tokens = lexing_engine.tokenize();
    println!("{:?}", tokens);

    println!("\n\n");
    
    match tokens {
        Ok(tokens) => {
            let mut parser_engine = parser::Parser::new(tokens);
            println!("{:?}", parser_engine.parse_program());
        }
        Err(error) => println!("Error {:?}", error)
    }
    Ok(())
}
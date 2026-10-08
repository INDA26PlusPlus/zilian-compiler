use std::fs;
use std::error::Error;
mod lexer;

// Takes contents of testcode and runs it through the lexer code
fn main() -> Result<(), Box<dyn Error>> {
    let message: String = fs::read_to_string("src/testcode.txt")?;
    println!("{}", message);
    Ok(())
}
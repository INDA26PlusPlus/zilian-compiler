use std::fs;
use std::error::Error;
use inkwell::context::Context;

use crate::codegen::CodeGen;
mod lexer;
mod parser;
mod codegen;

// Takes contents of testcode and runs it through the lexer code
fn main() -> Result<(), Box<dyn Error>> {

    // Copy test from testcode.txt
    let message: String = fs::read_to_string("src/testcode.txt")?;

    // Print the code
    //println!("{}\n", message);
    
    // Make a new lexer with the testcode as the input
    let mut lexing_engine = lexer::Lexer::new(message);
    let tokens = lexing_engine.tokenize();
    
    // Checks that tokens are ok, if not will return error
    match tokens {
        Ok(tokens) => {
            let mut parser_engine = parser::Parser::new(tokens);
            let parsed_program = parser_engine.parse_program();

            match parsed_program {
                Ok(program) => {
                    //println!("{:?}", program);

                    // Codegen goes here
                    let context = Context::create();
                    let mut compiler = codegen::CodeGen::new(&context, "test");

                    // Compiles and outputs IR to file
                    match compiler.compile(program) {
                        Ok(_) => {
                            let _ = compiler._print_ir_to_file("src/out.ll");
                            println!("{}", CodeGen::jit_run(&compiler).unwrap());
                        },
                        Err(e) => println!("{:?}", e),
                    }
                }
                Err(error) => println!("Parsing Error {:?}", error)
            }
        }
        Err(error) => println!("Lexing Error {:?}", error)
    }
    Ok(())
}
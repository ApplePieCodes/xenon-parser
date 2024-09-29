use std::{fs::File, io::{self, Read}};

use lexer::LexerIter;
use parser::Parser;

mod lexer;
mod parser;
mod node;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?; // Open the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Read the file's content into a string
    Ok(contents)
}

fn main() {
    let code = read_file("examples/kernel.xe");
    
    match code {
        Ok(contents) => {
            let tokens: LexerIter = lexer::lex_tokens(&contents);  // Lex tokens from the source code

            let tokvec: Vec<_> = tokens.collect(); // Collect tokens into a Vec
            let mut parser = Parser::new(&tokvec); // Pass the slice of tokens to the parser
            
            let program = parser.parse();  // Parse the tokens (assuming `parse` is implemented)
            println!("{:#?}", program);
        },
        Err(_e) => logex::log("File Could Not Be Read", logex::LogType::FatalError),
    }
}

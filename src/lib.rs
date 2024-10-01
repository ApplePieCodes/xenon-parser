use lexer::LexerIter;
use node::Program;
use parser::Parser;

mod lexer;
mod parser;
pub mod node;

pub fn parse(code: &str) -> Program {
    let tokens: LexerIter = lexer::lex_tokens(code);  // Lex tokens from the source code

    let tokvec: Vec<_> = tokens.collect(); // Collect tokens into a Vec
    let mut parser = Parser::new(&tokvec); // Pass the slice of tokens to the parser
    
    let program = parser.parse();  // Parse the tokens (assuming `parse` is implemented)
    return program
}
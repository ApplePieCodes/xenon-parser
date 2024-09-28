use lexer::Lexer;
use parser::Parser;

mod token;
mod lexer;
mod parser;
mod node;
mod newlexer;

fn main() {
    let code = "print(\"Hello World\");\nreturn 12 * 4;";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.lex();
    for token in tokens.clone() {
        println!("{}", token);
    }
    let mut parser = Parser::new(tokens);
}

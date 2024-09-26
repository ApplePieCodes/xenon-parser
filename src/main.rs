use lexer::Lexer;

mod token;
mod lexer;

fn main() {
    let code = "return \"Hello World\";";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.lex();
    for token in tokens {
        println!("{}", token);
    }
}

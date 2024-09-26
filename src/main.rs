use lexer::Lexer;

mod token;
mod lexer;

fn main() {
    let code = "print(\"Hello World\");\nreturn 12 * 4;";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.lex();
    for token in tokens {
        println!("{}", token);
    }
}

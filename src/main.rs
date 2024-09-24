use lexer::lex;

mod token;
mod lexer;

fn main() {
    let code = "return 12;";
    let tokens = lex(code);
    for token in tokens {
        println!("{}", token);
    }
}

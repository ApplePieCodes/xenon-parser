mod lexer;

fn main() {
    let code = "print(\"Hello World\");";
    let tokens = lexer::lex_tokens(code);
    for token in tokens {
        println!("{:#?}", token);
    }
}

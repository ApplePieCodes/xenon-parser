use crate::token::{Token, TokenType};

const KEYWORDS: [&str; 3] = ["use", "void", "return"];

/// Lexical analyzer (lexer) that takes a string input and returns a vector of tokens.
pub fn lex(c: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let code: Vec<char> = c.chars().collect();
    let mut buffer: String = String::from("");
    let mut i = 0;
    let mut line: u128 = 1;

    while i < code.len() {
        if code[i].is_alphabetic() || code[i] == '_' { // Keywords and Identifiers
            buffer.push(code[i]);
            i += 1;
            while i < code.len() && (code[i].is_alphanumeric() || code[i] == '_' || code[i] == '.') {
                buffer.push(code[i]);
                i += 1;
            }
            let mut item: Token = Token::new();
            if KEYWORDS.contains(&buffer.as_str()) {
                item.ttype = TokenType::Keyword;
            } else {
                item.ttype = TokenType::Identifier;
            }
            item.value = buffer.clone();
            item.line = line;
            tokens.push(item);
            buffer.clear();
        }
        else if code[i].is_numeric() { // Int and Float Lits
            buffer.push(code[i]);
            i+=1;
            while i < code.len() && (code[i].is_numeric() || code[i] == '.') {
                buffer.push(code[i]);
                i+=1;
            }
            let mut tok: Token = Token::new();
            if buffer.contains('.') {
                tok.ttype = TokenType::FloatLiteral;
            }
            else {
                tok.ttype = TokenType::IntegerLiteral;
            }
            tok.value = buffer.clone();
            tok.line  = line;
            tokens.push(tok);
        }
        else if code[i].is_whitespace() {
            if code[i] == '\n' {
                line+=1;
            }
            i+=1;
        }
        else {
            let mut tok: Token = Token::new();
            tok.line = line;
            if code[i] == ';' {
                tok.ttype = TokenType::Symbol;
                tok.value = String::from(";");
            }
            tokens.push(tok);

            i+=1;
        }
    }

    return tokens;
}
use crate::token::{Token, TokenType};

const KEYWORDS: [&str; 3] = ["use", "void", "return"];

pub struct Lexer {
    code: Vec<char>,
    i: u128
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        Lexer { i: 0, code: code.chars().collect()}
    }

    pub fn lex(&mut self) -> Vec<Token>{
        let mut tokens: Vec<Token> = vec![];
        let mut buffer: String = String::new();
        let length = self.code.len() as u128;
        let mut line: u128 = 1;

        while self.i < length {
            if self.peek(0).is_alphabetic() || self.peek(0) == '_' {
                buffer.push(self.peek(0));
                self.i+=1;
                while self.i < length && (self.peek(0).is_alphanumeric() || self.peek(0) == '.' || self.peek(0) == '_') {
                    buffer.push(self.peek(0));
                    self.i+=1;
                }

                let mut tok = Token::new();

                if KEYWORDS.contains(&buffer.as_str()) {
                    tok.ttype = TokenType::Keyword;
                }
                else {
                    tok.ttype = TokenType::Identifier
                }

                tok.value = buffer.clone();
                tok.line = line;
                tokens.push(tok);
            }
            else if self.peek(0).is_numeric() { // IntegerLiteral and FloatLiteral
                buffer.push(self.peek(0));
                self.i+=1;
                while self.i < length && (self.peek(0).is_numeric() || self.peek(0) == '.') {
                    buffer.push(self.peek(0));
                    self.i+=1;
                }
                let mut tok = Token::new();
                if buffer.contains('.') {
                    tok.ttype = TokenType::FloatLiteral;
                }
                else {
                    tok.ttype = TokenType::IntegerLiteral;
                }
                tok.value = buffer.to_string();
                tok.line = line;
                tokens.push(tok);
            }
            else if self.peek(0).is_whitespace() {
                if self.peek(0) == '\n' {
                    line+=1;
                }
                self.i+=1;
            }
            else if self.peek(0) == '"' {
                self.i+=1;
                while self.peek(0) != '"' {
                    buffer.push(self.peek(0));
                    self.i+=1;
                }
                self.i+=1;
                let mut tok = Token::new();
                tok.ttype = TokenType::StringLiteral;
                tok.value = buffer.clone();
                tok.line = line;
                tokens.push(tok);
            }
            else {
                let mut tok = Token::new();

                if self.peek(0) == ';' {
                    tok.ttype = TokenType::Symbol;
                    tok.value = ";".to_string();
                }
                else if self.peek(0) == '+' {
                    tok.ttype = TokenType::Operator;
                    if self.peek(1) == '=' {
                        tok.value = "+=".to_string();
                        self.i+=1;
                    }
                    else if self.peek(1) == '+' {
                        tok.value = "++".to_string();
                        self.i+=1;
                    }
                    else {
                        tok.value = "+".to_string();
                    }
                }
                else if self.peek(0) == '-' {
                    tok.ttype = TokenType::Operator;
                    if self.peek(1) == '=' {
                        tok.value = "-=".to_string();
                        self.i+=1;
                    }
                    else if self.peek(1) == '-' {
                        tok.value = "--".to_string();
                        self.i+=1;
                    }
                    else {
                        tok.value = "-".to_string();
                    }
                }
                else if self.peek(0) == '*' {
                    tok.ttype = TokenType::Operator;
                    if self.peek(1) == '=' {
                        tok.value = "*=".to_string();
                        self.i+=1;
                    }
                    else {
                        tok.value = "*".to_string();
                    }
                }
                else if self.peek(0) == '/' {
                    tok.ttype = TokenType::Operator;
                    if self.peek(1) == '=' {
                        tok.value = "/=".to_string();
                        self.i+=1;
                    }
                    else {
                        tok.value = "/".to_string();
                    }
                }
                else if self.peek(0) == '(' {
                    tok.ttype = TokenType::Symbol;
                    tok.value = "(".to_string();
                }
                else if self.peek(0) == ')' {
                    tok.ttype = TokenType::Symbol;
                    tok.value = ")".to_string();
                }
                else if self.peek(0) == '[' {
                    tok.ttype = TokenType::Symbol;
                    tok.value = "[".to_string();
                }
                else if self.peek(0) == ']' {
                    tok.ttype = TokenType::Symbol;
                    tok.value = "]".to_string();
                }
                else if self.peek(0) == '{' {
                    tok.ttype = TokenType::Symbol;
                    tok.value = "{".to_string();
                }
                else if self.peek(0) == '}' {
                    tok.ttype = TokenType::Symbol;
                    tok.value = "}".to_string();
                }
                else if self.peek(0) == '=' {
                    tok.ttype = TokenType::Operator;
                    if self.peek(1) == '=' {
                        tok.value = "==".to_string();
                        self.i+=1;
                    }
                    else {
                        tok.value = "=".to_string();
                    }
                }
                else if self.peek(0) == '<' {
                    tok.ttype = TokenType::Operator;
                    if self.peek(1) == '=' {
                        tok.value = "<=".to_string();
                        self.i+=1;
                    }
                    else {
                        tok.value = "<".to_string();
                    }
                }
                else if self.peek(0) == '>' {
                    tok.ttype = TokenType::Operator;
                    if self.peek(1) == '=' {
                        tok.value = ">=".to_string();
                        self.i+=1;
                    }
                    else {
                        tok.value = ">".to_string();
                    }
                }
                
                tok.line = line;
                self.i+=1;
                tokens.push(tok);
            }
            
            buffer.clear();
        }


        return tokens;
    }

    pub fn peek(&mut self, offset: i128) -> char {
        let index = self.i as i128 + offset;
        if index >= 0 && index < self.code.len() as i128 {
            return self.code[index as usize];
        }
        else {
            return ' ';
        }
    }
}
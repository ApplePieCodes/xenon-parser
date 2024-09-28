use logex::log;

use crate::{lexer::{SourceLoc, Token, TokenKind}, node::{Program, UseStatement}};

pub struct Parser<'a> {
    tokens: &'a [Token<'a>], // Changed to a slice for better flexibility
    i: usize,
}

impl<'a> Parser<'a> {
    // Constructor to take a slice instead of a reference to Vec
    pub fn new(toks: &'a Vec<Token<'a>>) -> Self { // Pass in tokens with lifetime 'a
        Parser { tokens: toks, i: 0 }
    }


    pub fn parse(&mut self) -> Program {
        let mut program = Program::new();
        while self.i < self.tokens.len() {
            if self.match_token(0, TokenKind::UseKw) {
                program.use_statements.push(self.parse_use_statement());
            }
            else {
                self.i+=1;
            }
        }
        program
    }

    pub fn parse_use_statement(&mut self) -> UseStatement {
        let mut statement = UseStatement::new();
        self.i+=1;
        if ! self.match_token(0, TokenKind::Identifier) {
            let mut buffer = "".to_string();
            buffer.push_str("at line ");
            buffer.push_str(self.peek(-1).location.line.to_string().as_str());
            buffer.push_str(": Expected Identifier after Use Keyword");
            log(&buffer, logex::LogType::FatalError);
        }
        statement.name = self.peek(0).literal.to_string();
        self.i+=1;
        if ! self.match_token(0, TokenKind::Semicolon) {
            let mut buffer = "".to_string();
            buffer.push_str("at line ");
            buffer.push_str(self.peek(-1).location.line.to_string().as_str());
            buffer.push_str(": Expected Semicolon after Identifier");
            log(&buffer, logex::LogType::FatalError);
        }
        self.i+=1;
        return statement;
    }

    fn match_token(&mut self, offset: isize, t: TokenKind) -> bool {
        if self.peek(offset).kind == t {
            true
        }
        else {
            false
        }
    }

    fn match_token_exact(&mut self, offset: isize, t: TokenKind, value: &str) -> bool {
        if self.peek(0).kind == t && self.peek(0).literal == value {
            true
        }
        else {
            false
        }
    }
    fn peek(&self, offset: isize) -> Token {
        let index = self.i as isize + offset;
        if index >= 0 && index < self.tokens.len() as isize {
            self.tokens[index as usize].clone()
        } else {
            Token {kind: TokenKind::Whitespace, location: SourceLoc {line: 0, start: 0, end: 0}, literal: ""}
        }
    }
}

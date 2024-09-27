use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    i: u128,
    tokenlen: u128
}

impl Parser {
    pub fn new(toks: Vec<Token>) -> Self {
        let clone = toks.clone();
        Parser {tokens: toks, i: 0, tokenlen: clone.len() as u128}
    }

    pub fn parse(&mut self) {
        while self.i < self.tokenlen {
            
        }
    }

    pub fn match_token(&mut self, offset: i128, t: TokenType) -> bool {
        if self.peek(offset).ttype == t {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn match_token_exact(&mut self, offset: i128, t: TokenType, value: String) -> bool {
        if self.peek(offset).ttype == t && self.peek(offset).value == value {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn peek(&mut self, offset: i128) -> Token {
        let index = self.i as i128 + offset;
        let indexusize = index as usize;
        if index >= 0 && index < self.tokenlen as i128 {
            return self.tokens[indexusize].clone();
        }
        else {
            return Token {ttype: TokenType::Unknown, value: "".to_string(), line: 0};
        }
    }
}
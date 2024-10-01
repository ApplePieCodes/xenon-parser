use crate::{lexer::{SourceLoc, Token, TokenKind}, node::{Definition, FunctionDefinition, Namespace, Program}};

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
        let mut program: Program = Program::new();

        while self.i < self.tokens.len() {
            if self.match_token(0, TokenKind::NamespaceKw) {
                program.namespaces.push(self.parse_namespace());
            }
        }

        return program;
    }

    fn parse_namespace(&mut self) -> Namespace {
        let mut namespace: Namespace = Namespace::new();

        // Skip namespace keyword

        self.i+=1;

        namespace.name = self.peek(0).literal.to_string();

        self.i+=1;

        // Skip opening curly

        self.i+=1;

        while ! self.match_token(0, TokenKind::CloseCurly) {
            namespace.definitions.push(self.parse_definition());
        }

        return namespace;
    }

    fn parse_definition(&mut self) -> Definition {
        if self.match_token(0, TokenKind::PubKw) {
            if self.match_token(1, TokenKind::ClassKw) {
                //Parse Class
            }
            else if self.match_token(1, TokenKind::Identifier) && self.match_token(2, TokenKind::Identifier) {
                if self.match_token(3, TokenKind::OpenParen) {
                    //Parse FunctionDefinition
                }
                else {
                    //Parse VariableDefinition
                }
            }
        }
        else {
            if self.match_token(0, TokenKind::ClassKw) {
                //Parse Class
            }
            else if self.match_token(0, TokenKind::Identifier) && self.match_token(1, TokenKind::Identifier) {
                if self.match_token(2, TokenKind::OpenParen) {
                    //Parse FunctionDefinition
                }
                else {
                    //Parse VariableDefinition
                }
            }
        }
        return  Definition::FunctionDefinition(FunctionDefinition::new());
    }

    fn match_token(&mut self, offset: isize, t: TokenKind) -> bool {
        if self.peek(offset).kind == t {
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

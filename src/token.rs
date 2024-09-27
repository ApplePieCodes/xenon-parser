use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum TokenType {
    Identifier,
    Keyword,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    Operator,
    Symbol,
    Unknown
}

#[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
    pub line: u128
}

impl Token {
    pub fn new() -> Self{
        let tok = Token {
            ttype: TokenType::Unknown,
            value: String::new(),
            line: 0
        };
        return tok;
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "Token at line {} is ", self.line);
        match self.ttype {
            TokenType::Identifier => write!(f, "Identifier({})", self.value),
            TokenType::Keyword => write!(f, "Keyword({})", self.value),
            TokenType::IntegerLiteral => write!(f, "IntegerLiteral({})", self.value),
            TokenType::FloatLiteral => write!(f, "FloatLiteral({})", self.value),
            TokenType::StringLiteral => write!(f, "StringLiteral(\"{}\")", self.value),
            TokenType::Operator => write!(f, "Operator({})", self.value),
            TokenType::Symbol => write!(f, "Symbol({})", self.value),
            TokenType::Unknown => write!(f, "Unknown"),
        }
    }
}
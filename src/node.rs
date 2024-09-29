#[derive(Debug)]
pub struct Program {
    pub use_statements: Vec<UseStatement>,
    pub function_defs: Vec<FunctionDefinition>
}
impl Program {
    pub fn new() -> Self {
        Program {use_statements: Vec::new(), function_defs: Vec::new()}
    }
}

#[derive(Debug)]
pub struct UseStatement {
    pub name: String
}
impl UseStatement {
    pub fn new() -> Self {
        UseStatement {name: "".to_string()}
    }
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub public: bool,
    pub rtype: String,
    pub name: String,
    pub arguements: Vec<Statement>,
    pub actions: Vec<Statement>
}
impl FunctionDefinition {
    pub fn new() -> Self {
        FunctionDefinition { public: false, rtype: "void".to_string(), name: "unknown".to_string(), arguements: vec![], actions: vec![]}
    }
}

#[derive(Debug)]
pub struct VariableDefinition {
    pub name: String,
    pub dtype: String,
    pub value: Expression
}

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String
}
#[derive(Debug)]
pub struct CharLiteral {
    pub value: String
}
#[derive(Debug)]
pub struct IntegerLiteral {
    pub value: String
}
#[derive(Debug)]
pub struct FloatLiteral {
    pub value: String
}
#[derive(Debug)]
pub struct BooleanLiteral {
    pub value: String
}

#[derive(Debug)]
pub enum Term {
    StringLiteral(StringLiteral),
    CharLiteral(CharLiteral),
    IntegerLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
    BooleanLiteral(BooleanLiteral)
}

#[derive(Debug)]
pub enum Expression {
    BinaryOperation(BinaryOperation)
}

#[derive(Debug)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>
}

#[derive(Debug)]
pub enum Statement {
    VariableDefinition(VariableDefinition)
}
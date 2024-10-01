#[derive(Debug)]
pub struct Program {
    pub namespaces: Vec<Namespace>
}
impl Program {
    pub fn new() -> Self {
        Program {namespaces: vec![]}
    }
}

#[derive(Debug)]
pub struct Namespace {
    pub name: String,
    pub definitions: Vec<Definition>
}
impl Namespace {
    pub fn new() -> Self {
        Namespace {name: "".to_string(), definitions: vec![]}
    }
}

#[derive(Debug)]
// Definitions
pub enum Definition {
    FunctionDefinition(FunctionDefinition),
    ClassDefinition(ClassDefinition),
    VariableDefinition(VariableDefinition)
}

#[derive(Debug)]
pub struct FunctionDefinition {
    ftype: String,
    name: String,
    arguements: Vec<VariableDefinition>,
    statements: Vec<Statement>,
}
impl FunctionDefinition {
    pub fn new() -> Self {
        FunctionDefinition {ftype: "".to_string(), name: "".to_string(), arguements: vec![], statements: vec![]}
    }
}

#[derive(Debug)]
pub struct ClassDefinition {
    name: String,
    definitions: Vec<Definition>
}

#[derive(Debug)]
//Statement and Definition
pub struct VariableDefinition {
    dtype: String,
    name: String,
    value: Expression
}

#[derive(Debug)]
// Statements
pub enum Statement {
    VariableDefinition(VariableDefinition),
    FunctionCall(FunctionCall),
    VariableRedefinition(VariableRedefinition)
}

#[derive(Debug)]
pub enum Expression {
    BinaryOperation(BinaryOperation),
    Term(Term)
}

#[derive(Debug)]
pub enum Term {
    FunctionCall(FunctionCall),
    IntegerLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
    StringLiteral(StringLiteral),
    CharLiteral(CharLiteral),
    BooleanLiteral(BooleanLiteral)
}

#[derive(Debug)]
pub struct IntegerLiteral {
    value: String
}

#[derive(Debug)]
pub struct FloatLiteral {
    value: String
}

#[derive(Debug)]
pub struct StringLiteral {
    value: String
}

#[derive(Debug)]
pub struct CharLiteral {
    value: String
}

#[derive(Debug)]
pub struct BooleanLiteral {
    value: String
}

#[derive(Debug)]
pub struct FunctionCall {
    name: String,
    arguements: Vec<Expression>
}

#[derive(Debug)]
pub struct VariableRedefinition {
    name: String,
    value: Expression
}

#[derive(Debug)]
pub struct BinaryOperation {
    left: Box<Expression>,
    op: String,
    right: Box<Expression>,
}
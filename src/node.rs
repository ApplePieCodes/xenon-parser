// Abstracts
pub enum Expression {
    Term(Term),
    BinaryOperation(Box<BinaryOperation>)
}
pub enum Term {
    StringLiteral(StringLiteral),
    IntegerLiteral(IntegerLiteral),
    FunctionCall(FunctionCall)
}
pub enum Statement {
    FunctionCall(FunctionCall),
    VariableDefinition(VariableDefinition),
    UseStatement(UseStatement)
}

pub struct UseStatement {
    pub name: String

    
}

// Core Classes
pub struct BinaryOperation {
    left: Box<Expression>,
    op: String,
    right: Box<Expression>
}
pub struct FunctionCall {
    name: String,
    arguements: Vec<Expression>
}
pub struct VariableDefinition {
    dtype: String,
    name: String,
    value: Option<Expression>,
}
pub struct VariableRedefinition {
    name: String,
    value: Expression
}

// Literals
pub struct StringLiteral {
    value: String
}
pub struct IntegerLiteral {
    value: String
}
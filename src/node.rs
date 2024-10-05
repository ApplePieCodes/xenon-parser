use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub namespaces: Vec<Namespace>,
    pub usestatements: Vec<UseStatement>
}
impl Program {
    pub fn new() -> Self {
        Program {namespaces: vec![], usestatements: vec![]}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UseStatement {
    pub name: String
}
impl UseStatement {
    pub fn new() -> Self {
        UseStatement {name: "".to_string()}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Namespace {
    pub name: String,
    pub definitions: Vec<Definition>
}
impl Namespace {
    pub fn new() -> Self {
        Namespace {name: "".to_string(), definitions: vec![]}
    }
}

#[derive(Debug, Serialize, Deserialize)]
// Definitions
pub enum Definition {
    FunctionDefinition(FunctionDefinition),
    ClassDefinition(ClassDefinition),
    VariableDefinition(VariableDefinition)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub public: bool,
    pub ftype: String,
    pub name: String,
    pub arguements: Vec<VariableDefinition>,
    pub statements: Vec<Statement>,
}
impl FunctionDefinition {
    pub fn new() -> Self {
        FunctionDefinition {public: false, ftype: "".to_string(), name: "".to_string(), arguements: vec![], statements: vec![]}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassDefinition {
    pub public: bool,
    pub name: String,
    pub definitions: Vec<Definition>
}
impl ClassDefinition {
    pub fn new() -> Self {
        ClassDefinition {name: "".to_string(), definitions: vec![], public: false}
    }
}
#[derive(Debug, Serialize, Deserialize)]
//Statement and Definition
pub struct VariableDefinition {
    pub public: bool,
    pub dtype: String,
    pub name: String,
    pub value: Expression
}
impl VariableDefinition {
    pub fn new() -> Self {
        VariableDefinition {public: false, dtype: "".to_string(), name: "".to_string(), value: Expression::Null(Null {})}
    }
}

#[derive(Debug, Serialize, Deserialize)]
// Statements
pub enum Statement {
    VariableDefinition(VariableDefinition),
    FunctionCall(FunctionCall),
    VariableRedefinition(VariableRedefinition),
    IfStatement(IfStatement),
    ElifStatement(ElifStatement),
    ElseStatement(ElseStatement),
    WhileStatement(WhileStatement),
    Null(Null)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub statements: Vec<Statement>
}
impl IfStatement {
    pub fn new() -> Self {
        IfStatement {condition: Expression::Null(Null {}), statements: vec![]}
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayLiteral {
    values: Vec<Expression>
}
impl ArrayLiteral {
    pub fn new() -> Self {
        ArrayLiteral {values: vec![]}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElifStatement {
    pub condition: Expression,
    pub statements: Vec<Statement>
}
impl ElifStatement {
    pub fn new() -> Self {
        ElifStatement {condition: Expression::Null(Null {}), statements: vec![]}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElseStatement {
    pub statements: Vec<Statement>
}
impl ElseStatement {
    pub fn new() -> Self {
        ElseStatement {statements: vec![]}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhileStatement {
    pub condition: Expression,
    pub statements: Vec<Statement>
}
impl WhileStatement {
    pub fn new() -> Self {
        WhileStatement {condition: Expression::Null(Null {}), statements: vec![]}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expression {
    BinaryOperation(BinaryOperation),
    Term(Term),
    Null(Null)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Term {
    FunctionCall(FunctionCall),
    IntegerLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
    StringLiteral(StringLiteral),
    CharLiteral(CharLiteral),
    BooleanLiteral(BooleanLiteral),
    VariableReference(VariableReference),
    ArrayLiteral(ArrayLiteral),
    Null(Null)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegerLiteral {
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FloatLiteral {
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringLiteral {
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharLiteral {
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanLiteral {
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableReference {
    pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguements: Vec<Expression>
}
impl FunctionCall {
    pub fn new() -> Self {
        FunctionCall {name: "".to_string(), arguements: vec![]}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableRedefinition {
    pub name: String,
    pub value: Expression
}
impl VariableRedefinition {
    pub fn new() -> Self {
        VariableRedefinition {name: "".to_string(), value: Expression::Null(Null {})}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub op: String,
    pub right: Box<Expression>,
}
impl BinaryOperation {
    pub fn new() -> Self {
        BinaryOperation {left: Box::new(Expression::Null(Null {})), op: "".to_string(), right: Box::new(Expression::Null(Null {}))}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Null {}
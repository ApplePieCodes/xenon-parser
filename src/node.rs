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
    pub dtype: String,
    pub name: String,
    pub value: Expression
}
impl VariableDefinition {
    pub fn new() -> Self {
        VariableDefinition {
            dtype: "".to_string(),
            name: "".to_string(),
            value: Expression::Term(Term::Null(Null::new())), // Correct enum wrapping
        }
    }
}


#[derive(Debug)]
pub struct StringLiteral {
    pub value: String
}
impl StringLiteral {
    pub fn new() -> Self {
        StringLiteral {value: "".to_string()}
    }
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
    BooleanLiteral(BooleanLiteral),
    Null(Null)
}

#[derive(Debug)]
pub enum Expression {
    BinaryOperation(BinaryOperation),
    Term(Term)
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

#[derive(Debug)]
pub struct Null {

}
impl Null {
    pub fn new() -> Self {
        Null {}
    }
}
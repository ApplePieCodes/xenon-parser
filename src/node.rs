#[derive(Debug)]
pub struct Program {
    pub use_statements: Vec<UseStatement>
}
impl Program {
    pub fn new() -> Self {
        Program {use_statements: Vec::new()}
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


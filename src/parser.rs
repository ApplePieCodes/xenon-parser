use crate::{lexer::{SourceLoc, Token, TokenKind}, node::{BinaryOperation, BooleanLiteral, ClassDefinition, Definition, Expression, FloatLiteral, FunctionCall, FunctionDefinition, IntegerLiteral, Namespace, Null, Program, Statement, StringLiteral, Term, UseStatement, VariableDefinition, VariableRedefinition, VariableReference}};

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
            else if self.match_token(0, TokenKind::UseKw) {
                program.usestatements.push(self.parse_use_statement());
            }
        }

        return program;
    }

    fn parse_use_statement(&mut self) -> UseStatement {
        let mut statement = UseStatement::new();

        self.i+=1;

        statement.name = self.peek(0).literal.to_string();

        self.i+=1;

        //Skip Semicolon

        self.i+=1;

        return statement;
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

        self.i+=1;

        return namespace;
    }

    fn parse_variable_definition(&mut self) -> VariableDefinition {
        let mut definition = VariableDefinition::new();
        
        if self.match_token(0, TokenKind::PubKw) {
            definition.public = true;
            self.i+=1;
        }

        definition.dtype = self.peek(0).literal.to_string();

        self.i+=1;

        definition.name = self.peek(0).literal.to_string();

        self.i+=1;

        if self.match_token(0, TokenKind::Equals) {
            self.i+=1;//Skip Equals
            definition.value = self.parse_expression();
        }

        return definition;
    }

    fn parse_expression(&mut self) -> Expression {
        if self.is_operator(0) {
            return Expression::BinaryOperation(self.parse_binary_operation(0));
        }
        else {
            let mut term = Expression::Term(self.parse_term());
            self.i+=1;
            return term;
        }
    }

    fn parse_binary_operation(&mut self, min_precedence: i32) -> BinaryOperation {
        let mut operation = BinaryOperation::new();
    
        // Parse the left-hand side
        operation.left = Box::new(Expression::Term(self.parse_term()));
    
        while self.is_operator(1) {
            let current_op = self.peek(0).literal.to_string();
            let precedence = self.get_precedence(&current_op);
    
            // If the operator's precedence is less than the current minimum, break out of the loop
            if precedence < min_precedence {
                break;
            }
    
            self.i += 1; // Consume the operator
    
            // Instead of parsing a Term for the right side, handle it as a BinaryOperation
            operation.op = current_op.clone(); // Store the operator
    
            // Parse the right-hand side, considering operator precedence
            let mut right_hand_side = Expression::Term(self.parse_term());
            while self.is_operator(1) {
                let next_op = self.peek(0).literal.to_string();
                let next_precedence = self.get_precedence(&next_op);
    
                // If the next operator has a higher precedence, handle it first
                if next_precedence > precedence {
                    right_hand_side = Expression::BinaryOperation(self.parse_binary_operation(next_precedence));
                } else {
                    break;
                }
            }
    
            operation.right = Box::new(right_hand_side); // Assign right side properly
        }
    
        operation // Return the fully constructed BinaryOperation
    }
    
    

    fn parse_term(&mut self) -> Term {
        if self.match_token(0, TokenKind::StringLit) {
            return Term::StringLiteral(StringLiteral {value: self.peek(0).literal.to_string()})
        }
        else if self.match_token(0, TokenKind::IntLit) {
            return Term::IntegerLiteral(IntegerLiteral {value: self.peek(0).literal.to_string()})
        }
        else if self.match_token(0, TokenKind::FloatLit) {
            return Term::FloatLiteral(FloatLiteral {value: self.peek(0).literal.to_string()})
        }
        else if self.match_token(0, TokenKind::TrueKw) || self.match_token(0, TokenKind::FalseKw) {
            return Term::BooleanLiteral(BooleanLiteral {value: self.peek(0).literal.to_string()})
        }
        else if self.match_token(0, TokenKind::Identifier) {
            if self.match_token(1, TokenKind::OpenParen) {
                return Term::FunctionCall(self.parse_function_call());
            }
            else {
                return Term::VariableReference(VariableReference {value: self.peek(0).literal.to_string()});
            }
        }
        else {
            return Term::Null(Null {});
        }
    }

    fn parse_function_call(&mut self) -> FunctionCall {
        let mut call = FunctionCall::new();

        call.name = self.peek(0).literal.to_string();

        self.i+=1;

        // Skip OpenParen

        self.i+=1;

        while ! self.match_token(0, TokenKind::CloseParen) {
            call.arguements.push(self.parse_expression());

            if self.match_token(0, TokenKind::Comma) {
                self.i+=1;
            }
        }

        // Skip Closing Paren

        self.i+=1;
        
        return call;
    }

    fn parse_definition(&mut self) -> Definition {
        if self.match_token(0, TokenKind::PubKw) {
            if self.match_token(1, TokenKind::ClassKw) {
                return Definition::ClassDefinition(self.parse_class_definition());
            }
            else if self.match_token(1, TokenKind::Identifier) && self.match_token(2, TokenKind::Identifier) {
                if self.match_token(3, TokenKind::OpenParen) {
                    return Definition::FunctionDefinition(self.parse_function_definition());
                }
                else {
                    return Definition::VariableDefinition(self.parse_variable_definition());
                }
            }
        }
        else {
            if self.match_token(0, TokenKind::ClassKw) {
                return Definition::ClassDefinition(self.parse_class_definition());
            }
            else if self.match_token(0, TokenKind::Identifier) && self.match_token(1, TokenKind::Identifier) {
                if self.match_token(2, TokenKind::OpenParen) {
                    return Definition::FunctionDefinition(self.parse_function_definition());
                }
                else {
                    return Definition::VariableDefinition(self.parse_variable_definition());
                }
            }
        }
        return  Definition::FunctionDefinition(FunctionDefinition::new());
    }

    fn parse_function_definition(&mut self) -> FunctionDefinition {
        let mut definition = FunctionDefinition::new();

        if self.match_token(0, TokenKind::PubKw) {
            definition.public = true;
            self.i+=1;
        }

        definition.ftype = self.peek(0).literal.to_string();

        self.i+=1;

        definition.name = self.peek(0).literal.to_string();

        self.i+=1;

        // Skip Open Paren

        self.i+=1;

        while ! self.match_token(0, TokenKind::CloseParen) {
            definition.arguements.push(self.parse_variable_definition());
        }

        self.i+=1;

        // Skip Opening Curly

        self.i+=1;

        while ! self.match_token(0, TokenKind::CloseCurly) {
            definition.statements.push(self.parse_statement());
        }

        self.i+=1;

        return definition;
    }

    fn parse_statement(&mut self) -> Statement {
        let mut statment: Statement;
        if self.match_token(0, TokenKind::Identifier) {
            if self.match_token(1, TokenKind::OpenParen) {
                statment = Statement::FunctionCall(self.parse_function_call());
            }
            else if self.match_token(1, TokenKind::Identifier) {
                statment = Statement::VariableDefinition(self.parse_variable_definition());
            }
            else if self.match_token(1, TokenKind::Equals) {
                statment = Statement::VariableRedefinition(self.parse_variable_redefinition());
            }
            else {
                statment = Statement::Null(Null {});
            }
        } 
        else {
            statment = Statement::Null(Null {});
        }

        //Skip Semicolon
        self.i+=1;
        

        return statment;
    }

    fn parse_variable_redefinition(&mut self) -> VariableRedefinition {
        let mut redefinition = VariableRedefinition::new();

        redefinition.name = self.peek(0).literal.to_string();

        self.i+=1;

        //Skip Equals

        self.i+=1;

        redefinition.value = self.parse_expression();

        return redefinition;
    }

    fn parse_class_definition(&mut self) -> ClassDefinition {
        let mut class = ClassDefinition::new();

        if self.match_token(0, TokenKind::PubKw) {
            class.public = true;
            self.i+=1;
        }

        self.i+=1;

        class.name = self.peek(0).literal.to_string();

        self.i+=1;

        // SKip open curly

        self.i+=1;

        while ! self.match_token(0, TokenKind::CloseCurly) {
            class.definitions.push(self.parse_definition());
        }

        self.i+=1;

        return class;
    }

    fn match_token(&mut self, offset: isize, t: TokenKind) -> bool {
        return self.peek(offset).kind == t;
    }

    fn peek(&self, offset: isize) -> Token {
        let index = self.i as isize + offset;
        if index >= 0 && index < self.tokens.len() as isize {
            self.tokens[index as usize].clone()
        } else {
            Token {kind: TokenKind::Whitespace, location: SourceLoc {line: 0, start: 0, end: 0}, literal: ""}
        }
    }

    fn is_operator(&mut self, offset: isize) -> bool {
        // Create a vector of the operator tokens you want to check
        let operators = vec![
            TokenKind::Add,
            TokenKind::Subtract,
            TokenKind::Multiply,
            TokenKind::Divide,
            TokenKind::Modulus,
            TokenKind::EqualsEquals,
            TokenKind::NotEqual,
            TokenKind::LessThan,
            TokenKind::GreaterThan,
            TokenKind::LessEqual,
            TokenKind::GreaterEq,
        ];

        // Check if the current token matches any of the operators
        for operator in operators {
            if self.match_token(offset, operator) {
                return true;
            }
        }
        false
    }

        // Function to return precedence based on the operator
    fn get_precedence(&self, op: &str) -> i32 {
        match op {
            "+" | "-" => 1,              // Addition and subtraction
            "*" | "/" | "%" => 2,        // Multiplication, division, and modulus
            "==" | "!=" | "<" | ">" | "<=" | ">=" => 3, // Comparison operators
            _ => 0,                      // Unknown operators or lowest precedence
        }
    }    
}

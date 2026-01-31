use std::collections::{HashMap, HashSet};

use log::trace;

use crate::{
    error::{
        CompilerError,
        SemanticErrorKind::{self},
    },
    frontend::{
        ast::*,
        tokens::Token,
        visitor::{
            AstVisitable, Visitor, semantic_error, walk_block, walk_declaration, walk_expression,
            walk_function, walk_statement,
        },
    },
};

pub struct VariableResolver {
    function_mappings: HashMap<String, HashSet<String>>,
    scopes: Vec<HashMap<String, String>>,
    error: Option<CompilerError>,
    current_function: String,
    counter: u64,
}

impl VariableResolver {
    pub fn new() -> Self {
        VariableResolver {
            function_mappings: HashMap::new(),
            scopes: Vec::new(),
            current_function: String::new(),
            error: None,
            counter: 0,
        }
    }

    fn begin_scope(&mut self) {
        trace!("Beginning new block scope");
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        let s = self.scopes.pop();
        trace!("Ending block scope: {:?}", s);
    }

    fn defined_in_scope(&self, name: &str) -> bool {
        return self.scopes.last().unwrap().contains_key(name);
    }

    fn is_in_scope(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(name) = scope.get(name) {
                return Some(name.clone());
            }
        }
        None
    }

    fn declare(&mut self, name: String) -> String {
        let unique_name = format!("{}_{}", name, self.counter);
        self.counter += 1;

        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.clone(), unique_name.clone());
        self.function_mappings
            .get_mut(&self.current_function)
            .unwrap()
            .insert(unique_name.clone());
        unique_name
    }
}

impl Visitor for VariableResolver {
    fn visit_function(&mut self, function: &mut ASTNode<Function>) {
        let Function::Function(name, _) = &function.node;
        trace!("Entering function: {}", name);
        self.current_function = name.clone();
        self.function_mappings.insert(name.clone(), HashSet::new());

        walk_function(self, function);
    }

    fn visit_block(&mut self, block: &mut ASTNode<Block>) {
        self.begin_scope();
        walk_block(self, block);
        self.end_scope();
    }

    fn visit_declaration(&mut self, declaration: &mut ASTNode<Declaration>) {
        if self.error.is_some() {
            return;
        }

        let Declaration::Declaration(name, _) = &mut declaration.node;

        if self.defined_in_scope(name) {
            self.error = semantic_error(
                declaration.span,
                SemanticErrorKind::MultipleVariableDefinition(name.clone()),
            );
        } else {
            trace!("Found variable in declaration: {}", name);
            let unique_name = self.declare(name.clone());
            *name = unique_name;
        }

        walk_declaration(self, declaration);
    }

    fn visit_statement(&mut self, statement: &mut ASTNode<Statement>){
        if self.error.is_some() {
            return;
        }
        
        match &mut statement.node {
            Statement::For(_, _, _, _,_) => {
                self.begin_scope();
                walk_statement(self, statement);
                self.end_scope();
            }
            _ => {
                walk_statement(self, statement);
            }
        }
        
    }

    fn visit_expression(&mut self, expression: &mut ASTNode<Expression>) {
        if self.error.is_some() {
            return;
        }

        match &mut expression.node {
            Expression::Variable(name) => {
                if let Some(unique_name) = self.is_in_scope(name) {
                    *name = unique_name
                } else {
                    self.error = semantic_error(
                        expression.span,
                        SemanticErrorKind::UndeclaredVariable(name.clone()),
                    )
                }
            }
            Expression::Assignment(left, _) => {
                if !left.node.is_assignable() {
                    self.error = semantic_error(left.span, SemanticErrorKind::InvalidAssignment)
                }
            }
            Expression::UnaryExpr(tok, expression) => {
                if *tok == Token::Increment || *tok == Token::Decrement {
                    if !expression.node.is_assignable() {
                        self.error =
                            semantic_error(expression.span, SemanticErrorKind::InvalidAssignment)
                    }
                }
            }
            _ => {}
        }

        walk_expression(self, expression);
    }
}

pub fn resolve_variables(program: &mut ASTNode<Program>) -> Result<HashSet<String>, CompilerError> {
    let mut resolver = VariableResolver::new();
    program.accept(&mut resolver);
    if let Some(error) = resolver.error {
        return Err(error);
    }
    Ok(resolver.function_mappings[&resolver.current_function].clone())
}

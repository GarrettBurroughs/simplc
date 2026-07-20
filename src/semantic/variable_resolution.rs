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
            AstVisitable, Visitor, semantic_error, walk_block, walk_expression,
            walk_function_declaration, walk_program, walk_statement, walk_variable_declaration,
        },
    },
};

#[derive(Debug, Clone)]
enum Linkage {
    External,
    None,
}

pub struct VariableResolver {
    function_mappings: HashMap<String, HashSet<String>>,
    scopes: Vec<HashMap<String, (String, Linkage)>>,
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
        trace!("Beginning new variable scope");
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        let s = self.scopes.pop();
        trace!("Ending variable scope: {:?}", s);
    }

    fn defined_in_scope(&self, name: &str) -> Option<&(String, Linkage)> {
        return self.scopes.last().unwrap().get(name);
    }

    fn is_in_scope(&self, name: &str) -> Option<(String, Linkage)> {
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
            .insert(name.clone(), (unique_name.clone(), Linkage::None));
        self.function_mappings
            .get_mut(&self.current_function)
            .unwrap()
            .insert(unique_name.clone());
        unique_name
    }
}

impl Visitor for VariableResolver {
    fn visit_program(&mut self, program: &mut ASTNode<Program>) {
        self.begin_scope();
        walk_program(self, program);
        self.end_scope();
    }
    fn visit_function_declaration(&mut self, function: &mut ASTNode<FunctionDeclaration>) {
        let FunctionDeclaration::FunctionDeclaration(name, arguments, body) = &mut function.node;
        if let Some(ident) = self.defined_in_scope(name) {
            if let Linkage::None = ident.1 {
                self.error = semantic_error(
                    function.span,
                    SemanticErrorKind::MultipleVariableDefinition(name.clone()),
                );
            }
        }
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.clone(), (name.clone(), Linkage::External));
        if body.is_some() {
            trace!("Entering function: {}", name);
            self.current_function = name.clone();
            self.function_mappings.insert(name.clone(), HashSet::new());
            self.begin_scope();
            for arg in arguments {
                if self.defined_in_scope(arg).is_some() {
                    self.error = semantic_error(
                        function.span,
                        SemanticErrorKind::MultipleVariableDefinition(arg.clone()),
                    );
                } else {
                    trace!("Found variable in function declaration: {}", arg);
                    let unique_name = self.declare(arg.clone());
                    *arg = unique_name;
                }
            }

            walk_function_declaration(self, function);
            self.end_scope();
        }
    }

    fn visit_block(&mut self, block: &mut ASTNode<Block>) {
        self.begin_scope();
        walk_block(self, block);
        self.end_scope();
    }

    fn visit_variable_declaration(
        &mut self,
        variable_declaration: &mut ASTNode<VariableDeclaration>,
    ) {
        if self.error.is_some() {
            return;
        }

        let VariableDeclaration::VariableDeclaration(name, _) = &mut variable_declaration.node;

        if self.defined_in_scope(name).is_some() {
            self.error = semantic_error(
                variable_declaration.span,
                SemanticErrorKind::MultipleVariableDefinition(name.clone()),
            );
        } else {
            trace!("Found variable in declaration: {}", name);
            let unique_name = self.declare(name.clone());
            *name = unique_name;
        }

        walk_variable_declaration(self, variable_declaration);
    }

    fn visit_statement(&mut self, statement: &mut ASTNode<Statement>) {
        if self.error.is_some() {
            return;
        }

        match &mut statement.node {
            Statement::For(_, _, _, _, _) => {
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
                    *name = unique_name.0
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
            Expression::FunctionCall(name, _) => {
                if let Some(unique_name) = self.is_in_scope(name) {
                    *name = unique_name.0
                } else {
                    self.error = semantic_error(
                        expression.span,
                        SemanticErrorKind::UndeclaredFunction(name.clone()),
                    )
                }
            }
            _ => {}
        }

        walk_expression(self, expression);
    }
}

pub fn resolve_variables(
    program: &mut ASTNode<Program>,
) -> Result<HashMap<String, HashSet<String>>, CompilerError> {
    let mut resolver = VariableResolver::new();
    program.accept(&mut resolver);
    if let Some(error) = resolver.error {
        return Err(error);
    }
    // debug!("{:?}", resolver.function_mappings);
    Ok(resolver.function_mappings)
}

use std::collections::{HashSet};

use log::trace;

use crate::{
    error::{CompilerError, SemanticErrorKind::{self}},
    frontend::{
        ast::*,
        tokens::Token,
        visitor::{AstVisitable, Visitor, semantic_error, walk_declaration, walk_expression},
    },
};

pub struct VariableResolver {
    variable_set: HashSet<String>,
    error: Option<CompilerError>,
}

impl VariableResolver {
    pub fn new() -> Self {
        VariableResolver {
            variable_set: HashSet::new(),
            error: None,
        }
    }

}

impl Visitor for VariableResolver {
    fn visit_declaration(&mut self, declaration: &mut ASTNode<Declaration>) {
        if self.error.is_some() {
            return;
        }

        let Declaration::Declaration(name, _) = &declaration.node;
        if self.variable_set.contains(name) {
            self.error = semantic_error(declaration, SemanticErrorKind::MultipleVariableDefinition(name.clone()));
        } else {
            trace!("Found variable in declaration: {}", name);
            self.variable_set.insert(name.clone());
        }

        walk_declaration(self, declaration);
    }

    fn visit_expression(&mut self, expression: &mut ASTNode<Expression>) {
        if self.error.is_some() {
            return;
        }

        match &expression.node {
            Expression::Variable(name) => {
                if !self.variable_set.contains(name) {
                    self.error = semantic_error(expression, SemanticErrorKind::UndeclaredVariable(name.clone()))
                }
            }
            Expression::Assignment(left, _) => {
                if !left.node.is_assignable() {
                    self.error = semantic_error(left, SemanticErrorKind::InvalidAssignment)
                }
            }
            Expression::UnaryExpr(tok, expression) => {
                if *tok == Token::Increment || *tok == Token::Decrement {
                    if !expression.node.is_assignable() {
                        self.error = semantic_error(expression, SemanticErrorKind::InvalidAssignment)
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
    Ok(resolver.variable_set)
}

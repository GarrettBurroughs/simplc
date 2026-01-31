use std::collections::HashSet;

use log::trace;

use crate::{
    error::{CompilerError, SemanticErrorKind::{AlreadyDeclaredLabel, UndeclaredLabel}},
    frontend::{
        ast::{ASTNode, Program, Statement},
        visitor::{AstVisitable, Visitor, semantic_error, walk_statement},
    },
};

pub struct LabelResolver {
    label_set: HashSet<String>,
    error: Option<CompilerError>,
}

impl LabelResolver {
    pub fn new() -> Self {
        LabelResolver {
            label_set: HashSet::new(),
            error: None,
        }
    }
}

impl Visitor for LabelResolver {
    fn visit_statement(&mut self, statement: &mut ASTNode<Statement>) {
        if self.error.is_some() {
            return;
        }

        match &statement.node {
            Statement::Label(label, _) => {
                if self.label_set.contains(label) {
                    self.error = semantic_error(statement.span, AlreadyDeclaredLabel(label.clone()));
                }
                trace!("Found label: {}", label);
                self.label_set.insert(label.to_string());
            }
            _ => {}
        }
        walk_statement(self, statement);
    }
}

pub struct GotoResolver {
    label_set: HashSet<String>,
    error: Option<CompilerError>,
}

impl GotoResolver {
    pub fn new(label_set: HashSet<String>) -> Self {
        GotoResolver {
            label_set: label_set,
            error: None,
        }
    }
}

impl Visitor for GotoResolver {
    fn visit_statement(&mut self, statement: &mut ASTNode<Statement>) {
        if self.error.is_some() {
            return;
        }

        match &statement.node {
            Statement::Goto(label) => {
                if !self.label_set.contains(label) {
                    self.error = semantic_error(statement.span, UndeclaredLabel(label.clone()));
                }
            }
            _ => {}
        }
        walk_statement(self, statement);
    }
}

pub fn resolve_labels(program: &mut ASTNode<Program>) -> Result<HashSet<String>, CompilerError> {
    let mut label_resolver = LabelResolver::new();
    program.accept(&mut label_resolver);
    if let Some(err) = label_resolver.error {
        return Err(err);
    }
    let mut goto_resolver = GotoResolver::new(label_resolver.label_set);
    program.accept(&mut goto_resolver);
    if let Some(err) = goto_resolver.error {
        return Err(err);
    }
    Ok(goto_resolver.label_set)

}


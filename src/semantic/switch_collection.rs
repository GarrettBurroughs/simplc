use std::collections::{HashMap, HashSet};

use crate::{
    error::{CompilerError, SemanticErrorKind::DuplicateCase},
    frontend::{
        ast::{ASTNode, Program, Statement},
        visitor::{AstVisitable, Visitor, walk_statement},
    },
};

#[derive(Debug, Clone)]
pub struct Switch {
    pub cases: Vec<(String, i32)>,
    pub default: Option<String>,
    existing_labels: HashSet<String>,
}

pub struct SwitchCollector {
    switch_statements: HashMap<String, Switch>,
    error: Option<CompilerError>,
}

impl SwitchCollector {
    fn new() -> Self {
        Self {
            switch_statements: HashMap::new(),
            error: None,
        }
    }
}

impl Visitor for SwitchCollector {
    fn visit_statement(&mut self, statement: &mut ASTNode<Statement>) {
        match &mut statement.node {
            Statement::Switch(_, _, label) => {
                self.switch_statements.insert(
                    label.clone().expect("switch should be labeled"),
                    Switch {
                        cases: Vec::new(),
                        default: None,
                        existing_labels: HashSet::new(),
                    },
                );
            }
            Statement::Case(expr, _, label) => {
                let v = expr.node.evaluate_const();
                let new_label = label
                    .clone()
                    .map(|l| format!("{}_case{}", l, v))
                    .expect("case to have a label");
                let switch = self
                    .switch_statements
                    .get_mut(&label.clone().expect("case should be labeled"))
                    .expect("switch statement to exist");
                if switch.existing_labels.contains(&new_label) {
                    self.error = Some(CompilerError::Semantic {
                        location: statement.span.into(),
                        kind: DuplicateCase,
                    })
                }

                switch.existing_labels.insert(new_label.clone());
                switch.cases.push((new_label.clone(), v));
                *label = Some(new_label);
            }
            Statement::Default(_, label) => {
                let new_label = label
                    .clone()
                    .map(|l| format!("{}_default", l))
                    .expect("default to have a label");
                let switch = self
                    .switch_statements
                    .get_mut(&label.clone().expect("case should be labeled"))
                    .expect("switch statement to exist");

                if switch.existing_labels.contains(&new_label) {
                    self.error = Some(CompilerError::Semantic {
                        location: statement.span.into(),
                        kind: DuplicateCase,
                    })
                }
                switch.existing_labels.insert(new_label.clone());
                switch.default = Some(new_label.clone());
                *label = Some(new_label);
            }
            _ => {}
        }
        walk_statement(self, statement);
    }
}

pub fn collect_switch(
    program: &mut ASTNode<Program>,
) -> Result<HashMap<String, Switch>, CompilerError> {
    let mut switch_collector = SwitchCollector::new();
    program.accept(&mut switch_collector);
    if let Some(e) = switch_collector.error {
        return Err(e);
    }
    Ok(switch_collector.switch_statements)
}

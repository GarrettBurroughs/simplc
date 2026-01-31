use crate::{
    error::{CompilerError, SemanticErrorKind},
    frontend::{
        ast::{ASTNode, Program, Statement},
        visitor::{AstVisitable, Visitor, walk_statement},
    },
};

pub struct LoopLabeler {
    loop_counter: i32,
    continue_stack: Vec<String>,
    break_stack: Vec<String>,
    switch_stack: Vec<String>,
    error: Option<CompilerError>,
}

impl LoopLabeler {
    fn new() -> Self {
        Self {
            loop_counter: 0,
            continue_stack: Vec::new(),
            break_stack: Vec::new(),
            switch_stack: Vec::new(),
            error: None,
        }
    }

    fn generate_label(&mut self, prefix: &str, is_loop: bool) -> String {
        let label = format!("{}_{}", prefix, self.loop_counter);
        self.break_stack.push(label.clone());
        if is_loop {
            self.continue_stack.push(label.clone());
        } else {
            self.switch_stack.push(label.clone());
        }
        self.loop_counter += 1;
        label
    }
}

impl Visitor for LoopLabeler {
    fn visit_statement(&mut self, statement: &mut ASTNode<Statement>) {
        if self.error.is_some() {
            return;
        }
        match &mut statement.node {
            Statement::For(_, _, _, _, label) => {
                *label = Some(self.generate_label("for", true));
                walk_statement(self, statement);
                self.break_stack.pop();
                self.continue_stack.pop();
            }
            Statement::While(_, _, label) => {
                *label = Some(self.generate_label("while", true));
                walk_statement(self, statement);
                self.break_stack.pop();
                self.continue_stack.pop();
            }
            Statement::DoWhile(_, _, label) => {
                *label = Some(self.generate_label("do_while", true));
                walk_statement(self, statement);
                self.break_stack.pop();
                self.continue_stack.pop();
            }
            Statement::Switch(_, _, label) => {
                *label = Some(self.generate_label("switch", false));
                walk_statement(self, statement);
                self.break_stack.pop();
                self.switch_stack.pop();
            }
            Statement::Break(label) => {
                if let Some(l) = self.break_stack.last() {
                    *label = Some(l.clone())
                } else {
                    self.error = Some(CompilerError::SemanticError {
                        location: statement.span.into(),
                        kind: SemanticErrorKind::InvalidBreak,
                    })
                }
                walk_statement(self, statement);
            }
            Statement::Case(_, _, label) => {
                if let Some(l) = self.switch_stack.last() {
                    *label = Some(l.clone())
                } else {
                    self.error = Some(CompilerError::SemanticError {
                        location: statement.span.into(),
                        kind: SemanticErrorKind::InvalidCase,
                    })
                }
                walk_statement(self, statement);
            }
            Statement::Default(_, label) => {
                if let Some(l) = self.switch_stack.last() {
                    *label = Some(l.clone())
                } else {
                    self.error = Some(CompilerError::SemanticError {
                        location: statement.span.into(),
                        kind: SemanticErrorKind::InvalidCase,
                    })
                }
                walk_statement(self, statement);
            }
            Statement::Continue(label) => {
                if let Some(l) = self.continue_stack.last() {
                    *label = Some(l.clone())
                } else {
                    self.error = Some(CompilerError::SemanticError {
                        location: statement.span.into(),
                        kind: SemanticErrorKind::InvalidContinue,
                    })
                }
                walk_statement(self, statement);
            }
            _ => {
                walk_statement(self, statement);
            }
        }
    }
}

pub fn label_loops(program: &mut ASTNode<Program>) -> Result<(), CompilerError> {
    let mut loop_labeler = LoopLabeler::new();
    program.accept(&mut loop_labeler);
    if let Some(err) = loop_labeler.error {
        return Err(err);
    }
    Ok(())
}

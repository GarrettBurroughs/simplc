use std::collections::HashMap;

use crate::{
    CompilerError,
    frontend::{
        ast::*,
        tokens::Token,
        visitor::{AstVisitable, Visitor, walk_declaration, walk_expression},
    },
};

pub struct VariableResolver {
    variable_map: HashMap<String, String>,
    counter: i32,
    error: Option<CompilerError>,
}

impl VariableResolver {
    pub fn new() -> Self {
        VariableResolver {
            variable_map: HashMap::new(),
            counter: 0,
            error: None,
        }
    }

    fn get_unique_name(&mut self, name: &String) -> String {
        let new_name = format!("{}", name);
        self.counter += 1;
        new_name
    }
}

impl Visitor for VariableResolver {
    fn visit_declaration(&mut self, declaration: &mut ASTNode<Declaration>) {
        if self.error.is_some() {
            return;
        }

        let Declaration::Declaration(name, _) = &mut declaration.node;
        if self.variable_map.contains_key(name) {
            self.error = Some(CompilerError::SemanticError(
                declaration.row,
                declaration.column,
                format!("Variable {} already defined", name),
            ));
        } else {
            let unique_name = self.get_unique_name(name);
            self.variable_map.insert(name.clone(), unique_name.clone());
            *name = unique_name;
        }
        walk_declaration(self, declaration);
    }

    fn visit_expression(&mut self, expression: &mut ASTNode<Expression>) {
        if self.error.is_some() {
            return;
        }

        match &mut expression.node {
            Expression::Variable(name) => {
                if let Some(v) = self.variable_map.get(name) {
                    *name = v.clone();
                } else {
                    self.error = Some(CompilerError::SemanticError(
                        expression.row,
                        expression.column,
                        format!("Undefined variable {} ", name),
                    ));
                }
            }
            Expression::Assignment(left, _) => {
                if !matches!(left.node, Expression::Variable(_)) {
                    self.error = Some(CompilerError::SemanticError(
                        expression.row,
                        expression.column,
                        "Invalid left hand side of assignment operator".to_string(),
                    ));
                }
            }
            Expression::UnaryExpr(tok, expression) => {
                if *tok == Token::Increment || *tok == Token::Decrement {
                    if !expression.node.is_assignable() {
                        self.error = Some(CompilerError::SemanticError(
                            expression.row,
                            expression.column,
                            format!("Cannot {} {:?}", tok, expression),
                        ))
                    }
                }
            }
            _ => {}
        }

        walk_expression(self, expression);
    }
}

pub fn resolve_variables(
    program: &mut ASTNode<Program>,
) -> Result<HashMap<String, String>, CompilerError> {
    let mut resolver = VariableResolver::new();
    program.accept(&mut resolver);
    if let Some(error) = resolver.error {
        Err(error)
    } else {
        Ok(resolver.variable_map)
    }
}

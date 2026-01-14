use std::{collections::HashMap};

use crate::{CompilerError, frontend::{ast::*, tokens::Token}};

pub struct VariableResolution {
    variable_map: HashMap<String, String>,
    counter: i32,
}

impl VariableResolution {
    pub fn new() -> Self {
        VariableResolution {
            variable_map: HashMap::new(),
            counter: 0,
        }
    }

    fn get_unique_name(&mut self, name: &String) -> String {
        let new_name = format!("{}{}", name, self.counter);
        self.counter += 1;
        return new_name;
    }
}

pub trait VerifyResolution {
    fn verify_resolution(
        &mut self,
        variable_resolution: &mut VariableResolution,
    ) -> Option<CompilerError>;
}

impl VerifyResolution for ASTNode<Program> {
    fn verify_resolution(
        &mut self,
        variable_resolution: &mut VariableResolution,
    ) -> Option<CompilerError> {
        match &mut self.node {
            Program::Program(function) => function.verify_resolution(variable_resolution),
        }
    }
}

impl VerifyResolution for ASTNode<Function> {
    fn verify_resolution(
        &mut self,
        variable_resolution: &mut VariableResolution,
    ) -> Option<CompilerError> {
        match &mut self.node {
            Function::Function(_, blocks) => blocks
                .iter_mut()
                .map(|b| b.verify_resolution(variable_resolution))
                .reduce(|r1, r2| r1.or(r2))
                .flatten(),
        }
    }
}

impl VerifyResolution for ASTNode<Block> {
    fn verify_resolution(
        &mut self,
        variable_resolution: &mut VariableResolution,
    ) -> Option<CompilerError> {
        match &mut self.node {
            Block::Declaration(declaration) => declaration.verify_resolution(variable_resolution),
            Block::Statement(statement) => statement.verify_resolution(variable_resolution),
        }
    }
}

impl VerifyResolution for ASTNode<Declaration> {
    fn verify_resolution(
        &mut self,
        variable_resolution: &mut VariableResolution,
    ) -> Option<CompilerError> {
        match &mut self.node {
            Declaration::Declaration(name, expression) => {
                if variable_resolution.variable_map.contains_key(name) {
                    return Some(CompilerError::SemanticError(
                        self.row,
                        self.column,
                        format!("Variable {} already defined", name),
                    ));
                } else {
                    let unique_name = variable_resolution.get_unique_name(name);
                    variable_resolution
                        .variable_map
                        .insert(name.to_string(), unique_name.to_string());
                    if let Some(expr) = expression {
                        expr.verify_resolution(variable_resolution);
                    }
                    *name = unique_name;
                    return None;
                }
            }
        }
    }
}

impl VerifyResolution for ASTNode<Statement> {
    fn verify_resolution(
        &mut self,
        variable_resolution: &mut VariableResolution,
    ) -> Option<CompilerError> {
        match &mut self.node {
            Statement::Return(expression) => expression.verify_resolution(variable_resolution),
            Statement::Expression(expression) => expression.verify_resolution(variable_resolution),
            Statement::Null => None,
            Statement::If(astnode, astnode1, astnode2) => {
                None
            }
        }
    }
}

impl VerifyResolution for ASTNode<Expression> {
    fn verify_resolution(
        &mut self,
        variable_resolution: &mut VariableResolution,
    ) -> Option<CompilerError> {
        match &mut self.node {
            Expression::IntLiteral(_) => None,
            Expression::Variable(name) => {
                if let Some(v) = variable_resolution.variable_map.get(name) {
                    *name = v.to_string();
                    None
                } else {
                    Some(CompilerError::SemanticError(
                        self.row,
                        self.column,
                        format!("Undefined variable {} ", name),
                    ))
                }
            }
            Expression::UnaryExpr(tok, expression) => {
                if *tok == Token::Increment || *tok == Token::Decrement {
                    if let Expression::Variable(_) = &expression.node {} else {
                        return Some(CompilerError::SemanticError(self.row, self.column, "".into()));
                    }
                }
                expression.verify_resolution(variable_resolution)
            }
            Expression::BinaryExpr(_, left, right) => {
                let l_valid = left.verify_resolution(variable_resolution);
                let r_valid = right.verify_resolution(variable_resolution);
                l_valid.or(r_valid)
            }
            Expression::Assignment(left, right) => {
                if let Expression::Variable(_) = (**left).node {
                    let l_valid = left.verify_resolution(variable_resolution);
                    let r_valid = right.verify_resolution(variable_resolution);
                    return l_valid.or(r_valid);
                }
                Some(CompilerError::SemanticError(
                    self.row,
                    self.column,
                    format!("Invalid left hand side of assignment operator {:?}", left),
                ))
            }
        }
    }
}

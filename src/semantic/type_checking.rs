use std::{collections::HashMap, fmt::Display};

use log::trace;

use crate::{
    error::{CompilerError, SemanticErrorKind},
    frontend::{
        ast::{ASTNode, Expression, FunctionDeclaration, Program, VariableDeclaration},
        visitor::{
            AstVisitable, Visitor, semantic_error, walk_expression, walk_function_declaration,
            walk_variable_declaration,
        },
    },
};

// TODO(https://github.com/GarrettBurroughs/simplc/issues/4): Refactor types into own module
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    Int,
    // num args, has_definition
    Function(usize),
}

impl Display for Type {
    // TODO(https://github.com/GarrettBurroughs/simplc/issues/4): Handle type displays better
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct SymbolEntry {
    symbol_type: Type,
    defined: bool,
}

struct TypeChecker {
    table: HashMap<String, SymbolEntry>,
    error: Option<CompilerError>,
}

impl Visitor for TypeChecker {
    fn visit_variable_declaration(
        &mut self,
        variable_declaration: &mut ASTNode<VariableDeclaration>,
    ) {
        let VariableDeclaration::VariableDeclaration(name, _) = &variable_declaration.node;

        // Right now, all variables are of type int
        self.table.insert(
            name.clone(),
            SymbolEntry {
                symbol_type: Type::Int,
                defined: true,
            },
        );
        trace!("Type checked variable {} with type {:?}", name, Type::Int);

        walk_variable_declaration(self, variable_declaration);
    }

    fn visit_function_declaration(&mut self, function: &mut ASTNode<FunctionDeclaration>) {
        let FunctionDeclaration::FunctionDeclaration(name, params, body) = &function.node;
        let fn_type = Type::Function(params.len());

        trace!(
            "Type checking function declaration {:?} with type {:?}",
            name, fn_type
        );

        let defined = if let Some(def) = self.table.get(name) {
            trace!("Found previous declaration {:?}", def);
            // Functions can be redeclared, but must have the same type
            if def.symbol_type != fn_type {
                self.error = semantic_error(
                    function.span,
                    SemanticErrorKind::InvalidType {
                        got: fn_type,
                        expected: def.symbol_type.clone(),
                    },
                );
                return;
            }
            // Check for redefinition
            if def.defined && body.is_some() {
                self.error = semantic_error(
                    function.span,
                    SemanticErrorKind::MultipleFunctionDefinition(name.clone()),
                );
                return;
            }
            def.defined
        } else {
            body.is_some()
        };
        self.table.insert(
            name.clone(),
            SymbolEntry {
                symbol_type: fn_type,
                defined,
            },
        );
        for param in params {
            self.table.insert(
                param.clone(),
                SymbolEntry {
                    symbol_type: Type::Int,
                    defined: true,
                },
            );
        }
        walk_function_declaration(self, function);
    }

    fn visit_expression(&mut self, expression: &mut ASTNode<Expression>) {
        match &expression.node {
            Expression::FunctionCall(name, args) => {
                let fn_type = self
                    .table
                    .get(name)
                    .expect("function declarations have already been checked");

                let caller_type = Type::Function(args.len());
                if fn_type.symbol_type != caller_type {
                    self.error = semantic_error(
                        expression.span,
                        SemanticErrorKind::InvalidType {
                            got: caller_type,
                            expected: fn_type.symbol_type.clone(),
                        },
                    );
                    return;
                }
            }
            Expression::Variable(name) => {
                let var_type = self
                    .table
                    .get(name)
                    .expect("variable to have been declared");

                if var_type.symbol_type != Type::Int {
                    self.error = semantic_error(
                        expression.span,
                        SemanticErrorKind::InvalidType {
                            got: var_type.symbol_type.clone(),
                            expected: Type::Int,
                        },
                    );
                    return;
                }
            }
            _ => {}
        }
        walk_expression(self, expression);
    }
}

pub fn check_types(
    program: &mut ASTNode<Program>,
) -> Result<HashMap<String, SymbolEntry>, CompilerError> {
    let mut type_checker = TypeChecker {
        table: HashMap::new(),
        error: None,
    };
    program.accept(&mut type_checker);
    if let Some(err) = type_checker.error {
        return Err(err);
    }
    Ok(type_checker.table)
}

use crate::frontend::{
    ast::{ASTNode, Program},
    visitor::{AstVisitable, Visitor},
};

struct TypeChecker;

impl Visitor for TypeChecker {}

pub fn check_types(program: &mut ASTNode<Program>) {
    let mut type_checker = TypeChecker;
    program.accept(&mut type_checker);
}

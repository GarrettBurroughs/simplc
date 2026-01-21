use inkwell::{builder::BuilderError, values::{BasicValueEnum}};

use crate::{codegen::codegen::{CodeGen, CodeGenerator}, frontend::ast::{ASTNode, Declaration}};


impl<'ctx> CodeGenerator<'ctx> for ASTNode<Declaration> {
    fn codegen(&self, codegen: &mut CodeGen<'ctx>) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        match &self.node {
            Declaration::Declaration(name, astnode) => {
                let ptr = codegen.variable_map.get(name).unwrap().clone();

                // If it is a compound assignment, store the value
                if let Some(node) = astnode {
                    let val = node.codegen(codegen)?.expect("expression to have a value");
                    codegen.builder.build_store(ptr, val)?;
                } 

                Ok(None)
            }
        }
    }
}


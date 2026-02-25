use inkwell::{builder::BuilderError, values::BasicValueEnum};

use crate::{
    codegen::codegen::{CodeGen, CodeGenerator},
    frontend::ast::{ASTNode, VariableDeclaration},
};

impl<'ctx> CodeGenerator<'ctx> for ASTNode<VariableDeclaration> {
    fn codegen(
        &self,
        codegen: &mut CodeGen<'ctx>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        match &self.node {
            VariableDeclaration::VariableDeclaration(name, astnode) => {
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

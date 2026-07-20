use inkwell::{builder::BuilderError, values::BasicValueEnum};

use crate::{
    codegen::codegen::{CodeGen, CodeGenerator},
    frontend::ast::{ASTNode, Program},
};

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Program> {
    fn codegen(
        &self,
        codegen: &mut CodeGen<'ctx>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        match &self.node {
            Program::Program(function_list) => {
                for function in function_list {
                    function.codegen(codegen)?;
                }
            }
        };
        Ok(None)
    }
}

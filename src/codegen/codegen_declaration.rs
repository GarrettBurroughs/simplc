use inkwell::{builder::BuilderError, values::BasicValueEnum};

use crate::{
    codegen::codegen::{CodeGen, CodeGenerator},
    frontend::ast::{ASTNode, Declaration},
};

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Declaration> {
    fn codegen(
        &self,
        codegen: &mut CodeGen<'ctx>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        match &self.node {
            Declaration::FunctionDeclaration(fn_decl) => fn_decl.codegen(codegen)?,
            Declaration::VariableDeclaration(v_decl) => v_decl.codegen(codegen)?,
        };
        Ok(None)
    }
}

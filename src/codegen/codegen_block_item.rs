use inkwell::{builder::BuilderError, values::BasicValueEnum};

use crate::{
    codegen::codegen::{CodeGen, CodeGenerator},
    frontend::ast::{ASTNode, BlockItem},
};

impl<'ctx> CodeGenerator<'ctx> for ASTNode<BlockItem> {
    fn codegen(
        &self,
        codegen: &mut CodeGen<'ctx>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        match &self.node {
            BlockItem::Declaration(decl) => decl.codegen(codegen),
            BlockItem::Statement(statement) => statement.codegen(codegen),
        }
    }
}

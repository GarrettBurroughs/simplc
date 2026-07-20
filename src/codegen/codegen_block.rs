use inkwell::{builder::BuilderError, values::BasicValueEnum};

use crate::{
    codegen::codegen::{CodeGen, CodeGenerator},
    frontend::ast::{ASTNode, Block},
};

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Block> {
    fn codegen(
        &self,
        codegen: &mut CodeGen<'ctx>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        match &self.node {
            Block::Block(block_items) => {
                for block_item in block_items {
                    block_item.codegen(codegen)?;
                }
            }
        }
        Ok(None)
    }
}

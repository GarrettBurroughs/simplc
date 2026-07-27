use inkwell::{builder::BuilderError, values::BasicValueEnum};

use crate::{
    codegen::{CodeGen, CodeGenerator},
    frontend::ast::{ASTNode, Initializer},
};

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Initializer> {
    fn codegen(
        &self,
        codegen: &mut CodeGen<'ctx>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        match &self.node {
            Initializer::Decl(decl) => {
                decl.codegen(codegen)?;
            }
            Initializer::Exp(expr) => {
                if let Some(expr) = expr {
                    expr.codegen(codegen)?;
                }
            }
        }
        Ok(None)
    }
}

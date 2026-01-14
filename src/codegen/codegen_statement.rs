use inkwell::{builder::BuilderError, values::BasicValueEnum};

use crate::{codegen::codegen::{CodeGen, CodeGenerator}, frontend::ast::{ASTNode, Statement}};


impl<'ctx> CodeGenerator<'ctx> for ASTNode<Statement> {
    fn codegen(&self, codegen: &mut CodeGen<'ctx>) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError>{
        match &self.node {
            Statement::Return(expression) => {
                let exp = expression.codegen(codegen)?.expect("expression to have a value");
                codegen.builder.build_return(Some(&exp))?;
                Ok(None)
            }
            Statement::Expression(expr) => expr.codegen(codegen),
            Statement::Null => Ok(None),
            Statement::If(condition, then, elseStmnt) => panic!("Not yet implemented"),
        }
    }
}

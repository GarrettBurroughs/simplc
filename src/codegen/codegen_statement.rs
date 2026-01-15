use inkwell::{builder::BuilderError, values::BasicValueEnum};

use crate::{
    codegen::codegen::{CodeGen, CodeGenerator},
    frontend::ast::{ASTNode, Statement},
};

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Statement> {
    fn codegen(
        &self,
        codegen: &mut CodeGen<'ctx>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        match &self.node {
            Statement::Return(expression) => {
                let exp = expression
                    .codegen(codegen)?
                    .expect("expression to have a value");
                codegen.builder.build_return(Some(&exp))?;
            }
            Statement::Expression(expr) => {
                expr.codegen(codegen)?;
            }
            Statement::Null => (),
            Statement::If(condition, then, else_stmnt) => {
                let cond = condition
                    .codegen(codegen)?
                    .expect("expected condition to have a value")
                    .into_int_value();

                let cmp = codegen.builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    cond,
                    cond.get_type().const_zero(),
                    "if_cmp",
                )?;

                let current_fn = codegen
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap();

                let then_block = codegen.context.append_basic_block(current_fn, "then_block");
                let else_block = codegen.context.append_basic_block(current_fn, "else_block");
                let merge_block = codegen.context.append_basic_block(current_fn, "merge_block");

                codegen.builder.build_conditional_branch(cmp, then_block, else_block)?;

                codegen.builder.position_at_end(then_block);
                then.codegen(codegen)?;
                codegen.builder.build_unconditional_branch(merge_block)?;

                codegen.builder.position_at_end(else_block);
                if let Some(else_stmnt) = else_stmnt {
                    else_stmnt.codegen(codegen)?;
                }
                codegen.builder.build_unconditional_branch(merge_block)?;

                codegen.builder.position_at_end(merge_block);
            }
            Statement::Label(name, stmt) => {
                let label_block = codegen.get_basic_block(name);

                let current_block = codegen
                    .builder
                    .get_insert_block()
                    .unwrap();

                if current_block.get_terminator().is_none() {
                    codegen.builder.build_unconditional_branch(label_block)?;
                }

                codegen.builder.position_at_end(label_block);
                stmt.codegen(codegen)?;
            }
            Statement::Goto(name) => {
                let label_block = codegen.get_basic_block(name);
                codegen.builder.build_unconditional_branch(label_block)?;
                let current_fn = codegen
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap();
                let dead_block = codegen.context.append_basic_block(current_fn, "dead");
                codegen.builder.position_at_end(dead_block);
            }
        };
        Ok(None)
    }
}

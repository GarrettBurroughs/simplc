use inkwell::{builder::BuilderError, values::BasicValueEnum};

use crate::{codegen::codegen::{CodeGen, CodeGenerator}, frontend::ast::{ASTNode, Function}};

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Function> {
    fn codegen(&self, codegen: &mut CodeGen<'ctx>) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        match &self.node {
            Function::Function(name, blocks) => {
                let i64_type = codegen.context.i64_type();
                let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);
                let function = codegen.module.add_function(name, fn_type, None);
                let basic_block = codegen.context.append_basic_block(function, "entry");
                codegen.builder.position_at_end(basic_block);

                for v in &codegen.variables {
                    let ptr = codegen.builder.build_alloca(codegen.context.i64_type(), v).unwrap();
                    codegen.variable_map.insert(v.to_string(), ptr);
                }

                for block in blocks {
                    block.codegen(codegen)?;
                }
                let current_block = codegen.builder.get_insert_block().unwrap();
                if let None = current_block.get_terminator() {
                    codegen
                        .builder
                        .build_return(Some(&codegen.context.i64_type().const_zero()))?;
                };

                Ok(None)
            }
        }
    }
}


use inkwell::{builder::BuilderError, values::BasicValueEnum};

use crate::{
    codegen::{CodeGen, CodeGenerator},
    frontend::ast::{ASTNode, FunctionDeclaration},
};

impl<'ctx> CodeGenerator<'ctx> for ASTNode<FunctionDeclaration> {
    fn codegen(
        &self,
        codegen: &mut CodeGen<'ctx>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        match &self.node {
            // Function declaration codegen sets up a few additional things
            // for the function scope:
            //
            // - Creates local variable bindings for the function arguments
            //
            // - Initializes all local variables within the function (scoping is handled in
            // the semantic passes)
            //
            // - Creates all basic blocks for case statements within the function
            //
            // - Creates a unified return block as an exit point and variable for return value
            FunctionDeclaration::FunctionDeclaration(name, arguments, Some(block)) => {
                let i64_type = codegen.context.i64_type();
                // Create local variable bindings for function arguments
                let mut args = Vec::new();
                let function = match codegen.functions.get(name) {
                    Some(func) => *func,
                    None => {
                        for _ in arguments {
                            args.push(i64_type.into());
                        }
                        let fn_type = i64_type.fn_type(&args, false);
                        let function = codegen.module.add_function(name, fn_type, None);
                        codegen.functions.insert(name.clone(), function);
                        function
                    }
                };

                let basic_block = codegen.context.append_basic_block(function, "entry");
                codegen.builder.position_at_end(basic_block);

                // Set up unified return point
                let return_block = codegen.context.append_basic_block(function, "exit");
                codegen.return_block = Some(return_block);
                codegen.return_value = Some(
                    codegen
                        .builder
                        .build_alloca(i64_type, &format!("{}_return", name))?,
                );
                codegen
                    .builder
                    .build_store(codegen.return_value.unwrap(), i64_type.const_zero())?;

                // Initialize/Allocate all function variables
                for v in &codegen.variable_mappings[name] {
                    let ptr = codegen
                        .builder
                        .build_alloca(codegen.context.i64_type(), v)
                        .unwrap();
                    codegen.variable_map.insert(v.to_string(), ptr);
                }

                // Assign arguments to local bindings
                for (i, param) in function.get_param_iter().enumerate() {
                    param.set_name(&arguments[i]);
                    let arg = codegen.variable_map.get(&arguments[i]).unwrap();
                    codegen.builder.build_store(*arg, param)?;
                }

                // Create blocks for all case statemnets
                for (label, switch) in codegen.switch_statements.clone() {
                    let cases = switch
                        .cases
                        .iter()
                        .map(|(l, v)| {
                            (
                                codegen.context.i64_type().const_int(*v as u64, false),
                                codegen.get_basic_block(l),
                            )
                        })
                        .collect();
                    codegen.switch_map.insert(label.clone(), cases);
                }

                block.codegen(codegen)?;

                let current_block = codegen.builder.get_insert_block().unwrap();
                if current_block.get_terminator().is_none() {
                    codegen.builder.build_unconditional_branch(return_block)?;
                };

                codegen.builder.position_at_end(return_block);
                let ret_val = codegen.builder.build_load(
                    i64_type,
                    codegen.return_value.expect("return value should exist"),
                    "fn_return",
                )?;
                codegen.builder.build_return(Some(&ret_val))?;
                Ok(None)
            }
            FunctionDeclaration::FunctionDeclaration(name, arguments, None) => {
                if codegen.functions.contains_key(name) {
                    return Ok(None);
                }
                let i64_type = codegen.context.i64_type();
                let mut args = Vec::new();
                for _ in arguments {
                    args.push(i64_type.into());
                }
                let fn_type = i64_type.fn_type(&args, false);
                let function = codegen.module.add_function(name, fn_type, None);
                codegen.functions.insert(name.clone(), function);
                Ok(None)
            }
        }
    }
}

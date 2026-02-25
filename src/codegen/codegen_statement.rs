use inkwell::{IntPredicate, builder::BuilderError, values::BasicValueEnum};

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
                codegen.builder.build_store(codegen.return_value.unwrap(), exp)?;
                codegen.builder.build_unconditional_branch(codegen.return_block.unwrap())?;
                let dead = codegen.get_basic_block("dead");
                codegen.builder.position_at_end(dead);
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
                    IntPredicate::NE,
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
                let merge_block = codegen
                    .context
                    .append_basic_block(current_fn, "merge_block");

                codegen
                    .builder
                    .build_conditional_branch(cmp, then_block, else_block)?;

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

                let current_block = codegen.builder.get_insert_block().unwrap();

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
            Statement::Compound(block) => {
                block.codegen(codegen)?;
            }
            Statement::While(cond, stmt, label) => {
                if let Some(label) = label {
                    let while_block = codegen.get_basic_block(&format!("{}_start", label));
                    let while_post = codegen.get_basic_block(&format!("{}_post", label));
                    let end_block = codegen.get_basic_block(&format!("{}_end", label));

                    let cond_val = cond
                        .codegen(codegen)?
                        .expect("condition to return a value")
                        .into_int_value();

                    let cond_cmp = codegen.builder.build_int_compare(
                        IntPredicate::NE,
                        cond_val,
                        cond_val.get_type().const_zero(),
                        "while_cond_cmp",
                    )?;

                    codegen
                        .builder
                        .build_conditional_branch(cond_cmp, while_block, end_block)?;
                    codegen.builder.position_at_end(while_block);

                    stmt.codegen(codegen)?;

                    codegen.builder.build_unconditional_branch(while_post)?;
                    codegen.builder.position_at_end(while_post);

                    let cond_val = cond
                        .codegen(codegen)?
                        .expect("condition to return a value")
                        .into_int_value();
                    let cond_cmp = codegen.builder.build_int_compare(
                        IntPredicate::NE,
                        cond_val,
                        cond_val.get_type().const_zero(),
                        "while_cond_cmp",
                    )?;
                    codegen
                        .builder
                        .build_conditional_branch(cond_cmp, while_block, end_block)?;
                    codegen.builder.position_at_end(end_block);
                } else {
                    panic!("Expected while to have a label");
                }
            }
            Statement::DoWhile(stmt, cond, label) => {
                if let Some(label) = label {
                    let do_while_block = codegen.get_basic_block(&format!("{}_start", label));
                    let do_while_post = codegen.get_basic_block(&format!("{}_post", label));
                    let end_block = codegen.get_basic_block(&format!("{}_end", label));

                    codegen.builder.build_unconditional_branch(do_while_block)?;
                    codegen.builder.position_at_end(do_while_block);

                    stmt.codegen(codegen)?;

                    codegen.builder.build_unconditional_branch(do_while_post)?;
                    codegen.builder.position_at_end(do_while_post);

                    let cond_val = cond
                        .codegen(codegen)?
                        .expect("condition to return a value")
                        .into_int_value();
                    let cond_cmp = codegen.builder.build_int_compare(
                        IntPredicate::NE,
                        cond_val,
                        cond_val.get_type().const_zero(),
                        "while_cond_cmp",
                    )?;

                    codegen.builder.build_conditional_branch(
                        cond_cmp,
                        do_while_block,
                        end_block,
                    )?;
                    codegen.builder.position_at_end(end_block);
                } else {
                    panic!("Expected do_while to have a label");
                }
            }
            Statement::For(init, cond, post, stmt, label) => {
                if let Some(label) = label {
                    init.codegen(codegen)?;

                    let for_block = codegen.get_basic_block(&format!("{}_start", label));
                    let for_post = codegen.get_basic_block(&format!("{}_post", label));
                    let end_block = codegen.get_basic_block(&format!("{}_end", label));

                    // Determine whether to enter the loop
                    let cond_cmp = match cond {
                        Some(cond) => {
                            let cond_val = cond
                                .codegen(codegen)?
                                .expect("condition to return a value")
                                .into_int_value();
                            codegen.builder.build_int_compare(
                                IntPredicate::NE,
                                cond_val,
                                cond_val.get_type().const_zero(),
                                "for_cond_cmp",
                            )?
                        }
                        None => codegen.context.bool_type().const_int(1, false),
                    };

                    codegen
                        .builder
                        .build_conditional_branch(cond_cmp, for_block, end_block)?;

                    // Loop Body
                    codegen.builder.position_at_end(for_block);
                    stmt.codegen(codegen)?;

                    // Post and check
                    codegen.builder.build_unconditional_branch(for_post)?;
                    codegen.builder.position_at_end(for_post);
                    if let Some(post) = post {
                        post.codegen(codegen)?;
                    }
                    let cond_cmp = match cond {
                        Some(cond) => {
                            let cond_val = cond
                                .codegen(codegen)?
                                .expect("condition to return a value")
                                .into_int_value();
                            codegen.builder.build_int_compare(
                                IntPredicate::NE,
                                cond_val,
                                cond_val.get_type().const_zero(),
                                "for_cond_cmp",
                            )?
                        }
                        None => codegen.context.bool_type().const_int(1, false),
                    };
                    codegen
                        .builder
                        .build_conditional_branch(cond_cmp, for_block, end_block)?;
                    codegen.builder.position_at_end(end_block);
                } else {
                    panic!("Expected for to have a label");
                }
            }
            Statement::Break(label) => {
                if let Some(label) = label {
                    let while_end = codegen.get_basic_block(&format!("{}_end", label));
                    codegen.builder.build_unconditional_branch(while_end)?;
                    let current_fn = codegen
                        .builder
                        .get_insert_block()
                        .unwrap()
                        .get_parent()
                        .unwrap();
                    let dead_block = codegen.context.append_basic_block(current_fn, "dead");
                    codegen.builder.position_at_end(dead_block);
                } else {
                    panic!("Expected while to have a label");
                }
            }
            Statement::Continue(label) => {
                if let Some(label) = label {
                    let while_start = codegen.get_basic_block(&format!("{}_post", label));
                    codegen.builder.build_unconditional_branch(while_start)?;
                    let current_fn = codegen
                        .builder
                        .get_insert_block()
                        .unwrap()
                        .get_parent()
                        .unwrap();
                    let dead_block = codegen.context.append_basic_block(current_fn, "dead");
                    codegen.builder.position_at_end(dead_block);
                } else {
                    panic!("Expected while to have a label");
                }
            }
            Statement::Switch(expr, stmt, label) => {
                let control = expr
                    .codegen(codegen)?
                    .expect("switch control to have a value")
                    .into_int_value();

                let default_block = codegen.get_basic_block(&format!(
                    "{}_default",
                    label.clone().expect("label should have a value")
                ));

                codegen.builder.build_switch(
                    control,
                    default_block,
                    codegen
                        .switch_map
                        .get(&label.clone().unwrap_or_default())
                        .expect("switch map to have cases"),
                )?;

                let current_fn = codegen
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap();

                let dead_block = codegen.context.append_basic_block(current_fn, "dead");
                codegen.builder.position_at_end(dead_block);

                stmt.codegen(codegen)?;

                let end_block = codegen.get_basic_block(&format!(
                    "{}_end",
                    label.clone().expect("label should have a value")
                ));

                // For fall through, ensure that the previous statement jumps to the end
                let current_block = codegen.builder.get_insert_block().unwrap();
                if current_block.get_terminator().is_none() {
                    codegen.builder.build_unconditional_branch(end_block)?;
                }

                // In the case that there was no defined default block, ensure it has a terminator
                if default_block.get_terminator().is_none() {
                    codegen.builder.position_at_end(default_block);
                    codegen.builder.build_unconditional_branch(end_block)?;

                }

                codegen.builder.position_at_end(end_block);
            }
            Statement::Case(_, stmt, label) => {
                let case_block =
                    codegen.get_basic_block(&label.clone().expect("label should have a value"));
                // For fall through, ensure that the previous statement jumps to this one
                let current_block = codegen.builder.get_insert_block().unwrap();
                if current_block.get_terminator().is_none() {
                    codegen.builder.build_unconditional_branch(case_block)?;
                }
                codegen.builder.position_at_end(case_block);
                stmt.codegen(codegen)?;
            }
            Statement::Default(stmt, label) => {
                let default_block =
                    codegen.get_basic_block(&label.clone().expect("label should have a value"));

                // For fall through, ensure that the previous statement jumps to this one
                let current_block = codegen.builder.get_insert_block().unwrap();
                if current_block.get_terminator().is_none() {
                    codegen.builder.build_unconditional_branch(default_block)?;
                }
                codegen.builder.position_at_end(default_block);
                stmt.codegen(codegen)?;
            }
        };
        Ok(None)
    }
}

use inkwell::{
    IntPredicate,
    builder::BuilderError,
    values::{BasicValue, BasicValueEnum, IntValue, PointerValue},
};

use crate::{
    codegen::codegen::{CodeGen, CodeGenerator},
    frontend::{
        ast::{ASTNode, Expression},
        tokens::Token,
    },
};


impl<'ctx> CodeGenerator<'ctx> for ASTNode<Expression> {
    fn codegen(
        &self,
        codegen: &mut CodeGen<'ctx>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        let i64_type = codegen.context.i64_type();

        match &self.node {
            Expression::IntLiteral(val) => Ok(Some(
                i64_type.const_int(*val as u64, false).as_basic_value_enum(),
            )),
            Expression::UnaryExpr(token, expression) => {
                if *token == Token::Increment || *token == Token::Decrement {
                    codegen.gen_l_value = true;
                }
                let expr = expression
                    .codegen(codegen)?
                    .expect("expression to have a value");
                codegen.gen_l_value = false;

                match expr {
                    BasicValueEnum::IntValue(int_value) => {
                        Ok(Some(build_int_unary_expr(codegen, token, int_value)?))
                    }
                    BasicValueEnum::PointerValue(ptr_value) => {
                        Ok(Some(build_ptr_unary_expr(codegen, token, ptr_value)?))
                    }
                    _ => panic!("Unsupported value in unary expr {}", expr),
                }
            }
            Expression::BinaryExpr(op, left, right) => {
                // Short circuiting requires generating one operand at a time
                if op.is_short_circuit() {
                    return Ok(Some(build_short_circuit(codegen, op, left, right)?));
                }

                let lhs = left.codegen(codegen)?.expect("lhs of binary expr to have a value");
                let rhs = right.codegen(codegen)?.expect("rhs of binary expr to have a value");
                let output = if let BasicValueEnum::IntValue(l) = lhs
                    && let BasicValueEnum::IntValue(r) = rhs
                {
                    build_int_int_binary_expr(codegen, op, l, r)?
                } else {
                    panic!(
                        "Invalid arguments to binary expr {:?} {} {:?}",
                        lhs, op, rhs
                    )
                };

                Ok(Some(output))
            }
            Expression::Variable(name) => {
                let pointer = codegen.variable_map.get(name).unwrap().clone();
                if codegen.gen_l_value {
                    return Ok(Some(pointer.as_basic_value_enum()))
                }
                Ok(Some(codegen
                    .builder
                    .build_load(codegen.context.i64_type(), pointer, name)?.as_basic_value_enum()))
            }
            Expression::Assignment(lhs, rhs) => {
                let val = rhs.codegen(codegen)?.expect("rhs of assignment to have a value");

                codegen.gen_l_value = true;
                let lhs_ptr = lhs.codegen(codegen)?.expect("lhs to have a value").into_pointer_value();
                codegen.gen_l_value = false;

                codegen
                    .builder
                    .build_store(lhs_ptr, val)?;
                Ok(Some(val.as_basic_value_enum()))
            }
        }
    }
}

fn build_int_unary_expr<'ctx>(
    codegen: &mut CodeGen<'ctx>,
    operator: &Token,
    expr: IntValue<'ctx>,
) -> Result<BasicValueEnum<'ctx>, BuilderError> {
    Ok(match operator {
        Token::Minus => codegen.builder.build_int_neg(expr, "name"),
        Token::BitwiseCompliment => codegen.builder.build_not(expr, "name"),
        Token::Not => {
            let result = codegen
                .builder
                .build_int_compare(IntPredicate::EQ, expr, expr.get_type().const_zero(), "cmp")
                .unwrap();
            codegen
                .builder
                .build_int_z_extend(result, expr.get_type(), "cmp_extended")
        }
        _ => panic!("Unexpected token in UnaryExpr {}", operator),
    }?
    .as_basic_value_enum())
}

fn build_ptr_unary_expr<'ctx>(
    codegen: &mut CodeGen<'ctx>,
    operator: &Token,
    expr: PointerValue<'ctx>,
) -> Result<BasicValueEnum<'ctx>, BuilderError> {
    // Load the value at the pointer into a temporary value
    let tmp = codegen
        .builder
        .build_load(codegen.context.i64_type(), expr, "inc_tmp")?;
    let one = tmp.get_type().into_int_type().const_int(1, false);

    if let BasicValueEnum::IntValue(iv) = tmp {
        // Perform the increment/decrement
        let inc = match operator {
            Token::Increment => codegen.builder.build_int_add(iv, one, "tmp")?,
            Token::Decrement => codegen.builder.build_int_sub(iv, one, "tmp")?,
            _ => panic!("Unexpected token in ptr unary operator {}", operator),
        };

        // Store the new value
        codegen.builder.build_store(expr, inc).unwrap();
    }

    // Return the old value since this is postfix increment
    Ok(tmp.as_basic_value_enum())
}

fn build_short_circuit<'ctx>(
    codegen: &mut CodeGen<'ctx>,
    operator: &Token,
    left: &ASTNode<Expression>,
    right: &ASTNode<Expression>,
) -> Result<BasicValueEnum<'ctx>, BuilderError> {
    let lhs = left
        .codegen(codegen)?
        .expect("short circuit lhs to have a value").into_int_value();

    let lhs_cmp = codegen.builder.build_int_compare(
        IntPredicate::NE,
        lhs,
        lhs.get_type().const_zero(),
        "lhs_cmp",
    )?;

    let current_fn = codegen
        .builder
        .get_insert_block()
        .unwrap()
        .get_parent()
        .unwrap();

    let rhs_block = codegen.context.append_basic_block(current_fn, "rhs_block");
    let merge_block = codegen
        .context
        .append_basic_block(current_fn, "merge_block");

    match operator {
        Token::LogicalAnd => {
            codegen
                .builder
                .build_conditional_branch(lhs_cmp, rhs_block, merge_block)?;
        }
        Token::LogicalOr => {
            codegen
                .builder
                .build_conditional_branch(lhs_cmp, merge_block, rhs_block)?;
        }
        _ => unreachable!(),
    };

    let start_block = codegen.builder.get_insert_block().unwrap();
    codegen.builder.position_at_end(rhs_block);

    let rhs = right
        .codegen(codegen)?
        .expect("short circuit rhs to have a value").into_int_value();

    let result = codegen.builder.build_int_compare(
        IntPredicate::NE,
        rhs,
        rhs.get_type().const_zero(),
        "lbool",
    )?;
    // Convert back to the type of the input
    let r = codegen
        .builder
        .build_int_z_extend(result, lhs.get_type(), "ExtendedResult")?;

    let current_rhs_block = codegen.builder.get_insert_block().unwrap();

    codegen.builder.build_unconditional_branch(merge_block)?;
    codegen.builder.position_at_end(merge_block);

    let phi = codegen
        .builder
        .build_phi(codegen.context.i64_type(), "logic_result")?;

    let phi_value = match operator {
        Token::LogicalAnd => codegen.context.i64_type().const_zero(),
        Token::LogicalOr => codegen.context.i64_type().const_int(1, false),
        _ => unreachable!(),
    };

    phi.add_incoming(&[(&phi_value, start_block), (&r, current_rhs_block)]);
    return Ok(phi.as_basic_value().as_basic_value_enum());
}

fn build_int_int_binary_expr<'ctx>(
    codegen: &mut CodeGen<'ctx>,
    operator: &Token,
    l: IntValue<'ctx>,
    r: IntValue<'ctx>,
) -> Result<BasicValueEnum<'ctx>, BuilderError> {
    let result = match operator {
        Token::Plus => codegen.builder.build_int_add(l, r, "add"),
        Token::Minus => codegen.builder.build_int_sub(l, r, "sub"),
        Token::Mul => codegen.builder.build_int_mul(l, r, "mul"),
        Token::Div => codegen.builder.build_int_signed_div(l, r, "div"),
        Token::Percent => codegen.builder.build_int_signed_rem(l, r, "div"),
        Token::BitwiseAnd => codegen.builder.build_and(l, r, "and"),
        Token::BitwiseOr => codegen.builder.build_or(l, r, "or"),
        Token::BitwiseXOR => codegen.builder.build_xor(l, r, "xor"),
        Token::LeftShift => codegen.builder.build_left_shift(l, r, "shift_left"),
        Token::RightShift => {
            codegen.builder.build_right_shift(l, r, true, "shift_right")
        }
        Token::LogicalEq => codegen.i_cmp_ze(IntPredicate::EQ, l, r, "equals"),
        Token::NotEqual => codegen.i_cmp_ze(IntPredicate::NE, l, r, "not_equals"),
        Token::GreaterThan => {
            codegen.i_cmp_ze(IntPredicate::SGT, l, r, "greater_than")
        }
        Token::LessThan => codegen.i_cmp_ze(IntPredicate::SLT, l, r, "less_than"),
        Token::GreaterThanEq => {
            codegen.i_cmp_ze(IntPredicate::SGE, l, r, "greater_than_eq")
        }
        Token::LessThanEq => {
            codegen.i_cmp_ze(IntPredicate::SLE, l, r, "less_than_eq")
        }
        _ => panic!("Invalid binary expression operator {}", operator),
    }?.as_basic_value_enum();

    Ok(result)
}

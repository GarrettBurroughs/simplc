use std::{collections::HashMap, panic};

use inkwell::{
    IntPredicate, OptimizationLevel,
    builder::Builder,
    context::Context,
    memory_buffer::MemoryBuffer,
    module::Module,
    support::LLVMString,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    values::{AnyValue, AnyValueEnum, BasicValue, BasicValueEnum, PointerValue},
};

use crate::frontend::{
    ast::{ASTNode, Block, Declaration, Expression, Function, Program, Statement},
    tokens::Token,
};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variable_map: HashMap<String, PointerValue<'ctx>>
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, program_name: &str) -> Self {
        CodeGen {
            context: context,
            module: context.create_module(program_name),
            builder: context.create_builder(),
            variable_map: HashMap::new(),
        }
    }

    pub fn run_codegen<T: CodeGenerator<'ctx>>(&mut self, generator: &T) {
        generator.codegen(self);
    }

    pub fn emit_ir(&self) -> String {
        self.module.to_string()
    }

    pub fn emit_assmebly(&self) -> Result<MemoryBuffer, LLVMString> {
        Target::initialize_all(&InitializationConfig::default());
        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple).unwrap();
        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                OptimizationLevel::None,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        return target_machine.write_to_memory_buffer(&self.module, FileType::Assembly);
    }
}

pub trait CodeGenerator<'ctx> {
    fn codegen(&self, codegen: &mut CodeGen<'ctx>) -> AnyValueEnum<'ctx>;
}

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Program> {
    fn codegen(&self, codegen: &mut CodeGen<'ctx>) -> AnyValueEnum<'ctx> {
        match &self.node {
            Program::Program(function) => function.codegen(codegen),
        }
    }
}

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Function> {
    fn codegen(&self, codegen: &mut CodeGen<'ctx>) -> AnyValueEnum<'ctx> {
        match &self.node {
            Function::Function(name, blocks) => {
                let i64_type = codegen.context.i64_type();
                let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);
                let function = codegen.module.add_function(name, fn_type, None);
                let basic_block = codegen.context.append_basic_block(function, "entry");
                codegen.builder.position_at_end(basic_block);

                for block in blocks {
                    block.codegen(codegen);
                }
                if let None = basic_block.get_terminator() {
                    codegen
                        .builder
                        .build_return(Some(&codegen.context.i64_type().const_zero()))
                        .unwrap()
                        .as_any_value_enum()
                } else {
                    codegen.context.i64_type().const_zero().as_any_value_enum()
                }

            }
        }
    }
}

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Block> {
    fn codegen(&self, codegen: &mut CodeGen<'ctx>) -> AnyValueEnum<'ctx> {
        match &self.node {
            Block::Declaration(decl) => {
                decl.codegen(codegen)
            },
            Block::Statement(statement) => statement.codegen(codegen),
        }
    }
}

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Declaration> {
    fn codegen(&self, codegen: &mut CodeGen<'ctx>) -> AnyValueEnum<'ctx> {
        match &self.node {
            Declaration::Declaration(name, astnode) => {
                let ptr = codegen.builder.build_alloca(codegen.context.i64_type(), name).unwrap();
                codegen.variable_map.insert(name.to_string(), ptr);

                if let Some(node) = astnode {
                    let val = node.codegen(codegen);
                    if let AnyValueEnum::IntValue(val) = val {
                        codegen.builder.build_store(ptr, val).unwrap().as_any_value_enum()
                    } else {
                        panic!("Attempting to assign a non int value");
                    }
                } else {
                    AnyValueEnum::IntValue(codegen.context.i64_type().const_zero())
                }
            }
        }
    }
}

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Statement> {
    fn codegen(&self, codegen: &mut CodeGen<'ctx>) -> AnyValueEnum<'ctx> {
        match &self.node {
            Statement::Return(expression) => {
                let exp = expression.codegen(codegen);

                let basic_value = match exp {
                    AnyValueEnum::IntValue(int_value) => int_value.as_basic_value_enum(),
                    _ => panic!("Expected integer return type got {}", exp.get_type()),
                };

                codegen
                    .builder
                    .build_return(Some(&basic_value))
                    .unwrap()
                    .as_any_value_enum()
            }
            Statement::Expression(expr) => {
                expr.codegen(codegen)
            }
            Statement::Null => {
                AnyValueEnum::IntValue(codegen.context.i64_type().const_zero())
            }
        }
    }
}

impl<'ctx> CodeGenerator<'ctx> for ASTNode<Expression> {
    fn codegen(&self, codegen: &mut CodeGen<'ctx>) -> AnyValueEnum<'ctx> {
        let i64_type = codegen.context.i64_type();
        match &self.node {
            Expression::IntLiteral(val) => {
                i64_type.const_int(*val as u64, false).as_any_value_enum()
            }
            Expression::UnaryExpr(token, expression) => {
                let expr = expression.codegen(codegen);
                let int_val = match expr {
                    AnyValueEnum::IntValue(int_value) => int_value,
                    _ => panic!("Invalid value in unary expr {}", expr),
                };
                match token {
                    Token::Minus => codegen.builder.build_int_neg(int_val, "name"),
                    Token::BitwiseCompliment => codegen.builder.build_not(int_val, "name"),
                    Token::Not => {
                        let result = codegen.builder.build_int_compare(IntPredicate::EQ, int_val, int_val.get_type().const_zero(), "cmp").unwrap();
                        codegen.builder.build_int_z_extend(result, int_val.get_type(), "cmp_extended")
                    },
                    Token::Increment => {
                        if let Expression::Variable(name) = &expression.node {
                            let ptr = codegen.variable_map.get(name).unwrap().clone();
                            let tmp = codegen.builder.build_load(int_val.get_type(), ptr, "inc_tmp").unwrap();
                            if let BasicValueEnum::IntValue(iv) = tmp {
                                let inc = codegen.builder.build_int_add(
                                    iv, 
                                    tmp.get_type().into_int_type().const_int(1, false),
                                "inc_add").unwrap();
                                codegen.builder.build_store(ptr, inc).unwrap();
                            }
                        }
                        Ok(int_val)
                    }
                    Token::Decrement => {
                        if let Expression::Variable(name) = &expression.node {
                            let ptr = codegen.variable_map.get(name).unwrap().clone();
                            let tmp = codegen.builder.build_load(int_val.get_type(), ptr, "inc_tmp").unwrap();
                            if let BasicValueEnum::IntValue(iv) = tmp {
                                let inc = codegen.builder.build_int_sub(
                                    iv, 
                                    tmp.get_type().into_int_type().const_int(1, false),
                                "inc_add").unwrap();
                                codegen.builder.build_store(ptr, inc).unwrap();
                            }
                        }
                        Ok(int_val)
                    }
                    _ => panic!("Unexpected token in UnaryExpr {}", token),
                }
                .unwrap()
                .as_any_value_enum()
            }
            Expression::BinaryExpr(op, left, right) => {
                if op.is_short_circuit() {
                    let lhs = left.codegen(codegen);
                    let l = match lhs {
                        AnyValueEnum::IntValue(l) => {
                            let result = codegen.builder.build_int_compare(IntPredicate::NE, l, l.get_type().const_zero(), "lbool").unwrap();
                            codegen.builder.build_int_z_extend(result, l.get_type(), "ExtendedResult").unwrap()
                        }
                        _ => panic!("Expected int for lhs of logical operator {}", op),
                    };

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

                    let is_true = codegen
                        .builder
                        .build_int_compare(IntPredicate::NE, l, codegen.context.i64_type().const_zero(), "is_true")
                        .unwrap();
                    match op {
                        Token::LogicalAnd => {
                            codegen
                                .builder
                                .build_conditional_branch(is_true, rhs_block, merge_block)
                                .unwrap();
                        }
                        Token::LogicalOr => {
                            codegen
                                .builder
                                .build_conditional_branch(is_true, merge_block, rhs_block)
                                .unwrap();
                        }
                        _ => unreachable!(),
                    };

                    let start_block = codegen.builder.get_insert_block().unwrap();
                    codegen.builder.position_at_end(rhs_block);
                    let rhs = right.codegen(codegen);
                    let r = match rhs {
                        AnyValueEnum::IntValue(r) => {
                            let result = codegen.builder.build_int_compare(IntPredicate::NE, r, r.get_type().const_zero(), "rbool").unwrap();
                            codegen.builder.build_int_z_extend(result, r.get_type(), "ResultExtended").unwrap()

                        }
                        _ => panic!("Expected int for lhs of logical operator {}", op),
                    };

                    let current_rhs_block = codegen.builder.get_insert_block().unwrap();

                    codegen
                        .builder
                        .build_unconditional_branch(merge_block)
                        .unwrap();
                    codegen.builder.position_at_end(merge_block);

                    let phi = codegen
                        .builder
                        .build_phi(codegen.context.i64_type(), "logic_result")
                        .unwrap();

                    let phi_value = match op {
                        Token::LogicalAnd => codegen.context.i64_type().const_zero(),
                        Token::LogicalOr => codegen.context.i64_type().const_int(1, false),
                        _ => unreachable!(),
                    };
                    phi.add_incoming(&[(&phi_value, start_block), (&r, current_rhs_block)]);
                    return phi.as_basic_value().as_any_value_enum();
                }

                let lhs = left.codegen(codegen);
                let rhs = right.codegen(codegen);
                let output = if let AnyValueEnum::IntValue(l) = lhs
                    && let AnyValueEnum::IntValue(r) = rhs
                {
                    match op {
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
                        },
                        Token::LogicalEq => {
                            let result = codegen
                                .builder
                                .build_int_compare(IntPredicate::EQ, l, r, "equals").unwrap();
                            codegen.builder.build_int_z_extend(result, l.get_type(), "ExtendedResult")
                        },
                        Token::NotEqual => {
                            let result = codegen
                                .builder
                                .build_int_compare(IntPredicate::NE, l, r, "not_equals").unwrap();
                            codegen.builder.build_int_z_extend(result, l.get_type(), "ExtendedResult")
                        },
                        Token::GreaterThan => {
                            let result = codegen
                                .builder
                                .build_int_compare(IntPredicate::SGT, l, r, "greater_than").unwrap();
                            codegen.builder.build_int_z_extend(result, l.get_type(), "ExtendedResult")
                        },
                        Token::LessThan => {
                            let result = codegen
                                .builder
                                .build_int_compare(IntPredicate::SLT, l, r, "less_than").unwrap();
                            codegen.builder.build_int_z_extend(result, l.get_type(), "ExtendedResult")
                        },
                        Token::GreaterThanEq => {
                            let result = codegen
                                .builder
                                .build_int_compare(IntPredicate::SGE, l, r, "greater_than_eq").unwrap();
                            codegen.builder.build_int_z_extend(result, l.get_type(), "ExtendedResult")
                        },
                        Token::LessThanEq => {
                            let result = codegen
                                .builder
                                .build_int_compare(IntPredicate::SLE, l, r, "less_than_eq").unwrap();
                            codegen.builder.build_int_z_extend(result, l.get_type(), "ExtendedResult")
                        },
                        _ => panic!("Invalid binary expression operator {}", op),
                    }
                } else {
                    panic!(
                        "Invalid arguments to binary expr {:?} {} {:?}",
                        lhs.get_type(), op, right
                    )
                }
                .unwrap()
                .as_any_value_enum();
                output
            }
            Expression::Variable(name) => {
                let pointer = codegen.variable_map.get(name).unwrap().clone();
                codegen.builder.build_load(codegen.context.i64_type(), pointer, name).unwrap().as_any_value_enum()
            },
            Expression::Assignment(lhs, rhs) => {
                let val = rhs.codegen(codegen);
                if let Expression::Variable(name) = &lhs.node && let AnyValueEnum::IntValue(val) = val {
                    let pointer = codegen.variable_map.get(name).unwrap().clone();
                    codegen.builder.build_store(pointer, val).unwrap().as_any_value_enum();
                    val.as_any_value_enum()
                } else {
                    panic!("Invailid L value");
                }
            },
        }
    }
}

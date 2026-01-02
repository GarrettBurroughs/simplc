use std::panic;

use inkwell::{
    OptimizationLevel,
    builder::Builder,
    context::Context,
    memory_buffer::MemoryBuffer,
    module::Module,
    support::LLVMString,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    values::{AnyValue, AnyValueEnum},
};

use crate::frontend::{
    ast::{Expression, Function, Program, Statement},
    lexer::Token,
};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, program_name: &str) -> Self {
        CodeGen {
            context: context,
            module: context.create_module(program_name),
            builder: context.create_builder(),
        }
    }

    pub fn run_codegen<T: CodeGenerator<'ctx>>(&self, generator: &T) {
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
    fn codegen(&self, codegen: &CodeGen<'ctx>) -> AnyValueEnum<'ctx>;
}

impl<'ctx> CodeGenerator<'ctx> for Program {
    fn codegen(&self, codegen: &CodeGen<'ctx>) -> AnyValueEnum<'ctx> {
        match self {
            Program::Program(function) => function.codegen(codegen),
        }
    }
}

impl<'ctx> CodeGenerator<'ctx> for Function {
    fn codegen(&self, codegen: &CodeGen<'ctx>) -> AnyValueEnum<'ctx> {
        match self {
            Function::Function(name, statement) => {
                let i64_type = codegen.context.i64_type();
                let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);
                let function = codegen.module.add_function(name, fn_type, None);
                let basic_block = codegen.context.append_basic_block(function, "entry");

                codegen.builder.position_at_end(basic_block);

                statement.codegen(codegen)
            }
        }
    }
}

impl<'ctx> CodeGenerator<'ctx> for Statement {
    fn codegen(&self, codegen: &CodeGen<'ctx>) -> AnyValueEnum<'ctx> {
        match self {
            Statement::Return(expression) => {
                let exp = expression.codegen(codegen);

                let basic_value = match exp {
                    AnyValueEnum::IntValue(int_value) => int_value,
                    _ => panic!("Expected integer return type"),
                };

                codegen
                    .builder
                    .build_return(Some(&basic_value))
                    .unwrap()
                    .as_any_value_enum()
            }
        }
    }
}

impl<'ctx> CodeGenerator<'ctx> for Expression {
    fn codegen(&self, codegen: &CodeGen<'ctx>) -> AnyValueEnum<'ctx> {
        let i64_type = codegen.context.i64_type();
        match self {
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
                    Token::Minus => codegen
                        .builder
                        .build_int_neg(int_val, "name")
                        .unwrap()
                        .as_any_value_enum(),
                    Token::BitwiseCompliment => codegen
                        .builder
                        .build_not(int_val, "name")
                        .unwrap()
                        .as_any_value_enum(),
                    _ => panic!("Unexpected token in UnaryExpr {}", token),
                }
            }
            Expression::BinaryExpr(op, left, right) => {
                let lhs = left.codegen(codegen);
                let rhs = right.codegen(codegen);
                if let AnyValueEnum::IntValue(l) = lhs && let AnyValueEnum::IntValue(r) = rhs {
                    match op {
                        Token::Plus => { codegen.builder.build_int_add(l, r, "add") },
                        Token::Minus => { codegen.builder.build_int_sub(l, r, "sub") },
                        Token::Mul => { codegen.builder.build_int_mul(l, r, "mul") },
                        Token::Div => { codegen.builder.build_int_signed_div(l, r, "div") },
                        Token::BitwiseAnd => { codegen.builder.build_and(l, r, "and") },
                        Token::BitwiseOr => { codegen.builder.build_or(l, r, "or") },
                        Token::BitwiseXOR => { codegen.builder.build_xor(l, r, "xor") },
                        Token::LeftShift => { codegen.builder.build_left_shift(l, r, "shift_left") },
                        Token::RightShift => { codegen.builder.build_right_shift(l, r, true,"shift_right") },
                        _ => panic!("Invalid binary expression operator {}", op)
                    }
                } else {
                    panic!("Invalid arguments to binary expr {:?} {} {:?}", left, op, right)
                }.unwrap().as_any_value_enum()
            },
        }
    }
}

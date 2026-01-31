use std::collections::{HashMap, HashSet};

use inkwell::{
    IntPredicate, OptimizationLevel,
    basic_block::BasicBlock,
    builder::{Builder, BuilderError},
    context::Context,
    memory_buffer::MemoryBuffer,
    module::Module,
    support::LLVMString,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    values::{BasicValueEnum, IntValue, PointerValue},
};

use crate::semantic::switch_collection::Switch;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub variables: HashSet<String>,
    pub variable_map: HashMap<String, PointerValue<'ctx>>,
    pub label_map: HashMap<String, BasicBlock<'ctx>>,
    pub gen_l_value: bool,
    pub switch_statements: HashMap<String, Switch>,
    pub switch_map: HashMap<String, Vec<(IntValue<'ctx>, BasicBlock<'ctx>)>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(
        context: &'ctx Context,
        program_name: &str,
        variables: HashSet<String>,
        switch_statements: HashMap<String, Switch>,
    ) -> Self {
        CodeGen {
            context: context,
            module: context.create_module(program_name),
            builder: context.create_builder(),
            variables,
            variable_map: HashMap::new(),
            label_map: HashMap::new(),
            switch_statements: switch_statements,
            switch_map: HashMap::new(),
            gen_l_value: false,
        }
    }

    pub fn run_codegen<T: CodeGenerator<'ctx>>(&mut self, generator: &T) {
        generator.codegen(self).unwrap();
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
                RelocMode::PIC,
                CodeModel::Default,
            )
            .unwrap();

        return target_machine.write_to_memory_buffer(&self.module, FileType::Assembly);
    }

    pub fn i_cmp_ze(
        &mut self,
        op: IntPredicate,
        lhs: IntValue<'ctx>,
        rhs: IntValue<'ctx>,
        name: &str,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        let result = self.builder.build_int_compare(op, lhs, rhs, name)?;
        let ext = self.builder.build_int_z_extend(
            result,
            lhs.get_type(),
            format!("{}_ext", &name).as_str(),
        );
        return ext;
    }

    pub fn get_basic_block(&mut self, name: &str) -> BasicBlock<'ctx> {
        let function = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();
        match self.label_map.get(name) {
            Some(bb) => *bb,
            None => {
                let bb = self.context.append_basic_block(function, name);
                self.label_map.insert(name.to_string(), bb);
                bb
            }
        }
    }
}

pub trait CodeGenerator<'ctx> {
    fn codegen(
        &self,
        codegen: &mut CodeGen<'ctx>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError>;
}

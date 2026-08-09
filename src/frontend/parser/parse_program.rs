use crate::{
    error::CompilerError,
    frontend::ast::{ASTNode, Program},
};

use super::Parser;

impl Parser {
    pub fn parse_program(&mut self) -> Result<ASTNode<Program>, CompilerError> {
        self.trace("Program");

        let mut fn_list = Vec::new();

        let mut loc = self.peek()?.span;
        while self.peek().is_ok() {
            let function = self.parse_function_decl()?;
            loc = loc.merge(&function.span);
            fn_list.push(function);
        }
        loc.build(Program::Program(fn_list))
    }
}

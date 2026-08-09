use crate::{error::CompilerError, frontend::tokens::Token};

use super::Parser;

impl Parser {
    pub(super) fn parse_unop(&mut self) -> Result<Token, CompilerError> {
        self.trace("Unary Operator");

        let operator = self.get_token()?.token;
        Ok(operator)
    }
}

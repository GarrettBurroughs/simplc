use crate::{
    error::CompilerError,
    frontend::{
        ast::{ASTNode, Expression},
        tokens::Token,
    },
};

use super::Parser;

impl Parser {
    pub(super) fn parse_optional_expr(
        &mut self,
    ) -> Result<Option<ASTNode<Expression>>, CompilerError> {
        self.trace("OptionalExpression");
        if self.peek()?.token == Token::Semicolon || self.peek()?.token == Token::CloseParen {
            Ok(None)
        } else {
            Ok(Some(self.parse_expr(0)?))
        }
    }
}

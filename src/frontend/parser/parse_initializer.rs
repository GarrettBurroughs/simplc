use crate::{
    error::CompilerError,
    frontend::{
        ast::{ASTNode, Initializer},
        tokens::Token,
    },
};

use super::Parser;

impl Parser {
    pub(super) fn parse_initializer(&mut self) -> Result<ASTNode<Initializer>, CompilerError> {
        self.trace("Initializer");

        let start = self.peek()?.span;

        match self.peek()?.token {
            Token::TypeInt => {
                let decl = self.parse_variable_decl()?;
                let span = start.merge(&decl.span);
                span.build(Initializer::Decl(decl))
            }
            Token::Semicolon => {
                let semicolon = self.get_token()?;
                let span = start.merge(&semicolon.span);
                span.build(Initializer::Exp(None))
            }
            _ => {
                let expr = self.parse_expr(0)?;
                let end = self.expect(Token::Semicolon)?;
                let span = start.merge(&end.span);
                span.build(Initializer::Exp(Some(expr)))
            }
        }
    }
}

use crate::{
    error::{CompilerError, ParseErrorKind},
    frontend::{
        ast::{ASTNode, VariableDeclaration},
        tokens::Token,
    },
    sourcemap::Span,
};

use super::Parser;

impl Parser {
    pub(super) fn parse_variable_decl(
        &mut self,
    ) -> Result<ASTNode<VariableDeclaration>, CompilerError> {
        self.trace("VariableDeclaration");

        let start = self.peek()?.span;
        self.expect(Token::TypeInt)?;
        let next_token = self.get_token()?;
        match next_token.token {
            Token::Identifier(ident) => self.parse_variable_decl_post(start, ident),
            _ => self.err(ParseErrorKind::Expected {
                got: next_token.token,
                expected: vec![Token::Identifier("Identifier".to_string())],
            }),
        }
    }

    pub(super) fn parse_variable_decl_post(
        &mut self,
        start: Span,
        ident: String,
    ) -> Result<ASTNode<VariableDeclaration>, CompilerError> {
        if let Ok(next_tok) = self.peek()
            && next_tok.token == Token::Equal
        {
            self.get_token()?;
            let expr = self.parse_expr(0)?;
            let end = self.expect(Token::Semicolon)?;
            let span = start.merge(&end.span);
            span.build(VariableDeclaration::VariableDeclaration(ident, Some(expr)))
        } else {
            let end = self.expect(Token::Semicolon)?;
            let span = start.merge(&end.span);
            span.build(VariableDeclaration::VariableDeclaration(ident, None))
        }
    }
}

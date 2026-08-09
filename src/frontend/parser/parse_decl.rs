use crate::{
    error::{CompilerError, ParseErrorKind},
    frontend::{
        ast::{ASTNode, Declaration},
        tokens::Token,
    },
};

use super::Parser;

impl Parser {
    pub(super) fn parse_decl(&mut self) -> Result<ASTNode<Declaration>, CompilerError> {
        self.trace("Declaration");

        let start = self.peek()?.span;
        self.expect(Token::TypeInt)?;

        let next_token = self.get_token()?;
        match next_token.token {
            Token::Identifier(ident) => {
                if let Ok(next_tok) = self.peek()
                    && next_tok.token == Token::OpenParen
                {
                    let fn_decl = self.parse_fn_decl_post(start, ident)?;
                    let span = start.merge(&fn_decl.span);
                    span.build(Declaration::FunctionDeclaration(fn_decl))
                } else {
                    let v_decl = self.parse_variable_decl_post(start, ident)?;
                    let span = start.merge(&v_decl.span);
                    span.build(Declaration::VariableDeclaration(v_decl))
                }
            }
            _ => self.err(ParseErrorKind::Expected {
                got: next_token.token,
                expected: vec![Token::Identifier("Identifier".to_string())],
            }),
        }
    }
}

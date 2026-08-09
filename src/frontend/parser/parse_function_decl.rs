use crate::{
    error::{CompilerError, ParseErrorKind},
    frontend::{
        ast::{ASTNode, FunctionDeclaration},
        tokens::Token,
    },
    sourcemap::Span,
};

use super::Parser;

impl Parser {
    pub(super) fn parse_function_decl(
        &mut self,
    ) -> Result<ASTNode<FunctionDeclaration>, CompilerError> {
        self.trace("FunctionDeclaration");

        let start = self.peek()?.span;
        self.expect(Token::TypeInt)?;

        let next_token = self.get_token()?.token;
        match next_token {
            Token::Identifier(ident) => self.parse_fn_decl_post(start, ident),
            _ => self.err(ParseErrorKind::Expected {
                got: next_token,
                expected: vec![Token::Identifier("Identifier".to_string())],
            }),
        }
    }

    pub(super) fn parse_fn_decl_post(
        &mut self,
        start: Span,
        ident: String,
    ) -> Result<ASTNode<FunctionDeclaration>, CompilerError> {
        self.expect(Token::OpenParen)?;
        let mut args = Vec::new();
        match self.peek()?.token {
            Token::TypeVoid => {
                self.get_token()?;
            }
            _ => {
                while let Token::TypeInt = self.peek()?.token {
                    self.get_token()?;
                    if let Token::Identifier(arg) =
                        self.expect(Token::Identifier(String::new()))?.token
                    {
                        args.push(arg);
                        if let Token::CloseParen = self.peek()?.token {
                            break;
                        }
                        self.expect(Token::Comma)?;
                        if let Token::CloseParen = self.peek()?.token {
                            self.err(ParseErrorKind::Expected {
                                got: Token::Comma,
                                expected: vec![Token::CloseParen],
                            })?;
                        }
                    }
                }
            }
        }
        self.expect(Token::CloseParen)?;
        if let Token::Semicolon = self.peek()?.token {
            let s = self.get_token()?;
            let span = start.merge(&s.span);

            return span.build(FunctionDeclaration::FunctionDeclaration(ident, args, None));
        }

        let body = self.parse_block()?;

        let span = start.merge(&body.span);
        span.build(FunctionDeclaration::FunctionDeclaration(
            ident,
            args,
            Some(body),
        ))
    }
}

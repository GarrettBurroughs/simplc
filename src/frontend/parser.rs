mod parse_block;
mod parse_decl;
mod parse_expr;
mod parse_factor;
mod parse_function_decl;
mod parse_initializer;
mod parse_optional_expr;
mod parse_program;
mod parse_statement;
mod parse_unop;
mod parse_variable_decl;

use std::{iter::Peekable, vec::IntoIter};

use log::trace;

use crate::error::ParseErrorKind;
use crate::frontend::ast::ASTNode;
use crate::sourcemap::Span;

use crate::{
    error::CompilerError,
    frontend::tokens::{Token, TokenLocation},
};

#[derive(Debug)]
pub struct Parser {
    pub(super) tokens: Peekable<IntoIter<TokenLocation>>,
}

impl Span {
    pub(super) fn build<T>(self, node: T) -> Result<ASTNode<T>, CompilerError> {
        Ok(ASTNode::new(node, self))
    }
}

impl Parser {
    pub fn new(tokens: Vec<TokenLocation>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub(super) fn err<T>(&mut self, kind: ParseErrorKind) -> Result<T, CompilerError> {
        let location = self
            .peek()
            .map(|t| t.span)
            .unwrap_or(Span { start: 0, end: 0 });
        Err(CompilerError::Parse {
            location: location.into(),
            kind,
        })
    }

    pub(super) fn precedence(tok: &Token) -> u32 {
        if tok.is_compound_assignment() {
            return 2;
        }
        match tok {
            Token::Equal => 2,
            Token::QuestionMark => 3,
            Token::LogicalOr => 4,
            Token::LogicalAnd => 5,
            Token::BitwiseOr => 6,
            Token::BitwiseXOR => 7,
            Token::BitwiseAnd => 8,
            Token::LogicalEq | Token::NotEqual => 9,
            Token::GreaterThanEq | Token::LessThanEq | Token::GreaterThan | Token::LessThan => 10,
            Token::LeftShift | Token::RightShift => 11,
            Token::Plus | Token::Minus => 12,
            Token::Div | Token::Mul | Token::Percent => 13,
            _ => panic!("Cannot take precedence of {}", tok),
        }
    }

    pub(super) fn expect(&mut self, expected: Token) -> Result<TokenLocation, CompilerError> {
        let t = self.get_token()?;

        if std::mem::discriminant(&t.token) == std::mem::discriminant(&expected) {
            Ok(t)
        } else {
            self.err(ParseErrorKind::Expected {
                got: t.token,
                expected: vec![expected],
            })
        }
    }

    pub(super) fn peek(&mut self) -> Result<&TokenLocation, CompilerError> {
        let err = CompilerError::Parse {
            location: Span { start: 0, end: 0 }.into(),
            kind: ParseErrorKind::Eof,
        };
        self.tokens.peek().ok_or(err)
    }

    pub(super) fn get_token(&mut self) -> Result<TokenLocation, CompilerError> {
        let tok = self
            .tokens
            .next()
            .ok_or(self.err::<&TokenLocation>(ParseErrorKind::Eof).unwrap_err())?;
        Ok(tok)
    }

    // Emits trace info when starting the parse of a new node
    pub(super) fn trace(&mut self, info: &str) {
        match self.peek() {
            Ok(tok) => trace!("Parsing {} AST Node at token: {}", info, tok),
            Err(err) => trace!("Error parsing {} AST Node at token: {}", info, err),
        }
    }
}

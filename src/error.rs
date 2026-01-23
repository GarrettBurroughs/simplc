use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::frontend::tokens::Token;

#[derive(Error, Debug, Diagnostic)]
pub enum CompilerError {
    #[error("Unexpected character: {character}")]
    #[diagnostic(code(lex::unsupported_char))]
    LexError {
        #[label("here")]
        location: SourceSpan,
        character: char,
    },

    #[error("Semantic error: {kind}")]
    #[diagnostic(code(lex::parse_error))]
    ParseError {
        #[label("here")]
        location: SourceSpan,
        kind: ParseErrorKind,
    },

    #[error("Parse Error: {kind}")]
    #[diagnostic(code(lex::semantic_error))]
    SemanticError {
        #[label("here")]
        location: SourceSpan,
        kind: SemanticErrorKind,
    },

    #[error("System error: {kind}")]
    #[diagnostic(code(lex::system_error))]
    SystemError { kind: SystemErrorKind },
}

#[derive(Error, Debug)]
pub enum ParseErrorKind {
    #[error("Unexpected token {got} expected: {expected:?}")]
    Expected { got: Token, expected: Vec<Token> },
    #[error("Unexpected end of file")]
    EOF,

    #[error("Unexpected end of file")]
    InvalidEOF(Token),

    #[error("Invalid Label")]
    InvalidLabel,
}

#[derive(Error, Debug)]
pub enum SemanticErrorKind {
    #[error("{0} declared multiple times in the same scope")]
    MultipleVariableDefinition(String),

    #[error("undeclared variable: {0}")]
    UndeclaredVariable(String),

    #[error("invalid assignment")]
    InvalidAssignment,

    #[error("attempted to declare already declared label: {0}")]
    AlreadyDeclaredLabel(String),

    #[error("use of undeclared label: {0}")]
    UndeclaredLabel(String),
}

#[derive(Error, Debug)]
pub enum SystemErrorKind {
    #[error("Cannot write to {0}")]
    FileWrite(String),

    #[error("Error generating assembly instructions")]
    AssemblyGeneration,
}

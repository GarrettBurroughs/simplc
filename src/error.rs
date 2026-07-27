use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::frontend::tokens::Token;

#[allow(unused)]
#[derive(Error, Debug, Diagnostic)]
pub enum CompilerError {
    #[error("Unexpected character: {character}")]
    #[diagnostic(code(lex::unsupported_char))]
    Lex {
        #[label("here")]
        location: SourceSpan,
        character: char,
    },

    #[error("Parse error: {kind}")]
    #[diagnostic(code(lex::parse_error))]
    Parse {
        #[label("here")]
        location: SourceSpan,
        kind: ParseErrorKind,
    },

    #[error("Semantic Error: {kind}")]
    #[diagnostic(code(lex::semantic_error))]
    Semantic {
        #[label("here")]
        location: SourceSpan,
        kind: SemanticErrorKind,
    },

    #[error("System error: {kind}")]
    #[diagnostic(code(lex::system_error))]
    System { kind: SystemErrorKind },
}

#[derive(Error, Debug)]
pub enum ParseErrorKind {
    #[error("Unexpected token {got} expected: {expected:?}")]
    Expected { got: Token, expected: Vec<Token> },
    #[error("Unexpected end of file")]
    Eof,

    // #[error("Unexpected end of file")]
    // InvalidEOF(Token),
    #[error("Invalid Label")]
    InvalidLabel,
}

#[derive(Error, Debug)]
pub enum SemanticErrorKind {
    #[error("{0} declared multiple times in the same scope")]
    MultipleVariableDefinition(String),

    #[error("undeclared variable: {0}")]
    UndeclaredVariable(String),

    #[error("attemptint to call undeclared function: {0}")]
    UndeclaredFunction(String),

    #[error("invalid assignment")]
    InvalidAssignment,

    #[error("attempted to declare already declared label: {0}")]
    AlreadyDeclaredLabel(String),

    #[error("use of undeclared label: {0}")]
    UndeclaredLabel(String),

    #[error("use of break outside of loop or switch statement")]
    InvalidBreak,

    #[error("use of continue out of loop")]
    InvalidContinue,

    #[error("use of case or default out of switch statement")]
    InvalidCase,

    #[error("use of duplicate case in switch statement")]
    DuplicateCase,
}

#[derive(Error, Debug)]
pub enum SystemErrorKind {
    #[error("Cannot write to {0}")]
    FileWrite(String),

    #[error("Error generating assembly instructions")]
    AssemblyGeneration,
}

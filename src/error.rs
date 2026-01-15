use thiserror::Error;

use crate::frontend::tokens::Token;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Lex Error at {location}: unexpected {character}")]
    LexError { location: Location, character: char },

    #[error("Parse Error at {location} {kind}")]
    ParseError {
        location: Location,
        kind: ParseErrorKind,
    },

    #[error("Parse Error at {location} {kind}")]
    SemanticError {
        location: Location,
        kind: SemanticErrorKind,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
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
    InvalidLabel
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

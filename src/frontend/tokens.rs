use std::fmt;

use crate::sourcemap::Span;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Identifier(String), // [a-zA-Z_]\w*\b
    IntLiteral(i32),    // [0-9]+\b

    // Keywords
    TypeInt,  // int
    TypeVoid, // void
    Return,   // return
    If,       // if
    Else,     // else
    Goto,     // goto
    Do,       // do
    While,    // while
    For,      // for
    Break,    // break
    Continue, // continue
    Switch,   // switch
    Case,     // case
    Default,  // default

    // Repeated tokens
    Increment,  // ++
    Decrement,  // --
    LogicalAnd, // &&
    LogicalOr,  // ||
    LeftShift,  // <<
    RightShift, // >>
    LogicalEq,  // ==

    Plus,        // +
    Minus,       // -
    BitwiseAnd,  // &
    BitwiseOr,   // |
    LessThan,    // <
    GreaterThan, // >
    Equal,       // =

    NotEqual,      // !=
    LessThanEq,    // <=
    GreaterThanEq, // >=

    PlusAssign,       // +=
    MinusAssign,      // -=
    MulAssign,        // *=
    DivAssign,        // /=
    ModAssign,        // %=
    AndAssign,        // &=
    OrAssign,         // |=
    XorAssign,        // ^=
    LeftShiftAssign,  // <<=
    RightShiftAssign, // >>=

    // Characters
    OpenParen,         // (
    CloseParen,        // )
    OpenBrace,         // {
    CloseBrace,        // }
    Semicolon,         // ;
    BitwiseCompliment, // ~
    Div,               // /
    Mul,               // *
    Percent,           // %
    BitwiseXOR,        // ^
    Not,               // !
    QuestionMark,      // ?
    Colon,             // :
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenLocation {
    pub token: Token,
    pub span: Span,
}

impl Token {
    // pub fn debug_string(&self) -> String {
    //     let string = self.to_string();
    //     return string.split('(').next().unwrap().to_string();
    // }

    pub fn is_binop(&self) -> bool {
        if self.is_compound_assignment() {
            return true;
        }
        match self {
            Token::Plus
            | Token::Minus
            | Token::Div
            | Token::Mul
            | Token::Percent
            | Token::BitwiseOr
            | Token::BitwiseAnd
            | Token::BitwiseXOR
            | Token::LeftShift
            | Token::RightShift
            | Token::LogicalEq
            | Token::NotEqual
            | Token::LogicalAnd
            | Token::LogicalOr
            | Token::GreaterThan
            | Token::LessThan
            | Token::GreaterThanEq
            | Token::LessThanEq
            | Token::Equal
            | Token::QuestionMark => true,
            _ => false,
        }
    }

    pub fn is_compound_assignment(&self) -> bool {
        match self {
            Token::PlusAssign
            | Token::MinusAssign
            | Token::MulAssign
            | Token::DivAssign
            | Token::ModAssign
            | Token::AndAssign
            | Token::OrAssign
            | Token::XorAssign
            | Token::LeftShiftAssign
            | Token::RightShiftAssign => true,
            _ => false,
        }
    }

    pub fn is_short_circuit(&self) -> bool {
        match self {
            Token::LogicalAnd | Token::LogicalOr => true,
            _ => false,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for TokenLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}:{}", self.token, self.span.start, self.span.end)
    }
}

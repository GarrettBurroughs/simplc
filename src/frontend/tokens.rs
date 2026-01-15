use std::fmt;

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

    PlusAssign, // +=
    MinusAssign, // -=
    MulAssign, // *=
    DivAssign, // /=
    ModAssign, // %=
    AndAssign, // &=
    OrAssign, // |=
    XorAssign, // ^=
    LeftShiftAssign, // <<=
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
    pub row: usize,
    pub column: usize,
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
            | Token::QuestionMark
            => true,
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
            | Token::RightShiftAssign
            => true, 
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
        match self {
            Token::Identifier(ident) => write!(f, "Identifier({})", ident),
            Token::IntLiteral(int) => write!(f, "IntLiteral({})", int),
            Token::TypeInt => write!(f, "TypeInt"),
            Token::TypeVoid => write!(f, "TypeVoid"),
            Token::Return => write!(f, "Return"),
            Token::OpenParen => write!(f, "OpenParen"),
            Token::CloseParen => write!(f, "CloseParen"),
            Token::OpenBrace => write!(f, "OpenBrace"),
            Token::CloseBrace => write!(f, "CloseBrace"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Increment => write!(f, "Increment"),
            Token::Decrement => write!(f, "Decrement"),
            Token::BitwiseCompliment => write!(f, "BitwiseCompliment"),
            Token::Minus => write!(f, "Minus"),
            Token::Plus => write!(f, "Plus"),
            Token::Div => write!(f, "Div"),
            Token::Mul => write!(f, "Mul"),
            Token::Percent => write!(f, "Percent"),
            Token::LogicalAnd => write!(f, "LogicalAnd"),
            Token::LogicalOr => write!(f, "LogicalOr"),
            Token::LeftShift => write!(f, "LeftShift"),
            Token::RightShift => write!(f, "RightShift"),
            Token::BitwiseAnd => write!(f, "BitwiseAnd"),
            Token::BitwiseOr => write!(f, "BitwiseOr"),
            Token::LessThan => write!(f, "LessThan"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::BitwiseXOR => write!(f, "BitwiseXOR"),
            Token::LogicalEq => write!(f, "LogicalEq"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::LessThanEq => write!(f, "LessThanEq"),
            Token::GreaterThanEq => write!(f, "GreaterThanEq"),
            Token::Not => write!(f, "Not"),
            Token::PlusAssign => write!(f, "PlusAssign"),
            Token::MinusAssign => write!(f, "MinusAssign"),
            Token::MulAssign => write!(f, "MulAssign"),
            Token::DivAssign => write!(f, "DivAssign"),
            Token::ModAssign => write!(f, "ModAssign"),
            Token::AndAssign => write!(f, "AndAssign"),
            Token::OrAssign => write!(f, "OrAssign"),
            Token::XorAssign => write!(f, "XorAssign"),
            Token::LeftShiftAssign => write!(f, "LeftShiftAssign"),
            Token::RightShiftAssign => write!(f, "RightShiftAssign"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::QuestionMark => write!(f, "QuestionMark"),
            Token::Colon => write!(f, "Colon"),
            Token::Goto => write!(f, "Goto"),
        }
    }
}

impl fmt::Display for TokenLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}:{}", self.token, self.row, self.column)
    }
}


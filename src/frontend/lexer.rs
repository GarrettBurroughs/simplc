use std::{fmt, iter::Peekable, str::Chars};

use crate::CompilerError;
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    row: usize,
    column: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Identifier(String), // [a-zA-Z_]\w*\b
    IntLiteral(i32),    // [0-9]+\b

    // Keywords
    TypeInt,  // int
    TypeVoid, // void
    Return,   // return

    // Repeated tokens
    Increment,  // ++
    Decrement,  // --
    LogicalAnd, // &&
    LogicalOr,  // ||
    LeftShift,  // <<
    RightShift, // >>

    Plus,        // +
    Minus,       // -
    BitwiseAnd,  // &
    BitwiseOr,   // |
    LessThan,    // <
    GreaterThan, //>

    Comment(String), // //\c*\n

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
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenLocation {
    pub token: Token,
    pub row: usize,
    pub column: usize,
}

impl Token {
    pub fn debug_string(&self) -> &str {
        match self {
            Token::Identifier(_) => "Identifier",
            Token::IntLiteral(_) => "IntLiteral",
            Token::TypeInt => "TypeInt",
            Token::TypeVoid => "TypeVoid",
            Token::Return => "Return",
            Token::OpenParen => "OpenParen",
            Token::CloseParen => "CloseParen",
            Token::OpenBrace => "OpenBrace",
            Token::CloseBrace => "CloseBrace",
            Token::Semicolon => "Semicolon",
            Token::Increment => "Increment",
            Token::Decrement => "Decrement",
            Token::BitwiseCompliment => "BitwiseCompliment",
            Token::Minus => "Minus",
            Token::Plus => "Plus",
            Token::Comment(_) => "Comment",
            Token::Div => "Div",
            Token::Mul => "Mul",
            Token::Percent => "Percent",
            Token::LogicalAnd => "LogicalAnd",
            Token::LogicalOr => "LogicalOr",
            Token::LeftShift => "LeftShift",
            Token::RightShift => "RightShift",
            Token::BitwiseAnd => "BitwiseAnd",
            Token::BitwiseOr => "BitwiseOr",
            Token::LessThan => "LessThan",
            Token::GreaterThan => "GreaterThan",
            Token::BitwiseXOR => "BitwiseXOR",
        }
    }

    pub fn is_binop(&self) -> bool {
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
            | Token::RightShift => true,
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
            Token::Comment(comment) => write!(f, "Comment({})", comment),
            Token::LogicalAnd => write!(f, "LogicalAnd"),
            Token::LogicalOr => write!(f, "LogicalOr"),
            Token::LeftShift => write!(f, "LeftShift"),
            Token::RightShift => write!(f, "RightShift"),
            Token::BitwiseAnd => write!(f, "BitwiseAnd"),
            Token::BitwiseOr => write!(f, "BitwiseOr"),
            Token::LessThan => write!(f, "LessThan"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::BitwiseXOR => write!(f, "BitwiseXOR"),
        }
    }
}

impl fmt::Display for TokenLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}:{}", self.token, self.row, self.column)
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        Self {
            chars: input.chars().peekable(),
            row: 0,
            column: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        if c == '\n' {
            self.row += 1;
            self.column = 0;
        } else {
            self.column += 1
        }
        Some(c)
    }

    fn lex_repeated(
        &mut self,
        first: char,
        second: char,
        first_tok: Token,
        second_tok: Token,
    ) -> Option<Token> {
        if let Some(&first_char) = self.chars.peek() {
            if first_char == first {
                self.next_char();
                if let Some(&second_char) = self.chars.peek() {
                    if second_char == second {
                        self.next_char();
                        return Some(second_tok);
                    }
                }
                return Some(first_tok);
            }
        }
        None
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<TokenLocation, CompilerError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }
        if let None = self.chars.peek() {
            return None;
        }

        let start_row = self.row;
        let start_column = self.column;

        // Keywords/Identifiers
        if let Some(&peeked_char) = self.chars.peek() {
            if peeked_char.is_alphabetic() || peeked_char == '_' {
                let mut ident = String::new();
                while let Some(&c) = self.chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(self.next_char().unwrap());
                    } else {
                        break;
                    }
                }

                let tok = match ident.as_str() {
                    "int" => Token::TypeInt,
                    "void" => Token::TypeVoid,
                    "return" => Token::Return,
                    _ => Token::Identifier(ident),
                };

                return Some(Ok(TokenLocation {
                    token: tok,
                    row: start_row,
                    column: start_column,
                }));
            }
        }

        // Comments
        if let Some(&peeked_char) = self.chars.peek() {
            if peeked_char == '/' {
                self.next_char();
                if let Some(&c) = self.chars.peek() {
                    if c == '/' {
                        // Lex comment until newline
                        self.next_char();
                        let mut comment_str = String::new();
                        while let Some(c) = self.next_char() {
                            if c == '\n' {
                                return Some(Ok(TokenLocation {
                                    token: Token::Comment(comment_str),
                                    row: start_row,
                                    column: start_column,
                                }));
                            }
                            comment_str.push(c);
                        }
                    } else {
                        return Some(Ok(TokenLocation {
                            token: Token::Div,
                            row: start_row,
                            column: start_column,
                        }));
                    }
                } else {
                    return Some(Ok(TokenLocation {
                        token: Token::Div,
                        row: start_row,
                        column: start_column,
                    }));
                }
            }
        }

        // Int literals
        if let Some(&peeked_char) = self.chars.peek() {
            if peeked_char.is_numeric() {
                let mut num_str = String::new();
                while let Some(&c) = self.chars.peek() {
                    if c.is_numeric() {
                        num_str.push(self.next_char().unwrap());
                    } else {
                        if let Some(&n) = self.chars.peek() {
                            if !(n.is_whitespace() || n == ';' || n == ')') {
                                return Some(Err(CompilerError::LexError(start_row, start_column)));
                            }
                        }
                        break;
                    }
                }

                let val = num_str.parse().unwrap();

                return Some(Ok(TokenLocation {
                    token: Token::IntLiteral(val),
                    row: start_row,
                    column: start_column,
                }));
            }
        }

        if let Some(tok) = self.lex_repeated('+', '+', Token::Plus, Token::Increment) {
            return Some(Ok(TokenLocation {
                token: tok,
                row: start_row,
                column: start_column,
            }));
        }

        if let Some(tok) = self.lex_repeated('-', '-', Token::Minus, Token::Decrement) {
            return Some(Ok(TokenLocation {
                token: tok,
                row: start_row,
                column: start_column,
            }));
        }

        if let Some(tok) = self.lex_repeated('&', '&', Token::BitwiseAnd, Token::LogicalAnd) {
            return Some(Ok(TokenLocation {
                token: tok,
                row: start_row,
                column: start_column,
            }));
        }

        if let Some(tok) = self.lex_repeated('|', '|', Token::BitwiseOr, Token::LogicalOr) {
            return Some(Ok(TokenLocation {
                token: tok,
                row: start_row,
                column: start_column,
            }));
        }

        if let Some(tok) = self.lex_repeated('>', '>', Token::GreaterThan, Token::RightShift) {
            return Some(Ok(TokenLocation {
                token: tok,
                row: start_row,
                column: start_column,
            }));
        }

        if let Some(tok) = self.lex_repeated('<', '<', Token::LessThan, Token::LeftShift) {
            return Some(Ok(TokenLocation {
                token: tok,
                row: start_row,
                column: start_column,
            }));
        }

        let tok = match self.next_char().unwrap() {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ';' => Token::Semicolon,
            '~' => Token::BitwiseCompliment,
            '*' => Token::Mul,
            '%' => Token::Percent,
            '^' => Token::BitwiseXOR,
            _ => return Some(Err(CompilerError::LexError(start_row, start_column))),
        };

        return Some(Ok(TokenLocation {
            token: tok,
            row: start_row,
            column: start_column,
        }));
    }
}

#[cfg(test)]
mod tests {
    // This brings all items from the outer scope into the test module
    use super::*;

    #[test]
    fn lex_full_program_suceeds() {
        let s = String::from("int main() {\n\treturn 0;\n}");
        let l = Lexer::new(&s);

        let tokens: Vec<_> = l.collect::<Result<Vec<_>, _>>().unwrap();

        let expected = vec![
            TokenLocation {
                token: Token::TypeInt,
                row: 0,
                column: 0,
            },
            TokenLocation {
                token: Token::Identifier(String::from("main")),
                row: 0,
                column: 4,
            },
            TokenLocation {
                token: Token::OpenParen,
                row: 0,
                column: 8,
            },
            TokenLocation {
                token: Token::CloseParen,
                row: 0,
                column: 9,
            },
            TokenLocation {
                token: Token::OpenBrace,
                row: 0,
                column: 11,
            },
            TokenLocation {
                token: Token::Return,
                row: 1,
                column: 1,
            },
            TokenLocation {
                token: Token::IntLiteral(0),
                row: 1,
                column: 8,
            },
            TokenLocation {
                token: Token::Semicolon,
                row: 1,
                column: 9,
            },
            TokenLocation {
                token: Token::CloseBrace,
                row: 2,
                column: 0,
            },
        ];
        assert_eq!(expected, tokens)
    }

    #[test]
    fn lex_should_error_invalid_identifier() {
        let s = String::from("int 0main() {\n\treturn 0;\n}");
        let l = Lexer::new(&s);

        let err: Result<Vec<_>, CompilerError> = l.collect::<Result<Vec<_>, _>>();
        assert_eq!(CompilerError::LexError(0, 4), err.unwrap_err())
    }

    #[test]
    fn lex_should_error_invalid_character() {
        let s = String::from("int main() {\n\treturn $;\n}");

        let l = Lexer::new(&s);

        let err: Result<Vec<_>, CompilerError> = l.collect::<Result<Vec<_>, _>>();
        assert_eq!(CompilerError::LexError(1, 8), err.unwrap_err())
    }
}

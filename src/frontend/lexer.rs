use log::trace;

use crate::frontend::tokens::Token;
use crate::frontend::tokens::TokenLocation;
use crate::sourcemap::SourceFile;
use crate::sourcemap::Span;
use std::{iter::Peekable, str::Chars};

use crate::error::CompilerError;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    position: usize,
    row: usize,
    column: usize,
    source: &'a SourceFile,
}

impl<'a> Lexer<'a> {
    fn err(&self, c: char) -> CompilerError {
        CompilerError::LexError {
            location: Span {
                start: self.position,
                end: self.position + 1,
            }.into(),
            character: c,
        }
    }

    pub fn new(input: &'a SourceFile) -> Self {
        Self {
            chars: input.contents.chars().peekable(),
            position: 0,
            row: 0,
            column: 0,
            source: input,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.position += 1;
        if c == '\n' {
            self.row += 1;
            self.column = 0;
        } else {
            self.column += 1
        }

        Some(c)
    }

    fn consume_if(&mut self, expected: char) -> bool {
        if let Some(&c) = self.chars.peek() {
            if c == expected {
                self.next_char();
                return true;
            }
        }
        return false;
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }
    }

    fn lex_token(&mut self) -> Result<Token, CompilerError> {
        let c = self.next_char().unwrap();

        // Keywords/Identifiers
        if c.is_alphabetic() || c == '_' {
            let mut ident = String::from(c);
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
                "if" => Token::If,
                "else" => Token::Else,
                "goto" => Token::Goto,
                _ => Token::Identifier(ident),
            };

            return Ok(tok);
        }

        // Int literals
        if c.is_numeric() {
            let mut num_str = String::from(c);
            while let Some(&c) = self.chars.peek() {
                if c.is_numeric() {
                    num_str.push(self.next_char().unwrap());
                } else {
                    if let Some(&n) = self.chars.peek() {
                        if n.is_alphabetic() {
                            return Err(self.err(n));
                        }
                    }
                    break;
                }
            }

            let val = num_str.parse().unwrap();

            return Ok(Token::IntLiteral(val));
        }

        match c {
            '+' => {
                if self.consume_if('+') {
                    return Ok(Token::Increment);
                }
                if self.consume_if('=') {
                    return Ok(Token::PlusAssign);
                }
                return Ok(Token::Plus);
            }
            '-' => {
                if self.consume_if('-') {
                    return Ok(Token::Decrement);
                }
                if self.consume_if('=') {
                    return Ok(Token::MinusAssign);
                }
                return Ok(Token::Minus);
            }
            '*' => {
                if self.consume_if('=') {
                    return Ok(Token::MulAssign);
                }
                return Ok(Token::Mul);
            }
            '/' => {
                if self.consume_if('=') {
                    return Ok(Token::DivAssign);
                }
                return Ok(Token::Div);
            }
            '%' => {
                if self.consume_if('=') {
                    return Ok(Token::ModAssign);
                }
                return Ok(Token::Percent);
            }
            '&' => {
                if self.consume_if('&') {
                    return Ok(Token::LogicalAnd);
                }
                if self.consume_if('=') {
                    return Ok(Token::AndAssign);
                }
                return Ok(Token::BitwiseAnd);
            }
            '|' => {
                if self.consume_if('|') {
                    return Ok(Token::LogicalOr);
                }
                if self.consume_if('=') {
                    return Ok(Token::OrAssign);
                }
                return Ok(Token::BitwiseOr);
            }
            '^' => {
                if self.consume_if('=') {
                    return Ok(Token::XorAssign);
                }
                return Ok(Token::BitwiseXOR);
            }
            '>' => {
                if self.consume_if('>') {
                    if self.consume_if('=') {
                        return Ok(Token::RightShiftAssign);
                    }
                    return Ok(Token::RightShift);
                }
                if self.consume_if('=') {
                    return Ok(Token::GreaterThanEq);
                }
                return Ok(Token::GreaterThan);
            }
            '<' => {
                if self.consume_if('<') {
                    if self.consume_if('=') {
                        return Ok(Token::LeftShiftAssign);
                    }
                    return Ok(Token::LeftShift);
                }
                if self.consume_if('=') {
                    return Ok(Token::LessThanEq);
                }
                return Ok(Token::LessThan);
            }
            '=' => {
                if self.consume_if('=') {
                    return Ok(Token::LogicalEq);
                }
                return Ok(Token::Equal);
            }
            '!' => {
                if self.consume_if('=') {
                    return Ok(Token::NotEqual);
                }
                return Ok(Token::Not);
            }
            _ => {}
        }

        let tok = match c {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ';' => Token::Semicolon,
            '~' => Token::BitwiseCompliment,
            '?' => Token::QuestionMark,
            ':' => Token::Colon,
            _ => return Err(self.err(c)),
        };
        return Ok(tok);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<TokenLocation, CompilerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        if let None = self.chars.peek() {
            return None;
        }

        let start = self.position;

        trace!("Lexing character {:?}", self.chars.peek());
        let tok_result = self.lex_token();
        let end = self.position;
        let sm_loc = self.source.lookup(start);
        trace!(
            "Got token {:?} at {}:{}",
            tok_result, sm_loc.row, sm_loc.column
        );

        match tok_result {
            Ok(token) => Some(Ok(TokenLocation {
                token,
                span: Span { start, end },
            })),
            Err(err) => Some(Err(err)),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn lex_full_program_suceeds() {
//         let s = String::from("int main() {\n\treturn 0;\n}");
//         let l = Lexer::new(&s);

//         let tokens: Vec<_> = l.collect::<Result<Vec<_>, _>>().unwrap();

//         let expected = vec![
//             TokenLocation {
//                 token: Token::TypeInt,
//                 row: 0,
//                 column: 0,
//             },
//             TokenLocation {
//                 token: Token::Identifier(String::from("main")),
//                 row: 0,
//                 column: 4,
//             },
//             TokenLocation {
//                 token: Token::OpenParen,
//                 row: 0,
//                 column: 8,
//             },
//             TokenLocation {
//                 token: Token::CloseParen,
//                 row: 0,
//                 column: 9,
//             },
//             TokenLocation {
//                 token: Token::OpenBrace,
//                 row: 0,
//                 column: 11,
//             },
//             TokenLocation {
//                 token: Token::Return,
//                 row: 1,
//                 column: 1,
//             },
//             TokenLocation {
//                 token: Token::IntLiteral(0),
//                 row: 1,
//                 column: 8,
//             },
//             TokenLocation {
//                 token: Token::Semicolon,
//                 row: 1,
//                 column: 9,
//             },
//             TokenLocation {
//                 token: Token::CloseBrace,
//                 row: 2,
//                 column: 0,
//             },
//         ];
//         assert_eq!(expected, tokens)
//     }

//     #[test]
//     fn lex_should_error_invalid_identifier() {
//         let s = String::from("int 0main() {\n\treturn 0;\n}");
//         let l = Lexer::new(&s);

//         let err: Result<Vec<_>, CompilerError> = l.collect::<Result<Vec<_>, _>>();
//         assert_eq!(CompilerError::LexError(0, 5), err.unwrap_err())
//     }

//     #[test]
//     fn lex_should_error_invalid_character() {
//         let s = String::from("int main() {\n\treturn $;\n}");

//         let l = Lexer::new(&s);

//         let err: Result<Vec<_>, CompilerError> = l.collect::<Result<Vec<_>, _>>();
//         assert_eq!(CompilerError::LexError(1, 9), err.unwrap_err())
//     }
// }

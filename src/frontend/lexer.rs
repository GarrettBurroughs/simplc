use std::fmt;

use crate::CompilerError;
pub struct Lexer { contents: Vec<char>,
    location: usize,
    row: usize,
    column: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // [a-zA-Z_]\w*\b
    Identifier(String),
    // [0-9]+\b
    IntLiteral(i32),

    // Keywords
    // int
    TypeInt,
    // void
    TypeVoid,
    // return
    Return,

    // Characters
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenLocation {
    token: Token,
    row: usize, 
    column: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(ident) => write!(f, "Identifier({})", ident),
            Token::IntLiteral(int) => write!(f, "IntLiteral({}", int),
            Token::TypeInt => write!(f, "TypeInt"),
            Token::TypeVoid => write!(f, "TypeVoid"),
            Token::Return => write!(f, "Return"),
            Token::OpenParen => write!(f, "OpenParen"),
            Token::CloseParen => write!(f, "CloseParen"),
            Token::OpenBrace => write!(f, "OpenBrace"),
            Token::CloseBrace => write!(f, "CloseBrace"),
            Token::Semicolon => write!(f, "Semicolon"),
        }
    }
}

impl fmt::Display for TokenLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}:{}", self.token, self.row, self.column)
    }
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            contents: input.chars().collect(),
            location: 0,
            row: 0,
            column: 0,
        }
    }

    pub fn has_next_token(&self) -> bool {
        self.location < self.contents.len()
    }

    fn increment_location(&mut self) {
        self.location += 1;
        self.column += 1;
    }

    pub fn get_next_token(&mut self) -> Result<TokenLocation, CompilerError> {
        while self.contents[self.location].is_whitespace() {
            if self.contents[self.location] == '\n' {
                self.column = 0;
                self.row += 1;
                self.location += 1;
            } else {
                self.increment_location();
            }

        }

        // Identifier/Keyword
        if self.contents[self.location].is_alphabetic() || self.contents[self.location] == '_' {
            let start = self.location;
            let start_column = self.column;
            while self.contents[self.location].is_alphanumeric()
                || self.contents[self.location] == '_'
            {
                self.increment_location();
            }
            let matched: String = self.contents[start..self.location].iter().collect();
            let tok = match matched.as_str() {
                "int" => Token::TypeInt,
                "void" => Token::TypeVoid,
                "return" => Token::Return,
                _ => Token::Identifier(matched),
            };
            return Ok(TokenLocation { token: tok, row: self.row, column: start_column})
        }

        if self.contents[self.location].is_numeric() {
            let start = self.location;
            let start_column = self.column;
            while self.contents[self.location].is_numeric() {
                self.increment_location();
            }

            // Ensure integer literals end at a line break
            if self.location + 1 < self.contents.len() {
                let trailing = self.contents[self.location + 1];
                if !(trailing.is_whitespace() || trailing == ';') {
                    return Err(CompilerError::LexError(self.row, start_column))
                }
            }

            let matched: String = self.contents[start..self.location].iter().collect();
            let tok = Token::IntLiteral(matched.parse::<i32>().unwrap());
            return Ok(TokenLocation { token: tok, row: self.row, column: start_column})
        }

        let tok = match self.contents[self.location] {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ';' => Token::Semicolon,
            _ => {
                let column = self.column;
                self.increment_location();  
                return Err(CompilerError::LexError(self.row, column))
            },
        };

        let row = self.row;
        let column = self.column;
        self.increment_location();
        return Ok(TokenLocation { token: tok, row: row, column: column})
    }
}


#[cfg(test)]
mod tests {
    // This brings all items from the outer scope into the test module
    use super::*;

    #[test]
    fn lex_full_program_suceeds() {
        let s = String::from("int main() {\n\treturn 0;\n}");
        let mut l = Lexer::new(s);
        let mut tokens: Vec<TokenLocation> = Vec::new();

        while l.has_next_token() {
            let tok = l.get_next_token().unwrap();
            tokens.push(tok);
        }

        let expected = vec![
            TokenLocation{ token: Token::TypeInt, row: 0, column: 0 },
            TokenLocation{ token: Token::Identifier(String::from("main")), row: 0, column: 4 },
            TokenLocation{ token: Token::OpenParen, row: 0, column: 8 },
            TokenLocation{ token: Token::CloseParen, row: 0, column: 9 },
            TokenLocation{ token: Token::OpenBrace, row: 0, column: 11 },
            TokenLocation{ token: Token::Return, row: 1, column: 1 },
            TokenLocation{ token: Token::IntLiteral(0), row: 1, column: 8 },
            TokenLocation{ token: Token::Semicolon, row: 1, column: 9 },
            TokenLocation{ token: Token::CloseBrace, row: 2, column: 0 },
        ];
        assert_eq!(expected, tokens)
    }

    #[test]
    fn lex_should_error_invalid_identifier() {
        let s = String::from("int 0main() {\n\treturn 0;\n}");
        let mut l = Lexer::new(s);

        while l.has_next_token() {
            let tok = l.get_next_token();
            match tok {
                Ok(_) => (),
                Err(err) => assert_eq!(CompilerError::LexError(0, 4), err),
            };
        }
    }

    #[test]
    fn lex_should_error_invalid_character() {
        let s = String::from("int main() {\n\treturn $;\n}");
        let mut l = Lexer::new(s);

        while l.has_next_token() {
            let tok = l.get_next_token();
            match tok {
                Ok(_) => (),
                Err(err) => assert_eq!(CompilerError::LexError(1, 8), err),
            };
        }
    }
}

use std::{iter::Peekable, vec::IntoIter};

use crate::{
    CompilerError,
    frontend::lexer::{Token, TokenLocation},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    IntLiteral(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Function {
    Function(String, Statement),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Program {
    Program(Function),
}

#[derive(Debug)]
pub struct AST {
    tokens: Peekable<IntoIter<TokenLocation>>,
}

impl AST {
    fn expect(&mut self, expected: Token) -> Result<TokenLocation, CompilerError> {
        let t = self.tokens.next().ok_or(CompilerError::ParseError(0, 0, "Unexpected EOF".into()))?;

        if std::mem::discriminant(&t.token) == std::mem::discriminant(&expected) {
            Ok(t)
        } else {
            Err(CompilerError::ParseError(
                t.row,
                t.column,
                expected.debug_string().to_string(),
            ))
        }
    }

    pub fn new(tokens: Vec<TokenLocation>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn peek(&mut self) -> Result<&TokenLocation, CompilerError> {
        self.tokens.peek().ok_or(CompilerError::ParseError(0, 0, "Unexpected EOF".into()))

    }

    fn get_token(&mut self) -> Result<TokenLocation, CompilerError> {
        self.tokens.next().ok_or(CompilerError::ParseError(0, 0, "Unexpected EOF".into()))
    }

    pub fn parse_program(&mut self) -> Result<Program, CompilerError> {
        let function = self.parse_function()?;
        return Ok(Program::Program(function));
    }

    fn parse_function(&mut self) -> Result<Function, CompilerError> {
        self.expect(Token::TypeInt)?;
        let identifier = self.expect(Token::Identifier(String::new()))?;
        let ident = match &identifier.token {
            Token::Identifier(ident) => ident.clone(),
            _ => panic!(""),
        };
        self.expect(Token::OpenParen)?;
        self.expect(Token::TypeVoid)?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;

        let statement = self.parse_statement()?;

        self.expect(Token::CloseBrace)?;

        return Ok(Function::Function(ident, statement));
    }

    fn parse_statement(&mut self) -> Result<Statement, CompilerError> {
        self.expect(Token::Return)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        return Ok(Statement::Return(expr));
    }

    fn parse_expr(&mut self) -> Result<Expression, CompilerError> {
        let loc = self.get_token()?;
        let val = match loc.token {
            Token::IntLiteral(val) => val,
            _ => {
                return Err(CompilerError::ParseError(
                    loc.row,
                    loc.column,
                    Token::IntLiteral(0).debug_string().to_string(),
                ));
            }
        };
        return Ok(Expression::IntLiteral(val));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ast_suceeds() {
        let tokens = vec![
            TokenLocation{ token: Token::TypeInt, row: 0, column: 0 },
            TokenLocation{ token: Token::Identifier(String::from("main")), row: 0, column: 4 },
            TokenLocation{ token: Token::OpenParen, row: 0, column: 8 },
            TokenLocation{ token: Token::TypeVoid, row: 0, column: 0},
            TokenLocation{ token: Token::CloseParen, row: 0, column: 9 },
            TokenLocation{ token: Token::OpenBrace, row: 0, column: 11 },
            TokenLocation{ token: Token::Return, row: 1, column: 1 },
            TokenLocation{ token: Token::IntLiteral(0), row: 1, column: 8 },
            TokenLocation{ token: Token::Semicolon, row: 1, column: 9 },
            TokenLocation{ token: Token::CloseBrace, row: 2, column: 0 },
        ];
        let expected_tree = Program::Program(
            Function::Function(
                String::from("main"),
                Statement::Return(
                    Expression::IntLiteral(0)
                )
            )
        );
        let mut ast = AST::new(tokens);

        let parsed_tree = ast.parse_program().unwrap();

        assert_eq!(expected_tree, parsed_tree);
    }

}

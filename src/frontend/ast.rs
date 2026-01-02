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
    UnaryExpr(Token, Box<Expression>),
    BinaryExpr(Token, Box<Expression>, Box<Expression>),
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

    fn precedence(tok: &Token) -> u32 {
        match tok {
            Token::LogicalOr => 4,
            Token::LogicalAnd => 5,
            Token::BitwiseOr => 6,
            Token::BitwiseXOR => 7,
            Token::BitwiseAnd => 8,
            Token::LeftShift | Token::RightShift => 11,
            Token::Plus | Token::Minus => 12,
            Token::Div | Token::Mul | Token::Percent => 13,
            _ => panic!("Cannot take precedence of {}", tok),
        }

    }

    fn expect(&mut self, expected: Token) -> Result<TokenLocation, CompilerError> {
        let t = self.get_token()?;

        if std::mem::discriminant(&t.token) == std::mem::discriminant(&expected) {
            Ok(t)
        } else {

            Err(CompilerError::ParseError(t.row, t.column, format!("Unexpected token {} expected {}", t.token, expected.debug_string())))
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
        let tok = self.tokens.next().ok_or(CompilerError::ParseError(0, 0, "Unexpected EOF".into()))?;
        if let Token::Comment(_) = tok.token {
            return self.get_token();
        }
        Ok(tok)
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
        let expr = self.parse_expr(0)?;
        self.expect(Token::Semicolon)?;
        return Ok(Statement::Return(expr));
    }

    fn parse_factor(&mut self) -> Result<Expression, CompilerError> {
        let next_token = self.get_token()?;
        match next_token.token {
            Token::IntLiteral(val) => {
                Ok(Expression::IntLiteral(val))
            },
            Token::Minus | Token::BitwiseCompliment => {
                let expr = self.parse_factor()?;
                Ok(Expression::UnaryExpr(next_token.token.clone(), Box::new(expr)))
            },
            Token::OpenParen => {
                let inner_expr = self.parse_expr(0);
                self.expect(Token::CloseParen)?;
                inner_expr
            }
            _ => Err(CompilerError::ParseError(next_token.row, next_token.column, format!("Unexpected token {}", next_token.token)))
        }
    }

    fn parse_expr(&mut self, min_precedence: u32) -> Result<Expression, CompilerError> {
        let mut left = self.parse_factor()?;
        while let Ok(tok) = self.peek() {
            if !tok.token.is_binop() {
                break;
            }
            let next_precedence = AST::precedence(&tok.token);
            if next_precedence < min_precedence {
                break
            }
            let operator = self.parse_unop()?;
            let right = self.parse_expr(next_precedence + 1)?;
            left = Expression::BinaryExpr(operator, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_unop(&mut self) -> Result<Token, CompilerError> {
        let operator = self.get_token()?.token;
        Ok(operator)
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

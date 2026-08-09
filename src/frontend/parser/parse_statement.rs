use crate::{
    error::{CompilerError, ParseErrorKind},
    frontend::{
        ast::{ASTNode, Expression, Statement},
        tokens::Token,
    },
};

use super::Parser;

impl Parser {
    pub(super) fn parse_statement(&mut self) -> Result<ASTNode<Statement>, CompilerError> {
        self.trace("Statement");

        let start = self.peek()?.span;

        match &self.peek()?.token {
            Token::Return => {
                self.get_token()?;
                let expr = self.parse_expr(0)?;
                let end = self.expect(Token::Semicolon)?;
                let span = start.merge(&end.span);
                span.build(Statement::Return(expr))
            }
            Token::If => {
                self.get_token()?;
                self.expect(Token::OpenParen)?;
                let expr = self.parse_expr(0)?;
                self.expect(Token::CloseParen)?;
                let stmt = self.parse_statement()?;
                if let Token::Else = self.peek()?.token {
                    self.get_token()?;
                    let else_stmt = self.parse_statement()?;
                    let span = start.merge(&else_stmt.span);
                    return span.build(Statement::If(
                        expr,
                        Box::new(stmt),
                        Some(Box::new(else_stmt)),
                    ));
                }
                let span = start.merge(&stmt.span);
                span.build(Statement::If(expr, Box::new(stmt), None))
            }
            Token::Semicolon => {
                self.get_token()?;
                start.build(Statement::Null)
            }
            Token::OpenBrace => {
                let block = self.parse_block()?;
                let span = start.merge(&block.span);
                span.build(Statement::Compound(block))
            }
            Token::Identifier(_) => {
                let expr = self.parse_expr(0)?;
                if self.peek()?.token == Token::Colon {
                    // A single "variable" is the same as an identifier
                    if let Expression::Variable(name) = &expr.node {
                        // Consume the ":"
                        self.get_token()?;
                        let stmt = self.parse_statement()?;
                        let span = start.merge(&stmt.span);
                        return span.build(Statement::Label(name.to_string(), Box::new(stmt)));
                    } else {
                        // If the expression is more than a single variable, this is invalid
                        return self.err(ParseErrorKind::InvalidLabel);
                    }
                }
                let end = self.expect(Token::Semicolon)?;
                let span = start.merge(&end.span);
                span.build(Statement::Expression(expr))
            }
            Token::Goto => {
                self.get_token()?;
                let ident = self.get_token()?;
                if let Token::Identifier(label) = ident.token {
                    let end = self.expect(Token::Semicolon)?;
                    let span = start.merge(&end.span);
                    span.build(Statement::Goto(label))
                } else {
                    self.err(ParseErrorKind::InvalidLabel)
                }
            }
            Token::Do => {
                self.get_token()?;
                let stmt = self.parse_statement()?;
                self.expect(Token::While)?;
                self.expect(Token::OpenParen)?;
                let expr = self.parse_expr(0)?;
                self.expect(Token::CloseParen)?;
                let end = self.expect(Token::Semicolon)?;
                let span = start.merge(&end.span);
                span.build(Statement::DoWhile(Box::new(stmt), expr, None))
            }
            Token::While => {
                self.get_token()?;
                self.expect(Token::OpenParen)?;
                let expr = self.parse_expr(0)?;
                self.expect(Token::CloseParen)?;
                let stmt = self.parse_statement()?;
                let span = start.merge(&stmt.span);
                span.build(Statement::While(expr, Box::new(stmt), None))
            }
            Token::For => {
                self.get_token()?;
                self.expect(Token::OpenParen)?;
                let initializer = self.parse_initializer()?;
                let cond = self.parse_optional_expr()?;
                self.expect(Token::Semicolon)?;
                let post = self.parse_optional_expr()?;
                self.expect(Token::CloseParen)?;
                let stmt = self.parse_statement()?;
                let span = start.merge(&stmt.span);
                span.build(Statement::For(
                    initializer,
                    cond,
                    post,
                    Box::new(stmt),
                    None,
                ))
            }
            Token::Switch => {
                self.get_token()?;
                self.expect(Token::OpenParen)?;
                let expr = self.parse_expr(0)?;
                self.expect(Token::CloseParen)?;
                let stmt = self.parse_statement()?;
                let span = start.merge(&stmt.span);
                span.build(Statement::Switch(expr, Box::new(stmt), None))
            }
            Token::Case => {
                self.get_token()?;
                let expr = self.parse_expr(0)?;
                self.expect(Token::Colon)?;
                let stmt = self.parse_statement()?;
                let span = start.merge(&stmt.span);
                span.build(Statement::Case(expr, Box::new(stmt), None))
            }
            Token::Default => {
                self.get_token()?;
                self.expect(Token::Colon)?;
                let stmt = self.parse_statement()?;
                let span = start.merge(&stmt.span);
                span.build(Statement::Default(Box::new(stmt), None))
            }
            Token::Break => {
                self.get_token()?;
                let end = self.expect(Token::Semicolon)?;
                let span = start.merge(&end.span);
                span.build(Statement::Break(None))
            }
            Token::Continue => {
                self.get_token()?;
                let end = self.expect(Token::Semicolon)?;
                let span = start.merge(&end.span);
                span.build(Statement::Continue(None))
            }
            _ => {
                let expr = self.parse_expr(0)?;
                let end = self.expect(Token::Semicolon)?;
                let span = start.merge(&end.span);
                span.build(Statement::Expression(expr))
            }
        }
    }
}

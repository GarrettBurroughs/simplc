use crate::{
    error::{CompilerError, ParseErrorKind},
    frontend::{
        ast::{ASTNode, Expression},
        tokens::Token,
    },
};

use super::Parser;

impl Parser {
    pub(super) fn parse_factor(&mut self) -> Result<ASTNode<Expression>, CompilerError> {
        self.trace("Factor");

        let start = self.peek()?.span;
        let next_token = self.get_token()?;
        let factor = match next_token.token {
            Token::IntLiteral(val) => start.build(Expression::IntLiteral(val)),
            Token::Minus | Token::BitwiseCompliment | Token::Not => {
                let expr = self.parse_factor()?;
                let span = start.merge(&expr.span);
                span.build(Expression::UnaryExpr(
                    next_token.token.clone(),
                    Box::new(expr),
                ))
            }
            Token::Increment | Token::Decrement => {
                let expr = self.parse_factor()?;
                let op = match next_token.token {
                    Token::Increment => Token::Plus,
                    Token::Decrement => Token::Minus,
                    _ => unreachable!(),
                };
                let span = start.merge(&expr.span);
                let transform = span.build(Expression::BinaryExpr(
                    op,
                    Box::new(expr.clone()),
                    Box::new(span.build(Expression::IntLiteral(1))?),
                ))?;
                span.build(Expression::Assignment(Box::new(expr), Box::new(transform)))
            }
            Token::OpenParen => {
                let inner_expr = self.parse_expr(0);
                self.expect(Token::CloseParen)?;
                inner_expr
            }
            Token::Identifier(ident) => {
                match self.peek()?.token {
                    Token::OpenParen => {
                        self.get_token()?; // Consume open paren
                        let mut args = Vec::new();
                        let mut span = start;

                        if let Token::CloseParen = self.peek()?.token {
                            let end = self.get_token()?;
                            span = span.merge(&end.span);
                            return span.build(Expression::FunctionCall(ident, args));
                        }

                        let expr = self.parse_expr(0)?;
                        span = span.merge(&expr.span);
                        args.push(expr);
                        while self.peek()?.token == Token::Comma {
                            self.get_token()?; // Consume comma

                            let expr = self.parse_expr(0)?;
                            span = span.merge(&expr.span);
                            args.push(expr);
                        }
                        self.expect(Token::CloseParen)?;
                        span.build(Expression::FunctionCall(ident, args))
                    }
                    _ => start.build(Expression::Variable(ident)),
                }
            }

            _ => self.err(ParseErrorKind::Expected {
                got: next_token.token,
                expected: vec![
                    Token::IntLiteral(0),
                    Token::Minus,
                    Token::Not,
                    Token::OpenParen,
                    Token::Increment,
                    Token::Decrement,
                    Token::BitwiseCompliment,
                    Token::Identifier("Identifier".to_string()),
                ],
            }),
        };

        if let Ok(next) = self.peek()
            && let Ok(factor) = &factor
            && (next.token == Token::Increment || next.token == Token::Decrement)
        {
            let start = self.peek()?.span;
            let next = self.get_token()?;
            let span = start.merge(&next.span);
            return span.build(Expression::UnaryExpr(next.token, Box::new(factor.clone())));
        }
        factor
    }
}

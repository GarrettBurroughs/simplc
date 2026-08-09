use log::trace;

use crate::{
    error::CompilerError,
    frontend::{
        ast::{ASTNode, Expression},
        tokens::Token,
    },
};

use super::Parser;

impl Parser {
    pub(super) fn parse_expr(
        &mut self,
        min_precedence: u32,
    ) -> Result<ASTNode<Expression>, CompilerError> {
        self.trace("Expression");
        match self.peek() {
            Ok(tok) => trace!("Parsing Expression AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Expression AST Node at token: {}", err),
        }
        let start = self.peek()?.span;
        let mut left = self.parse_factor()?;
        while let Ok(tok) = self.peek() {
            if !tok.token.is_binop() {
                break;
            }
            let next_precedence = Parser::precedence(&tok.token);
            if next_precedence < min_precedence {
                break;
            }
            let operator = self.parse_unop()?;
            left = if operator == Token::Equal {
                let right = self.parse_expr(next_precedence)?;
                let span = start.merge(&right.span);
                span.build(Expression::Assignment(Box::new(left), Box::new(right)))?
            } else if operator.is_compound_assignment() {
                let compound_operator = match operator {
                    Token::PlusAssign => Token::Plus,
                    Token::MinusAssign => Token::Minus,
                    Token::MulAssign => Token::Mul,
                    Token::DivAssign => Token::Div,
                    Token::ModAssign => Token::Percent,
                    Token::AndAssign => Token::BitwiseAnd,
                    Token::OrAssign => Token::BitwiseOr,
                    Token::XorAssign => Token::BitwiseXOR,
                    Token::LeftShiftAssign => Token::LeftShift,
                    Token::RightShiftAssign => Token::RightShift,
                    _ => unreachable!(),
                };

                let right = self.parse_expr(next_precedence)?;
                let span = start.merge(&right.span);
                let addition = span.build(Expression::BinaryExpr(
                    compound_operator,
                    Box::new(left.clone()),
                    Box::new(right.clone()),
                ))?;

                span.build(Expression::Assignment(Box::new(left), Box::new(addition)))?
            } else if operator == Token::QuestionMark {
                let middle = self.parse_expr(0)?;
                self.expect(Token::Colon)?;
                let right = self.parse_expr(next_precedence)?;
                let span = start.merge(&right.span);
                span.build(Expression::Ternary(
                    Box::new(left),
                    Box::new(middle),
                    Box::new(right),
                ))?
            } else {
                let right = self.parse_expr(next_precedence + 1)?;
                let span = start.merge(&right.span);
                span.build(Expression::BinaryExpr(
                    operator,
                    Box::new(left),
                    Box::new(right),
                ))?
            };
        }
        Ok(left)
    }
}

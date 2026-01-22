use log::trace;

use crate::error::ParseErrorKind;
use crate::frontend::ast::ASTNode;
use crate::frontend::ast::Block;
use crate::frontend::ast::Declaration;
use crate::frontend::ast::Expression;
use crate::frontend::ast::Function;
use crate::frontend::ast::Program;
use crate::frontend::ast::Statement;
use crate::sourcemap::Span;
use std::{iter::Peekable, vec::IntoIter};

use crate::{
    error::CompilerError,
    frontend::tokens::{Token, TokenLocation},
};

#[derive(Debug)]
pub struct Parser {
    tokens: Peekable<IntoIter<TokenLocation>>,
}

impl Span {
    fn build<T>(self, node: T) -> Result<ASTNode<T>, CompilerError> {
        Ok(ASTNode::new(node, self))
    }
}

impl Parser {
    pub fn new(tokens: Vec<TokenLocation>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn err<T>(&mut self, kind: ParseErrorKind) -> Result<T, CompilerError> {
        let location = self
            .peek()
            .map(|t| t.span)
            .unwrap_or(Span { start: 0, end: 0 });
        Err(CompilerError::ParseError { location, kind })
    }

    fn precedence(tok: &Token) -> u32 {
        if tok.is_compound_assignment() {
            return 2;
        }
        match tok {
            Token::Equal => 2,
            Token::QuestionMark => 3,
            Token::LogicalOr => 4,
            Token::LogicalAnd => 5,
            Token::BitwiseOr => 6,
            Token::BitwiseXOR => 7,
            Token::BitwiseAnd => 8,
            Token::LogicalEq | Token::NotEqual => 9,
            Token::GreaterThanEq | Token::LessThanEq | Token::GreaterThan | Token::LessThan => 10,
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
            self.err(ParseErrorKind::Expected {
                got: t.token,
                expected: vec![expected],
            })
        }
    }

    fn peek(&mut self) -> Result<&TokenLocation, CompilerError> {
        let err = CompilerError::ParseError {
            location: Span { start: 0, end: 0 },
            kind: ParseErrorKind::EOF,
        };
        self.tokens.peek().ok_or(err)
    }

    fn get_token(&mut self) -> Result<TokenLocation, CompilerError> {
        let tok = self
            .tokens
            .next()
            .ok_or(self.err::<&TokenLocation>(ParseErrorKind::EOF).unwrap_err())?;
        Ok(tok)
    }

    pub fn parse_program(&mut self) -> Result<ASTNode<Program>, CompilerError> {
        match self.peek() {
            Ok(tok) => trace!("Parsing Program AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Program AST Node at token: {}", err),
        }

        let start = self.peek()?.span;
        let function = self.parse_function()?;
        if let Ok(next) = self.peek() {
            let next_token = next.token.clone();
            return self.err(ParseErrorKind::InvalidEOF(next_token));
        }
        let loc = start.merge(&function.span);
        return loc.build(Program::Program(function));
    }

    fn parse_function(&mut self) -> Result<ASTNode<Function>, CompilerError> {
        match self.peek() {
            Ok(tok) => trace!("Parsing Function AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Function AST Node at token: {}", err),
        }

        let start = self.peek()?.span;
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

        let mut blocks = Vec::new();
        while let Some(tok) = self.tokens.peek() {
            if tok.token == Token::CloseBrace {
                break;
            }
            if let Some(next_tok) = self.tokens.peek()
                && next_tok.token == Token::TypeInt
            {
                let decl = self.parse_decl()?;
                let span = decl.span;
                blocks.push(span.build(Block::Declaration(decl))?);
            } else {
                let statement = self.parse_statement()?;
                let span = statement.span;
                blocks.push(span.build(Block::Statement(statement))?);
            }
        }

        let end = self.expect(Token::CloseBrace)?;

        let span = start.merge(&end.span);
        span.build(Function::Function(ident, blocks))
    }

    fn parse_statement(&mut self) -> Result<ASTNode<Statement>, CompilerError> {
        match self.peek() {
            Ok(tok) => trace!("Parsing Statement AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Statement AST Node at token: {}", err),
        }

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
                    return self.err(ParseErrorKind::InvalidLabel);
                }
            }
            _ => {
                let expr = self.parse_expr(0)?;
                let end = self.expect(Token::Semicolon)?;
                let span = start.merge(&end.span);
                span.build(Statement::Expression(expr))
            }
        }
    }

    fn parse_decl(&mut self) -> Result<ASTNode<Declaration>, CompilerError> {
        match self.peek() {
            Ok(tok) => trace!("Parsing Declaration AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Declaration AST Node at token: {}", err),
        }

        let start = self.peek()?.span;
        self.expect(Token::TypeInt)?;
        let next_token = self.get_token()?;
        match next_token.token {
            Token::Identifier(ident) => {
                if let Ok(next_tok) = self.peek()
                    && next_tok.token == Token::Equal
                {
                    self.get_token()?;
                    let expr = self.parse_expr(0)?;
                    let end = self.expect(Token::Semicolon)?;
                    let span = start.merge(&end.span);
                    span.build(Declaration::Declaration(ident, Some(expr)))
                } else {
                    let end = self.expect(Token::Semicolon)?;
                    let span = start.merge(&end.span);
                    span.build(Declaration::Declaration(ident, None))
                }
            }
            _ => self.err(ParseErrorKind::Expected {
                got: next_token.token,
                expected: vec![Token::Identifier("Identifier".to_string())],
            }),
        }
    }

    fn parse_factor(&mut self) -> Result<ASTNode<Expression>, CompilerError> {
        match self.peek() {
            Ok(tok) => trace!("Parsing Factor AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Factor AST Node at token: {}", err),
        }
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
            Token::Identifier(ident) => start.build(Expression::Variable(ident)),

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
        {
            if next.token == Token::Increment || next.token == Token::Decrement {
                let start = self.peek()?.span;
                let next = self.get_token()?;
                let span = start.merge(&next.span);
                return span.build(Expression::UnaryExpr(next.token, Box::new(factor.clone())));
            }
        }
        factor
    }

    fn parse_expr(&mut self, min_precedence: u32) -> Result<ASTNode<Expression>, CompilerError> {
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

    fn parse_unop(&mut self) -> Result<Token, CompilerError> {
        match self.peek() {
            Ok(tok) => trace!("Parsing Unop AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Unop AST Node at token: {}", err),
        }
        let operator = self.get_token()?.token;
        Ok(operator)
    }
}

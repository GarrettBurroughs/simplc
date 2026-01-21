use log::trace;

use crate::error::Location;
use crate::error::ParseErrorKind;
use crate::frontend::ast::ASTNode;
use crate::frontend::ast::Block;
use crate::frontend::ast::Declaration;
use crate::frontend::ast::Expression;
use crate::frontend::ast::Function;
use crate::frontend::ast::Program;
use crate::frontend::ast::Statement;
use std::{iter::Peekable, vec::IntoIter};

use crate::{
    error::CompilerError,
    frontend::tokens::{Token, TokenLocation},
};

#[derive(Debug)]
pub struct Parser {
    tokens: Peekable<IntoIter<TokenLocation>>,
    node_loc: Location,
}

impl Location {
    fn build<T>(self, node: T) -> Result<ASTNode<T>, CompilerError> {
        Ok(ASTNode::new(node, self))
    }
}



impl Parser {
    pub fn new(tokens: Vec<TokenLocation>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            node_loc: Location { row: 0, column: 0 },
        }
    }

    fn err<T>(&self, kind: ParseErrorKind) -> Result<T, CompilerError> {
        Err(CompilerError::ParseError {
            location: self.node_loc,
            kind,
        })
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
        self.set_node_loc();
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
        let err = self.err::<&TokenLocation>(ParseErrorKind::EOF);
        self.tokens.peek().ok_or(err.unwrap_err())
    }

    fn get_token(&mut self) -> Result<TokenLocation, CompilerError> {
        let tok = self.tokens.next().ok_or(self.err::<&TokenLocation>(ParseErrorKind::EOF).unwrap_err())?;
        Ok(tok)
    }

    fn set_node_loc(&mut self) -> Location {
        if let Ok(n) = self.peek() {
            self.node_loc = Location {
                row: n.row,
                column: n.column,
            };
            self.node_loc.clone()
        } else {
            Location { row: 0, column: 0 }
        }
    }

    pub fn parse_program(&mut self) -> Result<ASTNode<Program>, CompilerError> {
        match self.peek() {
            Ok(tok) => trace!("Parsing Program AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Program AST Node at token: {}", err),
        }
        let loc = self.set_node_loc();
        let function = self.parse_function()?;
        if let Ok(next) = self.peek() {
            let next_token = next.token.clone();
            self.set_node_loc();
            return self.err(ParseErrorKind::InvalidEOF(next_token));
        }
        return loc.build(Program::Program(function));
    }

    fn parse_function(&mut self) -> Result<ASTNode<Function>, CompilerError> {
        match self.peek() {
            Ok(tok) => trace!("Parsing Function AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Function AST Node at token: {}", err),
        }
        
        self.expect(Token::TypeInt)?;
        let loc = self.set_node_loc();

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
                let loc = self.set_node_loc();
                let decl = self.parse_decl()?;
                blocks.push(loc.build(Block::Declaration(decl))?);
            } else {
                let loc = self.set_node_loc();
                let statement = self.parse_statement()?;
                blocks.push(loc.build(Block::Statement(statement))?);
            }
        }

        self.expect(Token::CloseBrace)?;

        loc.build(Function::Function(ident, blocks))
    }

    fn parse_statement(&mut self) -> Result<ASTNode<Statement>, CompilerError> {
        match self.peek() {
            Ok(tok) => trace!("Parsing Statement AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Statement AST Node at token: {}", err),
        }
        let loc = self.set_node_loc();
        match &self.peek()?.token {
            Token::Return => {
                self.get_token()?;
                let expr = self.parse_expr(0)?;
                self.expect(Token::Semicolon)?;
                loc.build(Statement::Return(expr))
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
                    return loc.build(Statement::If(
                        expr,
                        Box::new(stmt),
                        Some(Box::new(else_stmt)),
                    ));
                }
                loc.build(Statement::If(expr, Box::new(stmt), None))
            }
            Token::Semicolon => {
                self.get_token()?;
                loc.build(Statement::Null)
            }
            Token::Identifier(_) => {
                let expr = self.parse_expr(0)?;
                if self.peek()?.token == Token::Colon {
                    // A single "variable" is the same as an identifier
                    if let Expression::Variable(name) = &expr.node {
                        // Consume the ":"
                        self.get_token()?;
                        let stmt = self.parse_statement()?;
                        return loc.build(Statement::Label(name.to_string(), Box::new(stmt)));
                    } else {
                        // If the expression is more than a single variable, this is invalid
                        return self.err(ParseErrorKind::InvalidLabel);
                    }
                }
                self.expect(Token::Semicolon)?;
                loc.build(Statement::Expression(expr))
            }
            Token::Goto => {
                self.get_token()?;
                let ident = self.get_token()?;
                if let Token::Identifier(label) = ident.token {
                    self.expect(Token::Semicolon)?;
                    loc.build(Statement::Goto(label))
                } else {
                    return self.err(ParseErrorKind::InvalidLabel);
                }
            }
            _ => {
                let expr = self.parse_expr(0)?;
                self.expect(Token::Semicolon)?;
                loc.build(Statement::Expression(expr))
            }
        }
    }

    fn parse_decl(&mut self) -> Result<ASTNode<Declaration>, CompilerError> {
        match self.peek() {
            Ok(tok) => trace!("Parsing Declaration AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Declaration AST Node at token: {}", err),
        }
        self.expect(Token::TypeInt)?;
        let next_token = self.get_token()?;
        match next_token.token {
            Token::Identifier(ident) => {
                let loc = self.set_node_loc();
                if let Ok(next_tok) = self.peek()
                    && next_tok.token == Token::Equal
                {
                    self.get_token()?;
                    let expr = self.parse_expr(0)?;
                    self.expect(Token::Semicolon)?;
                    loc.build(Declaration::Declaration(ident, Some(expr)))
                } else {
                    self.expect(Token::Semicolon)?;
                    loc.build(Declaration::Declaration(ident, None))
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
        let loc = self.set_node_loc();
        let next_token = self.get_token()?;
        let factor = match next_token.token {
            Token::IntLiteral(val) => loc.build(Expression::IntLiteral(val)),
            Token::Minus | Token::BitwiseCompliment | Token::Not => {
                let expr = self.parse_factor()?;
                loc.build(Expression::UnaryExpr(
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
                let transform = loc.build(Expression::BinaryExpr(
                    op,
                    Box::new(expr.clone()),
                    Box::new(loc.build(Expression::IntLiteral(1))?)
                ))?;
                loc.build(Expression::Assignment(Box::new(expr), Box::new(transform)))
            }
            Token::OpenParen => {
                let inner_expr = self.parse_expr(0);
                self.expect(Token::CloseParen)?;
                inner_expr
            }
            Token::Identifier(ident) => loc.build(Expression::Variable(ident)),

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
                let loc = self.set_node_loc();
                let next = self.get_token()?;
                return loc.build(Expression::UnaryExpr(next.token, Box::new(factor.clone())));
            }
        }
        factor
    }

    fn parse_expr(&mut self, min_precedence: u32) -> Result<ASTNode<Expression>, CompilerError> {
        match self.peek() {
            Ok(tok) => trace!("Parsing Expression AST Node at token: {}", tok),
            Err(err) => trace!("Parsing Expression AST Node at token: {}", err),
        }
        let mut left = self.parse_factor()?;
        while let Ok(tok) = self.peek() {
            if !tok.token.is_binop() {
                break;
            }
            let next_precedence = Parser::precedence(&tok.token);
            if next_precedence < min_precedence {
                break;
            }
            let loc = self.set_node_loc();
            let operator = self.parse_unop()?;
            left = if operator == Token::Equal {
                let right = self.parse_expr(next_precedence)?;
                loc.build(Expression::Assignment(Box::new(left), Box::new(right)))?
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
                let addition = loc.build(Expression::BinaryExpr(
                    compound_operator,
                    Box::new(left.clone()),
                    Box::new(right.clone()),
                ))?;

                loc.build(Expression::Assignment(Box::new(left), Box::new(addition)))?
            } else if operator == Token::QuestionMark {
                let middle = self.parse_expr(0)?;
                self.expect(Token::Colon)?;
                let right = self.parse_expr(next_precedence)?;
                loc.build(Expression::Ternary(
                    Box::new(left),
                    Box::new(middle),
                    Box::new(right),
                ))?
            } else {
                let right = self.parse_expr(next_precedence + 1)?;
                loc.build(Expression::BinaryExpr(
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

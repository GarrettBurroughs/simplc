use crate::frontend::ast::ASTNode;
use crate::frontend::ast::Block;
use crate::frontend::ast::Declaration;
use crate::frontend::ast::Expression;
use crate::frontend::ast::Function;
use crate::frontend::ast::Program;
use crate::frontend::ast::Statement;
use std::{iter::Peekable, vec::IntoIter};

use crate::{
    CompilerError,
    frontend::tokens::{Token, TokenLocation},
};

#[derive(Debug)]
pub struct Parser {
    tokens: Peekable<IntoIter<TokenLocation>>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenLocation>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn precedence(tok: &Token) -> u32 {
        if tok.is_compound_assignment() {
            return 2;
        }
        match tok {
            Token::Equal => 2,
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
            Err(CompilerError::ParseError(
                t.row,
                t.column,
                format!(
                    "Unexpected token {} expected {}",
                    t.token,
                    expected.debug_string()
                ),
            ))
        }
    }

    fn peek(&mut self) -> Result<&TokenLocation, CompilerError> {
        self.tokens
            .peek()
            .ok_or(CompilerError::ParseError(0, 0, "Unexpected EOF".into()))
    }

    fn get_token(&mut self) -> Result<TokenLocation, CompilerError> {
        let tok =
            self.tokens
                .next()
                .ok_or(CompilerError::ParseError(0, 0, "Unexpected EOF".into()))?;
        Ok(tok)
    }

    fn get_loc(&mut self) -> (usize, usize) {
        if let Ok(n) = self.peek() {
            return (n.row, n.column);
        }
        return (0, 0);
    }

    pub fn parse_program(&mut self) -> Result<ASTNode<Program>, CompilerError> {
        let loc = self.get_loc();
        let function = self.parse_function()?;
        if let Ok(next) = self.peek() {
            return Err(CompilerError::ParseError(
                next.row,
                next.column,
                "Invalid token at end of file".into(),
            ));
        }
        return Ok(ASTNode::new(Program::Program(function), loc));
    }

    fn parse_function(&mut self) -> Result<ASTNode<Function>, CompilerError> {
        self.expect(Token::TypeInt)?;
        let loc = self.get_loc();
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
                let loc = self.get_loc();
                let decl = self.parse_decl()?;
                blocks.push(ASTNode::new(Block::Declaration(decl), loc));
            } else {
                let loc = self.get_loc();
                let statement = self.parse_statement()?;
                blocks.push(ASTNode::new(Block::Statement(statement), loc));
            }
        }

        self.expect(Token::CloseBrace)?;

        return Ok(ASTNode::new(Function::Function(ident, blocks), loc));
    }

    fn parse_statement(&mut self) -> Result<ASTNode<Statement>, CompilerError> {
        let loc = self.get_loc();
        match self.peek()?.token {
            Token::Return => {
                self.get_token()?;
                let expr = self.parse_expr(0)?;
                self.expect(Token::Semicolon)?;
                Ok(ASTNode::new(Statement::Return(expr), loc))
            }
            Token::Semicolon => {
                self.get_token()?;
                Ok(ASTNode::new(Statement::Null, loc))
            }
            _ => {
                let expr = self.parse_expr(0)?;
                self.expect(Token::Semicolon)?;
                Ok(ASTNode::new(Statement::Expression(expr), loc))
            }
        }
    }

    fn parse_decl(&mut self) -> Result<ASTNode<Declaration>, CompilerError> {
        self.expect(Token::TypeInt)?;
        let next_token = self.get_token()?;
        match next_token.token {
            Token::Identifier(ident) => {
                let loc = self.get_loc();
                if let Ok(next_tok) = self.peek()
                    && next_tok.token == Token::Equal
                {
                    self.get_token()?;
                    let expr = self.parse_expr(0)?;
                    self.expect(Token::Semicolon)?;
                    Ok(ASTNode::new(
                        Declaration::Declaration(ident, Some(expr)),
                        loc,
                    ))
                } else {
                    self.expect(Token::Semicolon)?;
                    Ok(ASTNode::new(Declaration::Declaration(ident, None), loc))
                }
            }
            _ => Err(CompilerError::ParseError(
                next_token.row,
                next_token.column,
                format!("Unexpected token {} expected Identifier", next_token.token),
            )),
        }
    }

    fn parse_factor(&mut self) -> Result<ASTNode<Expression>, CompilerError> {
        let loc = self.get_loc();
        let next_token = self.get_token()?;
        let factor = match next_token.token {
            Token::IntLiteral(val) => Ok(ASTNode::new(Expression::IntLiteral(val), loc)),
            Token::Minus | Token::BitwiseCompliment | Token::Not => {
                let expr = self.parse_factor()?;
                Ok(ASTNode::new(
                    Expression::UnaryExpr(next_token.token.clone(), Box::new(expr)),
                    loc,
                ))
            }
            Token::Increment | Token::Decrement => {
                let expr = self.parse_factor()?;
                let op = match next_token.token {
                    Token::Increment => Token::Plus,
                    Token::Decrement => Token::Minus,
                    _ => unreachable!(),
                };
                let transform = ASTNode::new(
                    Expression::BinaryExpr(
                        op,
                        Box::new(expr.clone()),
                        Box::new(ASTNode::new(Expression::IntLiteral(1), loc)),
                    ),
                    loc,
                );
                let assign = ASTNode::new(
                    Expression::Assignment(Box::new(expr), Box::new(transform)),
                    loc,
                );
                Ok(assign)
            }
            Token::OpenParen => {
                let inner_expr = self.parse_expr(0);
                self.expect(Token::CloseParen)?;
                inner_expr
            }
            Token::Identifier(ident) => Ok(ASTNode::new(Expression::Variable(ident), loc)),
            _ => Err(CompilerError::ParseError(
                next_token.row,
                next_token.column,
                format!(
                    "Unexpected token {}, Expected: IntLiteral, -, !, (, or identifier",
                    next_token.token
                ),
            )),
        };

        if let Ok(next) = self.peek()
            && let Ok(factor) = &factor
        {
            if next.token == Token::Increment || next.token == Token::Decrement {
                let loc = self.get_loc();
                let next = self.get_token()?;
                return Ok(ASTNode::new(
                    Expression::UnaryExpr(next.token, Box::new(factor.clone())),
                    loc,
                ));
            }
        }
        factor
    }

    fn parse_expr(&mut self, min_precedence: u32) -> Result<ASTNode<Expression>, CompilerError> {
        let mut left = self.parse_factor()?;
        while let Ok(tok) = self.peek() {
            if !tok.token.is_binop() {
                break;
            }
            let next_precedence = Parser::precedence(&tok.token);
            if next_precedence < min_precedence {
                break;
            }
            let loc = self.get_loc();
            let operator = self.parse_unop()?;
            left = if operator == Token::Equal {
                let right = self.parse_expr(next_precedence)?;
                ASTNode::new(Expression::Assignment(Box::new(left), Box::new(right)), loc)
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
                let addition = ASTNode::new(
                    Expression::BinaryExpr(
                        compound_operator,
                        Box::new(left.clone()),
                        Box::new(right.clone()),
                    ),
                    loc,
                );

                ASTNode::new(
                    Expression::Assignment(Box::new(left), Box::new(addition)),
                    loc,
                )
            } else {
                let right = self.parse_expr(next_precedence + 1)?;
                ASTNode::new(
                    Expression::BinaryExpr(operator, Box::new(left), Box::new(right)),
                    loc,
                )
            };
        }
        Ok(left)
    }

    fn parse_unop(&mut self) -> Result<Token, CompilerError> {
        let operator = self.get_token()?.token;
        Ok(operator)
    }
}

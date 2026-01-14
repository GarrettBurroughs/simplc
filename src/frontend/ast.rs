use crate::frontend::tokens::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Return(ASTNode<Expression>),
    Expression(ASTNode<Expression>),
    If(ASTNode<Expression>, Box<ASTNode<Statement>>, Option<Box<ASTNode<Statement>>>),
    Null,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    IntLiteral(i32),
    Variable(String),
    UnaryExpr(Token, Box<ASTNode<Expression>>),
    BinaryExpr(Token, Box<ASTNode<Expression>>, Box<ASTNode<Expression>>),
    Assignment(Box<ASTNode<Expression>>, Box<ASTNode<Expression>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Function {
    Function(String, Vec<ASTNode<Block>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Program {
    Program(ASTNode<Function>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Block {
    Declaration(ASTNode<Declaration>),
    Statement(ASTNode<Statement>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Declaration {
    Declaration(String, Option<ASTNode<Expression>>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ASTNode<T> {
    pub node: T,
    pub row: usize,
    pub column: usize,
}

impl<T> ASTNode<T> {
    pub fn new(node: T, loc: (usize, usize)) -> Self {
        ASTNode {
            node,
            row: loc.0,
            column: loc.1,
        }
    }
}

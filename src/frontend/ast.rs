use crate::{frontend::tokens::Token, sourcemap::Span};

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Return(ASTNode<Expression>),
    Expression(ASTNode<Expression>),
    If(
        ASTNode<Expression>,
        Box<ASTNode<Statement>>,
        Option<Box<ASTNode<Statement>>>,
    ),
    Compound(ASTNode<Block>),
    Label(String, Box<ASTNode<Statement>>),
    Goto(String),
    Null,
    While(ASTNode<Expression>, Box<ASTNode<Statement>>, Option<String>),
    DoWhile(Box<ASTNode<Statement>>, ASTNode<Expression>, Option<String>),
    For(
        ASTNode<Initializer>,
        Option<ASTNode<Expression>>,
        Option<ASTNode<Expression>>,
        Box<ASTNode<Statement>>,
        Option<String>,
    ),
    Switch(ASTNode<Expression>, Box<ASTNode<Statement>>, Option<String>),
    Case(ASTNode<Expression>, Box<ASTNode<Statement>>, Option<String>),
    Default(Box<ASTNode<Statement>>, Option<String>),
    Break(Option<String>),
    Continue(Option<String>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    IntLiteral(i32),
    Variable(String),
    UnaryExpr(Token, Box<ASTNode<Expression>>),
    BinaryExpr(Token, Box<ASTNode<Expression>>, Box<ASTNode<Expression>>),
    Assignment(Box<ASTNode<Expression>>, Box<ASTNode<Expression>>),
    Ternary(
        Box<ASTNode<Expression>>,
        Box<ASTNode<Expression>>,
        Box<ASTNode<Expression>>,
    ),
    FunctionCall(String, Vec<ASTNode<Expression>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FunctionDeclaration {
    FunctionDeclaration(String, Vec<String>, Option<ASTNode<Block>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Block {
    Block(Vec<ASTNode<BlockItem>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Program {
    Program(Vec<ASTNode<FunctionDeclaration>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlockItem {
    Declaration(ASTNode<Declaration>),
    Statement(ASTNode<Statement>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Initializer {
    Decl(ASTNode<VariableDeclaration>),
    Exp(Option<ASTNode<Expression>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Declaration {
    FunctionDeclaration(ASTNode<FunctionDeclaration>),
    VariableDeclaration(ASTNode<VariableDeclaration>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum VariableDeclaration {
    VariableDeclaration(String, Option<ASTNode<Expression>>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ASTNode<T> {
    pub node: T,
    pub span: Span,
}

impl<T> ASTNode<T> {
    pub fn new(node: T, span: Span) -> Self {
        ASTNode { node, span }
    }
}

impl Expression {
    pub fn is_assignable(&self) -> bool {
        matches!(self, Expression::Variable(_))
    }

    pub fn evaluate_const(&self) -> i32 {
        match &self {
            Expression::IntLiteral(val) => *val,
            Expression::BinaryExpr(op, lhs, rhs) => {
                let lhs = lhs.node.evaluate_const();
                let rhs = rhs.node.evaluate_const();
                match op {
                    Token::Plus => lhs + rhs,
                    Token::Minus => lhs - rhs,
                    Token::Div => lhs / rhs,
                    Token::Mul => lhs * rhs,
                    Token::Percent => lhs % rhs,
                    Token::LogicalAnd => (lhs != 0 && rhs != 0) as i32,
                    Token::LogicalOr => (lhs != 0 || rhs != 0) as i32,
                    Token::BitwiseAnd => lhs & rhs,
                    Token::BitwiseOr => lhs | rhs,
                    Token::BitwiseXOR => lhs ^ rhs,
                    Token::LeftShift => lhs << rhs,
                    Token::RightShift => lhs >> rhs,
                    Token::LogicalEq => (lhs == rhs) as i32,
                    Token::NotEqual => (lhs != rhs) as i32,
                    Token::GreaterThan => (lhs > rhs) as i32,
                    Token::GreaterThanEq => (lhs >= rhs) as i32,
                    Token::LessThan => (lhs < rhs) as i32,
                    Token::LessThanEq => (lhs <= rhs) as i32,
                    _ => panic!("Invalid const expr"),
                }
            }
            Expression::UnaryExpr(op, val) => {
                let val = val.node.evaluate_const();

                // <unop>                    ::= "-" | "~" | "!" | <increment>
                match op {
                    Token::Minus => -val,
                    Token::BitwiseCompliment => !val,
                    Token::Not => {
                        if val == 0 {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Invalid const expr"),
                }
            }
            _ => panic!("Invalid const expr"),
        }
    }
}

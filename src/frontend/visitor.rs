use crate::{error::{CompilerError, SemanticErrorKind}, frontend::ast::*};

pub trait Visitor {
    fn visit_program(&mut self, program: &mut ASTNode<Program>)
    where
        Self: Sized,
    {
        walk_program(self, program);
    }

    fn visit_function(&mut self, function: &mut ASTNode<Function>)
    where
        Self: Sized,
    {
        walk_function(self, function);
    }

    fn visit_block_item(&mut self, block_item: &mut ASTNode<BlockItem>)
    where
        Self: Sized,
    {
        walk_block_item(self, block_item);
    }

    fn visit_declaration(&mut self, declaration: &mut ASTNode<Declaration>)
    where
        Self: Sized,
    {
        walk_declaration(self, declaration);
    }

    fn visit_statement(&mut self, statement: &mut ASTNode<Statement>)
    where
        Self: Sized,
    {
        walk_statement(self, statement);
    }

    fn visit_expression(&mut self, expression: &mut ASTNode<Expression>)
    where
        Self: Sized,
    {
        walk_expression(self, expression);
    }

}

pub fn walk_program<T: Visitor>(visitor: &mut T, program: &mut ASTNode<Program>) {
    match &mut program.node {
        Program::Program(func) => func.accept(visitor),
    }
}

pub fn walk_function<T: Visitor>(visitor: &mut T, function: &mut ASTNode<Function>) {
    match &mut function.node {
        Function::Function(_, block_items) => {
            for block_item in block_items {
                block_item.accept(visitor);
            }
        }
    }
}

pub fn walk_block_item<T: Visitor>(visitor: &mut T, block_item: &mut ASTNode<BlockItem>) {
    match &mut block_item.node {
        BlockItem::Declaration(decl) => decl.accept(visitor),
        BlockItem::Statement(stmt) => stmt.accept(visitor),
    }
}

pub fn walk_declaration<T: Visitor>(
    visitor: &mut T,
    declaration: &mut ASTNode<Declaration>,
) {
    match &mut declaration.node {
        Declaration::Declaration(_, Some(expr)) => expr.accept(visitor),
        Declaration::Declaration(_, None) => {}
    }
}

pub fn walk_statement<T: Visitor>(visitor: &mut T, statement: &mut ASTNode<Statement>) {
    match &mut statement.node {
        Statement::Return(expr) => expr.accept(visitor),
        Statement::Expression(expr) => expr.accept(visitor),
        Statement::If(cond, then_stmt, else_stmt) => {
            cond.accept(visitor);
            then_stmt.accept(visitor);
            if let Some(else_stmt) = else_stmt {
                else_stmt.accept(visitor);
            }
        }
        Statement::Null => {}
        Statement::Label(_, stmt) => stmt.accept(visitor),
        Statement::Goto(_) => {}
    }
}

pub fn walk_expression<T: Visitor>(visitor: &mut T, expression: &mut ASTNode<Expression>) {
    match &mut expression.node {
        Expression::UnaryExpr(_, expr) => expr.accept(visitor),
        Expression::BinaryExpr(_, left, right) => {
            left.accept(visitor);
            right.accept(visitor);
        }
        Expression::Assignment(left, right) => {
            left.accept(visitor);
            right.accept(visitor);
        }
        Expression::Ternary(cond, then_expr, else_expr) => {
            cond.accept(visitor);
            then_expr.accept(visitor);
            else_expr.accept(visitor);
        }
        Expression::IntLiteral(_) => {}
        Expression::Variable(_) => {}
    }
}

pub trait AstVisitable {
    fn accept<V: Visitor>(&mut self, visitor: &mut V);
}

impl<T> AstVisitable for ASTNode<T>
where
    T: VisitableNode,
{
    fn accept<V: Visitor>(&mut self, visitor: &mut V) {
        T::accept(self, visitor);
    }
}

pub trait VisitableNode {
    fn accept<V: Visitor>(node: &mut ASTNode<Self>, visitor: &mut V)
    where
        Self: Sized;
}

impl VisitableNode for Program {
    fn accept<V: Visitor>(node: &mut ASTNode<Self>, visitor: &mut V) {
        visitor.visit_program(node);
    }
}

impl VisitableNode for Function {
    fn accept<V: Visitor>(node: &mut ASTNode<Self>, visitor: &mut V) {
        visitor.visit_function(node);
    }
}

impl VisitableNode for BlockItem {
    fn accept<V: Visitor>(node: &mut ASTNode<Self>, visitor: &mut V) {
        visitor.visit_block_item(node);
    }
}

impl VisitableNode for Declaration {
    fn accept<V: Visitor>(node: &mut ASTNode<Self>, visitor: &mut V) {
        visitor.visit_declaration(node);
    }
}

impl VisitableNode for Statement {
    fn accept<V: Visitor>(node: &mut ASTNode<Self>, visitor: &mut V) {
        visitor.visit_statement(node);
    }
}

impl VisitableNode for Expression {
    fn accept<V: Visitor>(node: &mut ASTNode<Self>, visitor: &mut V) {
        visitor.visit_expression(node);
    }
}

pub fn semantic_error<T>(node: &ASTNode<T>, kind: SemanticErrorKind) -> Option<CompilerError> {
    Some(CompilerError::SemanticError {
        location: node.span.into(),
        kind
    })
}

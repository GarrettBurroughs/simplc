use crate::{
    error::{CompilerError, SemanticErrorKind},
    frontend::ast::*,
    sourcemap::Span,
};

pub trait Visitor {
    fn visit_program(&mut self, program: &mut ASTNode<Program>)
    where
        Self: Sized,
    {
        walk_program(self, program);
    }

    fn visit_block(&mut self, block: &mut ASTNode<Block>)
    where
        Self: Sized,
    {
        walk_block(self, block);
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

    fn visit_function_declaration(&mut self, function: &mut ASTNode<FunctionDeclaration>)
    where
        Self: Sized,
    {
        walk_function_declaration(self, function);
    }

    fn visit_variable_declaration(
        &mut self,
        variable_declaration: &mut ASTNode<VariableDeclaration>,
    ) where
        Self: Sized,
    {
        walk_variable_declaration(self, variable_declaration);
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

    fn visit_initializer(&mut self, initializer: &mut ASTNode<Initializer>)
    where
        Self: Sized,
    {
        walk_initializer(self, initializer);
    }
}

pub fn walk_program<T: Visitor>(visitor: &mut T, program: &mut ASTNode<Program>) {
    match &mut program.node {
        Program::Program(func_list) => {
            for func in func_list {
                func.accept(visitor)
            }
        }
    }
}

pub fn walk_function_declaration<T: Visitor>(
    visitor: &mut T,
    function: &mut ASTNode<FunctionDeclaration>,
) {
    match &mut function.node {
        FunctionDeclaration::FunctionDeclaration(_, _, Some(block)) => block.accept(visitor),
        _ => {}
    }
}

pub fn walk_block<T: Visitor>(visitor: &mut T, block: &mut ASTNode<Block>) {
    match &mut block.node {
        Block::Block(block_items) => {
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

pub fn walk_declaration<T: Visitor>(visitor: &mut T, declaration: &mut ASTNode<Declaration>) {
    match &mut declaration.node {
        Declaration::FunctionDeclaration(fn_decl) => fn_decl.accept(visitor),
        Declaration::VariableDeclaration(v_decl) => v_decl.accept(visitor),
    }
}

pub fn walk_variable_declaration<T: Visitor>(
    visitor: &mut T,
    variable_declaration: &mut ASTNode<VariableDeclaration>,
) {
    match &mut variable_declaration.node {
        VariableDeclaration::VariableDeclaration(_, Some(expr)) => expr.accept(visitor),
        VariableDeclaration::VariableDeclaration(_, _) => {}
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
        Statement::Compound(block) => block.accept(visitor),
        Statement::While(condition, stmt, _) => {
            condition.accept(visitor);
            stmt.accept(visitor);
        }
        Statement::DoWhile(stmt, condition, _) => {
            stmt.accept(visitor);
            condition.accept(visitor);
        }
        Statement::For(initializer, condition, post, stmt, _) => {
            initializer.accept(visitor);
            if let Some(condition) = condition {
                condition.accept(visitor);
            }
            if let Some(post) = post {
                post.accept(visitor);
            }
            stmt.accept(visitor);
        }
        Statement::Break(_) => {}
        Statement::Continue(_) => {}
        Statement::Switch(expr, stmt, _) => {
            expr.accept(visitor);
            stmt.accept(visitor);
        }
        Statement::Case(expr, stmt, _) => {
            expr.accept(visitor);
            stmt.accept(visitor);
        }
        Statement::Default(expr, _) => expr.accept(visitor),
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
        Expression::FunctionCall(_, args) => {
            for a in args {
                a.accept(visitor);
            }
        }
    }
}

pub fn walk_initializer<T: Visitor>(visitor: &mut T, initializer: &mut ASTNode<Initializer>) {
    match &mut initializer.node {
        Initializer::Decl(decl) => decl.accept(visitor),
        Initializer::Exp(expr) => {
            if let Some(expr) = expr {
                expr.accept(visitor);
            }
        }
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

impl VisitableNode for FunctionDeclaration {
    fn accept<V: Visitor>(node: &mut ASTNode<Self>, visitor: &mut V) {
        visitor.visit_function_declaration(node);
    }
}

impl VisitableNode for Block {
    fn accept<V: Visitor>(node: &mut ASTNode<Self>, visitor: &mut V) {
        visitor.visit_block(node);
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

impl VisitableNode for VariableDeclaration {
    fn accept<V: Visitor>(node: &mut ASTNode<Self>, visitor: &mut V) {
        visitor.visit_variable_declaration(node);
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

impl VisitableNode for Initializer {
    fn accept<V: Visitor>(node: &mut ASTNode<Self>, visitor: &mut V) {
        visitor.visit_initializer(node);
    }
}

pub fn semantic_error(loc: Span, kind: SemanticErrorKind) -> Option<CompilerError> {
    Some(CompilerError::SemanticError {
        location: loc.into(),
        kind,
    })
}

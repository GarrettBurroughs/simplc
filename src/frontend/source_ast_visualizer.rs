use crate::{frontend::ast::*, sourcemap::SourceFile};

pub struct SourceASTVisualizer<'a> {
    source_file: &'a SourceFile,
}

impl<'a> SourceASTVisualizer<'a> {
    pub fn new(source_file: &'a SourceFile) -> Self {
        Self { source_file }
    }

    pub fn visualize(&self, program: &ASTNode<Program>) -> String {
        println!("AST Visualization:");
        self.visit_program(program)
    }

    fn visit_program(&self, program: &ASTNode<Program>) -> String {
        let source = &self.source_file.contents[program.span.start..program.span.end];
        let mut output = format!("Program: \n'{}''\n", source);
        let Program::Program(function) = &program.node;
        output.push_str(&self.visit_function(function));
        output
    }

    fn visit_function(&self, function: &ASTNode<Function>) -> String {
        let source = &self.source_file.contents[function.span.start..function.span.end];
        let Function::Function(name, blocks) = &function.node;
        let mut output = format!("Function({}): \n'{}'\n", name, source);
        for block in blocks {
            output.push_str(&self.visit_block(block));
        }
        output
    }

    fn visit_block(&self, block: &ASTNode<Block>) -> String {
        let source = &self.source_file.contents[block.span.start..block.span.end];
        let mut output = format!("Block: \n'{}'\n", source);
        let contents = match &block.node {
            Block::Declaration(declaration) => self.visit_declaration(declaration),
            Block::Statement(statement) => self.visit_statement(statement),
        };
        output.push_str(&contents);
        output
    }

    fn visit_declaration(&self, declaration: &ASTNode<Declaration>) -> String {
        let source = &self.source_file.contents[declaration.span.start..declaration.span.end];
        let Declaration::Declaration(name, expr) = &declaration.node;
        let mut output = format!("Declaration({}): \n'{}'\n", name, source);
        if let Some(expr) = expr {
            output.push_str(&self.visit_expression(expr));
        }
        output
    }

    fn visit_statement(&self, statement: &ASTNode<Statement>) -> String {
        let source = &self.source_file.contents[statement.span.start..statement.span.end];
        let mut output = format!("Statement: \n'{}'\n", source);
        match &statement.node {
            Statement::Return(expr) => {
                output.push_str(&self.visit_expression(expr));
            }
            Statement::Expression(expr) => {
                output.push_str(&self.visit_expression(expr));
            }
            Statement::If(cond, then, else_stmt) => {
                output.push_str(&self.visit_expression(cond));
                output.push_str(&self.visit_statement(then));
                if let Some(else_stmt) = else_stmt {
                    output.push_str(&self.visit_statement(else_stmt));
                }
            }
            Statement::Label(_, stmt) => {
                output.push_str(&self.visit_statement(stmt));
            }
            Statement::Goto(_) => {
            }
            Statement::Null => {
            }
        };
        output
    }

    fn visit_expression(&self, expression: &ASTNode<Expression>) -> String {
        let source = &self.source_file.contents[expression.span.start..expression.span.end];
        let mut output = format!("Expression: \n'{}'\n", source);
        match &expression.node {
            Expression::IntLiteral(_) => {
            }
            Expression::Variable(_) => {
            }
            Expression::UnaryExpr(_, expr) => {
                output.push_str(&self.visit_expression(expr));
            }
            Expression::BinaryExpr(_, left, right) => {
                output.push_str(&self.visit_expression(left));
                output.push_str(&self.visit_expression(right));
            }
            Expression::Assignment(left, right) => {
                output.push_str(&self.visit_expression(left));
                output.push_str(&self.visit_expression(right));
            }
            Expression::Ternary(cond, then, else_expr) => {
                output.push_str(&self.visit_expression(cond));
                output.push_str(&self.visit_expression(then));
                output.push_str(&self.visit_expression(else_expr));
            }
        }
        output
    }
}

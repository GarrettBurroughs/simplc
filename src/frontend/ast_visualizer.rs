use crate::{
    frontend::ast::{ASTNode, BlockItem, Declaration, Expression, Function, Program, Statement},
    sourcemap::SourceFile,
};

// Helper to manage indentation
fn indent(level: usize) -> String {
    "  ".repeat(level)
}

pub struct ASTVisualizer<'a> {
    source_file: &'a SourceFile,
}

impl<'a> ASTVisualizer<'a> {
    pub fn new(source_file: &'a SourceFile) -> Self {
        ASTVisualizer { source_file }
    }

    // Helper to format the location cleanly
    fn format_loc<T>(&self, node: &ASTNode<T>) -> String {
        let loc = self.source_file.lookup(node.span.start);
        format!("@ {}", loc)
    }

    pub fn visualize(&self, program: &ASTNode<Program>) -> String {
        self.visit_program(program)
    }

    fn visit_program(&self, program: &ASTNode<Program>) -> String {
        match &program.node {
            Program::Program(func) => format!(
                "Program {}\n{}",
                self.format_loc(program),
                self.visit_function(func, 0)
            ),
        }
    }

    fn visit_function(&self, function: &ASTNode<Function>, level: usize) -> String {
        match &function.node {
            Function::Function(name, block_items) => {
                let mut output = format!(
                    "{}Function \"{}\" {}\n",
                    indent(level),
                    name,
                    self.format_loc(function)
                );
                for block_item in block_items {
                    output.push_str(&self.visit_block_item(block_item, level + 1));
                }
                output
            }
        }
    }

    fn visit_block_item(&self, block: &ASTNode<BlockItem>, level: usize) -> String {
        match &block.node {
            BlockItem::Statement(stmt) => self.visit_statement(stmt, level),
            BlockItem::Declaration(decl) => self.visit_declaration(decl, level),
        }
    }

    fn visit_declaration(&self, declaration: &ASTNode<Declaration>, level: usize) -> String {
        match &declaration.node {
            Declaration::Declaration(name, expr_opt) => {
                let base = format!(
                    "{}Decl \"{}\" {}",
                    indent(level),
                    name,
                    self.format_loc(declaration)
                );
                if let Some(expr) = expr_opt {
                    format!("{} = \n{}", base, self.visit_expression(expr, level + 1))
                } else {
                    format!("{}\n", base)
                }
            }
        }
    }

    fn visit_statement(&self, statement: &ASTNode<Statement>, level: usize) -> String {
        let i = indent(level);
        let loc = self.format_loc(statement);
        match &statement.node {
            Statement::Return(expr) => format!(
                "{}Return {}\n{}",
                i,
                loc,
                self.visit_expression(expr, level + 1)
            ),
            Statement::Expression(expr) => format!(
                "{}ExprStmt {}\n{}",
                i,
                loc,
                self.visit_expression(expr, level + 1)
            ),
            Statement::Goto(target) => format!("{}Goto \"{}\" {}\n", i, target, loc),
            Statement::Label(name, stmt) => format!(
                "{}Label \"{}\": {}\n{}",
                i,
                name,
                loc,
                self.visit_statement(stmt, level + 1)
            ),
            Statement::If(cond, then_branch, else_branch) => {
                let mut out = format!(
                    "{}If {}\n{}",
                    i,
                    loc,
                    self.visit_expression(cond, level + 1)
                );
                out.push_str(&format!(
                    "{}Then\n{}",
                    indent(level),
                    self.visit_statement(then_branch, level + 1)
                ));
                if let Some(else_b) = else_branch {
                    out.push_str(&format!(
                        "{}Else\n{}",
                        indent(level),
                        self.visit_statement(else_b, level + 1)
                    ));
                }
                out
            }
            Statement::Null => format!("{}NullStmt {}\n", i, loc),
        }
    }

    fn visit_expression(&self, expression: &ASTNode<Expression>, level: usize) -> String {
        let i = indent(level);
        let loc = self.format_loc(expression);
        match &expression.node {
            Expression::IntLiteral(val) => format!("{}Int({}) {}\n", i, val, loc),
            Expression::Variable(name) => format!("{}Var(\"{}\") {}\n", i, name, loc),
            Expression::BinaryExpr(op, left, right) => {
                format!(
                    "{}BinaryOp({:?}) {}\n{}{}",
                    i,
                    op,
                    loc,
                    self.visit_expression(left, level + 1),
                    self.visit_expression(right, level + 1),
                )
            }
            Expression::UnaryExpr(op, expr) => {
                format!(
                    "{}UnaryOp({:?}) {}\n{}",
                    i,
                    op,
                    loc,
                    self.visit_expression(expr, level + 1),
                )
            }
            Expression::Assignment(target, value) => {
                format!(
                    "{}Assign {}\n{}{}",
                    i,
                    loc,
                    self.visit_expression(target, level + 1),
                    self.visit_expression(value, level + 1)
                )
            }
            Expression::Ternary(cond, true_expr, false_expr) => {
                format!(
                    "{}Ternary {}\n{}{}{}",
                    i,
                    loc,
                    self.visit_expression(cond, level + 1),
                    self.visit_expression(true_expr, level + 1),
                    self.visit_expression(false_expr, level + 1),
                )
            }
        }
    }
}

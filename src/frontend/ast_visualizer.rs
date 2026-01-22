use crate::frontend::ast::{ASTNode, Program, Function, Block, Statement, Expression, Declaration};

// Helper to manage indentation
fn indent(level: usize) -> String {
    "  ".repeat(level)
}

// Helper to format the location cleanly
fn format_loc<T>(node: &ASTNode<T>) -> String {
    format!("@ {}:{}", node.span.start, node.span.end)
}

impl ASTNode<Program> {
    pub fn visualize(&self) -> String {
        match &self.node {
            Program::Program(func) => format!("Program {}\n{}", format_loc(self), func.visualize(1)),
        }
    }
}

impl ASTNode<Function> {
    pub fn visualize(&self, level: usize) -> String {
        match &self.node {
            Function::Function(name, blocks) => {
                let mut output = format!("{}Function \"{}\" {}\n", indent(level), name, format_loc(self));
                for block in blocks {
                    output.push_str(&block.visualize(level + 1));
                }
                output
            }
        }
    }
}

impl ASTNode<Block> {
    pub fn visualize(&self, level: usize) -> String {
        match &self.node {
            Block::Statement(stmt) => stmt.visualize(level),
            Block::Declaration(decl) => decl.visualize(level),
        }
    }
}

impl ASTNode<Declaration> {
    pub fn visualize(&self, level: usize) -> String {
        match &self.node {
            Declaration::Declaration(name, expr_opt) => {
                let base = format!("{}Decl \"{}\" {}", indent(level), name, format_loc(self));
                if let Some(expr) = expr_opt {
                    format!("{} = \n{}", base, expr.visualize(level + 1))
                } else {
                    format!("{}\n", base)
                }
            }
        }
    }
}

impl ASTNode<Statement> {
    pub fn visualize(&self, level: usize) -> String {
        let i = indent(level);
        let loc = format_loc(self);
        match &self.node {
            Statement::Return(expr) => format!("{}Return {}\n{}", i, loc, expr.visualize(level + 1)),
            Statement::Expression(expr) => format!("{}ExprStmt {}\n{}", i, loc, expr.visualize(level + 1)),
            Statement::Goto(target) => format!("{}Goto \"{}\" {}\n", i, target, loc),
            Statement::Label(name, stmt) => format!("{}Label \"{}\": {}\n{}", i, name, loc, stmt.visualize(level + 1)),
            Statement::If(cond, then_branch, else_branch) => {
                let mut out = format!("{}If {}\n{}", i, loc, cond.visualize(level + 1));
                out.push_str(&format!("{}Then\n{}", indent(level), then_branch.visualize(level + 1)));
                if let Some(else_b) = else_branch {
                    out.push_str(&format!("{}Else\n{}", indent(level), else_b.visualize(level + 1)));
                }
                out
            },
            Statement::Null => format!("{}NullStmt {}\n", i, loc),
        }
    }
}

impl ASTNode<Expression> {
    pub fn visualize(&self, level: usize) -> String {
        let i = indent(level);
        let loc = format_loc(self);
        match &self.node {
            Expression::IntLiteral(val) => format!("{}Int({}) {}\n", i, val, loc),
            Expression::Variable(name) => format!("{}Var(\"{}\") {}\n", i, name, loc),
            Expression::BinaryExpr(op, left, right) => {
                format!(
                    "{}BinaryOp({:?}) {}\n{}{}", 
                    i, op, loc,
                    left.visualize(level + 1), 
                    right.visualize(level + 1)
                )
            },
            Expression::UnaryExpr(op, expr) => {
                format!(
                    "{}UnaryOp({:?}) {}\n{}", 
                    i, op, loc,
                    expr.visualize(level + 1)
                )
            },
            Expression::Assignment(target, value) => {
                format!("{}Assign {}\n{}{}", i, loc, target.visualize(level + 1), value.visualize(level + 1))
            },
            Expression::Ternary(cond, true_expr, false_expr) => {
                format!(
                    "{}Ternary {}\n{}{}{}", 
                    i, loc,
                    cond.visualize(level + 1), 
                    true_expr.visualize(level + 1), 
                    false_expr.visualize(level + 1)
                )
            }
        }
    }
}

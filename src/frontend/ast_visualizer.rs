use crate::{
    frontend::ast::{
        ASTNode, Block, BlockItem, Declaration, Expression, Function, Initializer, Program,
        Statement,
    },
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
                self.visit_function(func, 1)
            ),
        }
    }

    fn visit_function(&self, function: &ASTNode<Function>, level: usize) -> String {
        match &function.node {
            Function::Function(name, block) => {
                let mut output = format!(
                    "{}Function \"{}\" {}\n",
                    indent(level),
                    name,
                    self.format_loc(function)
                );
                output.push_str(&self.visit_block(block, level + 1));
                output
            }
        }
    }

    fn visit_block(&self, block: &ASTNode<Block>, level: usize) -> String {
        match &block.node {
            Block::Block(block_items) => {
                let mut output = format!("{}Block {}\n", indent(level), self.format_loc(block));
                for block_item in block_items {
                    output.push_str(&self.visit_block_item(block_item, level + 1));
                }
                output
            }
        }
    }

    fn visit_block_item(&self, block_item: &ASTNode<BlockItem>, level: usize) -> String {
        match &block_item.node {
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
            Statement::Compound(block) => format!("{}", self.visit_block(block, level)),
            Statement::While(condition, stmt, label) => format!(
                "{}While#{} {}\n{}{}",
                indent(level),
                label.clone().unwrap_or_default(),
                loc,
                self.visit_expression(condition, level + 1),
                self.visit_statement(stmt, level + 1)
            ),
            Statement::DoWhile(stmt, condition, label) => format!(
                "{}DoWhile#{} {}\n{}{}",
                indent(level),
                label.clone().unwrap_or_default(),
                loc,
                self.visit_statement(stmt, level + 1),
                self.visit_expression(condition, level + 1)
            ),
            Statement::For(initializer, condition, post, stmt, label) => {
                let mut output = format!(
                    "{}For#{} {}\n",
                    indent(level),
                    label.clone().unwrap_or_default(),
                    loc
                );
                output.push_str(&format!(
                    "{}",
                    self.visit_initializer(initializer, level + 1)
                ));
                if let Some(condition) = condition {
                    output.push_str(&format!("{}", self.visit_expression(condition, level + 1)));
                }
                if let Some(post) = post {
                    output.push_str(&format!("{}", self.visit_expression(post, level + 1)));
                }
                output.push_str(&format!("{}", self.visit_statement(stmt, level + 1)));
                output
            }
            Statement::Break(label) => {
                format!(
                    "{}Break#{} {}\n",
                    indent(level),
                    label.clone().unwrap_or_default(),
                    loc
                )
            }
            Statement::Continue(label) => {
                format!(
                    "{}Continue#{} {}\n",
                    indent(level),
                    label.clone().unwrap_or_default(),
                    loc
                )
            }
            Statement::Switch(expr, stmt, label) => {
                format!(
                    "{}Switch#{} {}\n{}{}",
                    indent(level),
                    label.clone().unwrap_or_default(),
                    loc,
                    self.visit_expression(expr, level + 1),
                    self.visit_statement(stmt, level + 1),
                )
            }
            Statement::Case(expr, stmt, label) => {
                format!(
                    "{}Case#{} {}\n{}{}",
                    indent(level),
                    label.clone().unwrap_or_default(),
                    loc,
                    self.visit_expression(expr, level + 1),
                    self.visit_statement(stmt, level + 1),
                )
            }
            Statement::Default(stmt, label) => {
                format!(
                    "{}Default#{} {}\n{}",
                    indent(level),
                    label.clone().unwrap_or_default(),
                    loc,
                    self.visit_statement(stmt, level + 1)
                )
            }
        }
    }

    fn visit_initializer(&self, initializer: &ASTNode<Initializer>, level: usize) -> String {
        match &initializer.node {
            Initializer::Decl(decl) => self.visit_declaration(decl, level),
            Initializer::Exp(expr) => {
                if let Some(expr) = expr {
                    self.visit_expression(expr, level)
                } else {
                    String::new()
                }
            }
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

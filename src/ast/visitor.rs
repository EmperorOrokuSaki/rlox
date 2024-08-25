use crate::errors::RLoxError;

use super::{expr::Expr, stmt::Stmt};

pub trait ExprVisitor<R> {
    fn visit_binary_expr(&self, expr: &Expr) -> Result<R, RLoxError>;
    fn visit_literal_expr(&self, expr: &Expr) -> Result<R, RLoxError>;
    fn visit_grouping_expr(&self, expr: &Expr) -> Result<R, RLoxError>;
    fn visit_unary_expr(&self, expr: &Expr) -> Result<R, RLoxError>;
}

pub trait StmtVisitor<R> {
    fn visit_expr_stmt(&self, stmt: &Stmt) -> Result<R, RLoxError>;
    fn visit_print_stmt(&self, stmt: &Stmt) -> Result<R, RLoxError>;
}

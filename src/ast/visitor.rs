use crate::errors::RLoxError;

use super::expr::Expr;

pub trait Visitor<R> {
    fn visit_binary_expr(&self, expr: &Expr) -> Result<R, RLoxError>;
    fn visit_literal_expr(&self, expr: &Expr) -> Result<R, RLoxError>;
    fn visit_grouping_expr(&self, expr: &Expr) -> Result<R, RLoxError>;
    fn visit_unary_expr(&self, expr: &Expr) -> Result<R, RLoxError>;
}

use super::expr::Expr;

pub trait Visitor<R> {
    fn visit_binary_expr(&self, expr: &Expr) -> R;
    fn visit_assign_expr(&self, expr: &Expr) -> R;
    fn visit_literal_expr(&self, expr: &Expr) -> R;
    fn visit_grouping_expr(&self, expr: &Expr) -> R;
    fn visit_unary_expr(&self, expr: &Expr) -> R;
}

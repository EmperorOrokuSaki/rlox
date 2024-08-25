use crate::{
    errors::RLoxError,
    tokens::{Object, Token},
};

use super::{
    expr::Expr,
    visitor::{ExprVisitor, StmtVisitor},
};

#[derive(Debug)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &dyn StmtVisitor<R>) -> Result<R, RLoxError> {
        match self {
            Stmt::Expression { .. } => visitor.visit_expr_stmt(self),
            Stmt::Print { .. } => visitor.visit_print_stmt(self),
        }
    }
}

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
    Var {name: Token, initializer: Expr}
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> Result<R, RLoxError> {
        match self {
            Stmt::Expression { .. } => visitor.visit_expr_stmt(self),
            Stmt::Print { .. } => visitor.visit_print_stmt(self),
            Stmt::Var { .. } => visitor.visit_var_stmt(self),
        }
    }
}

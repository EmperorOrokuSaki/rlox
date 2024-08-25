use crate::{
    errors::RLoxError,
    tokens::{Object, Token},
};

use super::visitor::ExprVisitor;

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: Object,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token
    }
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn ExprVisitor<R>) -> Result<R, RLoxError> {
        match self {
            Expr::Binary { .. } => visitor.visit_binary_expr(self),
            Expr::Literal { .. } => visitor.visit_literal_expr(self),
            Expr::Grouping { .. } => visitor.visit_grouping_expr(self),
            Expr::Unary { .. } => visitor.visit_unary_expr(self),
            Expr::Variable { .. } => visitor.visit_variable_expr(self),
        }
    }
}

use crate::tokens::Token;

use super::visitor::Visitor;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: String,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expr::Binary { .. } => visitor.visit_binary_expr(self),
            Expr::Literal { .. } => visitor.visit_literal_expr(self),
            Expr::Grouping { .. } => visitor.visit_grouping_expr(self),
            Expr::Unary { .. } => visitor.visit_unary_expr(self),
        }
    }
}

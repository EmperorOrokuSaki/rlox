use crate::errors::RLoxError;

use super::{expr::Expr, visitor::Visitor};

pub struct AstPrinter {}

impl AstPrinter {
    fn parenthesize(lexeme: &str, exprs: Vec<&str>) -> String {
        let mut response = format!("({}", lexeme);
        exprs.into_iter().for_each(|expression| {
            response.push_str(&format!(" {}", expression));
        });
        response.push(')');
        response
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Expr) -> Result<String, RLoxError> {
        let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        else {
            panic!("PANIC! `visit_binary_expr` was called with a non Expr::Binary value!")
        };
        let left_string = left.accept(&Self {})?;
        let right_string = right.accept(&Self {})?;
        Ok(Self::parenthesize(
            &operator.lexeme,
            vec![&left_string, &right_string],
        ))
    }

    fn visit_literal_expr(&self, expr: &Expr) -> Result<String, RLoxError> {
        let Expr::Literal { value } = expr else {
            panic!("PANIC! `visit_literal_expr` was called with a non Expr::Literal value!")
        };

        Ok(format!("{:#?}", value))
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Result<String, RLoxError> {
        let Expr::Grouping { expression } = expr else {
            panic!("PANIC! `visit_grouping_expr` was called with a non Expr::Grouping value!")
        };
        let expression_string = expression.accept(&Self {})?;
        Ok(Self::parenthesize("group", vec![&expression_string]))
    }

    fn visit_unary_expr(&self, expr: &Expr) -> Result<String, RLoxError> {
        let Expr::Unary { operator, right } = expr else {
            panic!("PANIC! `visit_unary_expr` was called with a non Expr::Unary value!")
        };
        let right_string = right.accept(&Self {})?;
        Ok(Self::parenthesize(&operator.lexeme, vec![&right_string]))
    }
}

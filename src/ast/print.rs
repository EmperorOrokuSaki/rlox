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
    fn visit_binary_expr(&self, expr: &Expr) -> String {
        let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        else {
            panic!("PANIC! `visit_binary_expr` was called with a non Expr::Binary value!")
        };
        let left_string = left.accept(&Self {});
        let right_string = right.accept(&Self {});
        return Self::parenthesize(&operator.lexeme, vec![&left_string, &right_string]);
    }

    fn visit_assign_expr(&self, expr: &Expr) -> String {
        todo!()
    }

    fn visit_literal_expr(&self, expr: &Expr) -> String {
        todo!()
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> String {
        todo!()
    }

    fn visit_unary_expr(&self, expr: &Expr) -> String {
        todo!()
    }
}

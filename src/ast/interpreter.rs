use crate::tokens::{Object, TokenType};

use super::{expr::Expr, visitor::Visitor};

pub struct Interpreter {}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Object {
        expr.accept(self)
    }

    fn is_truthy(&self, object: Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(boolean) => boolean,
            _ => true,
        }
    }

    fn is_equal(&self, left_side: Object, right_side: Object) -> bool {
        match left_side {
            Object::Nil => {
                if let Object::Nil = right_side {
                    return true;
                }
                false
            }
            Object::Boolean(left_boolean) => {
                if let Object::Boolean(right_boolean) = right_side {
                    return left_boolean == right_boolean;
                }
                panic!("expected boolean on both sides, got another type on the right");
            }
            Object::Number(left_number) => {
                if let Object::Number(right_number) = right_side {
                    return left_number == right_number;
                }
                panic!("expected number on both sides, got another type on the right");
            }
            Object::String(left_string) => {
                if let Object::String(right_string) = right_side {
                    return left_string == right_string;
                }
                panic!("expected string on both sides, got another type on the right");
            }
        }
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_binary_expr(&self, expr: &Expr) -> Object {
        if let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        {
            let left_resolved = self.evaluate(left);
            let right_resolved = self.evaluate(right);

            // The == and != checks work with any pair of objects, as long as both sides are the same type.
            if let TokenType::EqualEqual = operator.token_type {
                return Object::Boolean(self.is_equal(left_resolved, right_resolved));
            } else if let TokenType::BangEqual = operator.token_type {
                return Object::Boolean(!self.is_equal(left_resolved, right_resolved));
            }

            if let Object::Number(left_number) = left_resolved {
                if let Object::Number(right_number) = right_resolved {
                    let return_number = match operator.token_type {
                        TokenType::Minus => Some(left_number - right_number),
                        TokenType::Slash => Some(left_number / right_number),
                        TokenType::Star => Some(left_number * right_number),
                        TokenType::Plus => Some(left_number + right_number),
                        _ => None,
                    };

                    let return_bool = match operator.token_type {
                        TokenType::Greater => Some(left_number > right_number),
                        TokenType::GreaterEqual => Some(left_number >= right_number),
                        TokenType::Less => Some(left_number < right_number),
                        TokenType::LessEqual => Some(left_number <= right_number),
                        _ => None,
                    };

                    if let Some(number) = return_number {
                        return Object::Number(number);
                    } else if let Some(boolean) = return_bool {
                        return Object::Boolean(boolean);
                    }
                    panic!("Unexpected operator between numbers");
                }
            } else if let Object::String(left_string) = left_resolved {
                if let Object::String(right_string) = right_resolved {
                    let return_string = match operator.token_type {
                        TokenType::Plus => format!("{}{}", left_string, right_string),
                        _ => panic!("Expected arithmetic operators"),
                    };

                    return Object::String(return_string);
                }
            }
        }
        panic!("Types on right and left sides of the binary are different");
    }

    fn visit_literal_expr(&self, expr: &Expr) -> Object {
        if let Expr::Literal { value } = expr {
            return value.clone();
        } else {
            panic!("Expected literal, got other value")
        }
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Object {
        if let Expr::Grouping { expression } = expr {
            return self.evaluate(expression);
        } else {
            panic!("Expected grouping, got other value")
        }
    }

    fn visit_unary_expr(&self, expr: &Expr) -> Object {
        if let Expr::Unary { operator, right } = expr {
            let right_side = self.evaluate(right);
            match operator.token_type {
                TokenType::Bang => Object::Boolean(!self.is_truthy(right_side)),
                TokenType::Minus => {
                    if let Object::Number(number) = right_side {
                        Object::Number(-number)
                    } else {
                        panic!("Expected number got something else");
                    }
                }
                _ => panic!("Expected BANG or MINUS but got something else"),
            }
        } else {
            panic!("Expected grouping, got other value")
        }
    }
}

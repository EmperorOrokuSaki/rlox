use crate::{
    errors::RLoxError,
    tokens::{Object, Token, TokenType},
};

use crate::ast::{expr::Expr, visitor::Visitor};

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, expr: Expr) -> Result<(), RLoxError> {
        let value: Object = self.evaluate(&expr)?;
        println!("{:#?}", value);
        Ok(())
    }

    fn check_number_operand(&self, operator: Token, right: Object) -> Result<f64, RLoxError> {
        if let Object::Number(number) = right {
            return Ok(number);
        }
        Err(RLoxError::InterpreterError(
            operator,
            "Operand must be a number.".to_string(),
        ))
    }

    fn check_number_operands(
        &self,
        operator: Token,
        left: Object,
        right: Object,
    ) -> Result<(f64, f64), RLoxError> {
        if let Object::Number(left_number) = left {
            if let Object::Number(right_number) = right {
                return Ok((left_number, right_number));
            }
        }
        Err(RLoxError::InterpreterError(
            operator,
            "Operands must be numbers.".to_string(),
        ))
    }

    fn check_string_operands(
        &self,
        operator: Token,
        left: Object,
        right: Object,
    ) -> Result<(String, String), RLoxError> {
        if let Object::String(left_string) = left {
            if let Object::String(right_string) = right {
                return Ok((left_string, right_string));
            }
        }
        Err(RLoxError::InterpreterError(
            operator,
            "Operands must be strings.".to_string(),
        ))
    }

    fn evaluate(&self, expr: &Expr) -> Result<Object, RLoxError> {
        expr.accept(self)
    }

    fn is_truthy(&self, object: Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(boolean) => boolean,
            _ => true,
        }
    }

    fn is_equal(&self, left_side: Object, right_side: Object) -> Result<bool, String> {
        let error_message =
            format!("Unexpected different types on the left and right sides of the operator.");

        match left_side {
            Object::Nil => {
                if let Object::Nil = right_side {
                    return Ok(true);
                }
                Ok(false)
            }
            Object::Boolean(left_boolean) => {
                if let Object::Boolean(right_boolean) = right_side {
                    return Ok(left_boolean == right_boolean);
                }
                return Err(error_message);
            }
            Object::Number(left_number) => {
                if let Object::Number(right_number) = right_side {
                    return Ok(left_number == right_number);
                }
                return Err(error_message);
            }
            Object::String(left_string) => {
                if let Object::String(right_string) = right_side {
                    return Ok(left_string == right_string);
                }
                return Err(error_message);
            }
        }
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_binary_expr(&self, expr: &Expr) -> Result<Object, RLoxError> {
        if let Expr::Binary {
            left,
            operator,
            right,
        } = expr
        {
            let left_resolved = self.evaluate(left)?;
            let right_resolved = self.evaluate(right)?;

            // The == and != checks work with any pair of objects, as long as both sides are the same type.
            if let TokenType::EqualEqual = operator.token_type {
                let result = self.is_equal(left_resolved, right_resolved);

                if let Err(err) = result {
                    return Err(RLoxError::InterpreterError(operator.clone(), err));
                } else {
                    return Ok(Object::Boolean(result.unwrap()));
                }
            } else if let TokenType::BangEqual = operator.token_type {
                let result = self.is_equal(left_resolved, right_resolved);

                if let Err(err) = result {
                    return Err(RLoxError::InterpreterError(operator.clone(), err));
                } else {
                    return Ok(Object::Boolean(!result.unwrap()));
                }
            }

            if let Ok((left_number, right_number)) = self.check_number_operands(
                operator.clone(),
                left_resolved.clone(),
                right_resolved.clone(),
            ) {
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
                    return Ok(Object::Number(number));
                } else if let Some(boolean) = return_bool {
                    return Ok(Object::Boolean(boolean));
                }
                panic!("Unexpected operator between numbers");
            }

            if let Ok((left_string, right_string)) =
                self.check_string_operands(operator.clone(), left_resolved, right_resolved)
            {
                let return_string = match operator.token_type {
                    TokenType::Plus => format!("{}{}", left_string, right_string),
                    _ => panic!("Expected arithmetic operators"),
                };

                return Ok(Object::String(return_string));
            }
            return Err(RLoxError::InterpreterError(
                operator.clone(),
                "The resolved values on right and left are not of the same type.".to_string(),
            ));
        }
        unreachable!()
    }

    fn visit_literal_expr(&self, expr: &Expr) -> Result<Object, RLoxError> {
        if let Expr::Literal { value } = expr {
            return Ok(value.clone());
        }
        panic!("Expected literal, got other value")
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Result<Object, RLoxError> {
        if let Expr::Grouping { expression } = expr {
            return self.evaluate(expression);
        }
        panic!("Expected grouping, got other value")
    }

    fn visit_unary_expr(&self, expr: &Expr) -> Result<Object, RLoxError> {
        if let Expr::Unary { operator, right } = expr {
            let right_side = self.evaluate(right)?;
            return match operator.token_type {
                TokenType::Bang => Ok(Object::Boolean(!self.is_truthy(right_side))),
                TokenType::Minus => {
                    // check if the right_side is a number
                    self.check_number_operand(operator.clone(), right_side.clone())?;
                    if let Object::Number(number) = right_side {
                        return Ok(Object::Number(-number));
                    }
                    panic!("Expected number got something else");
                }
                _ => panic!("Expected BANG or MINUS but got something else"),
            };
        }
        panic!("Expected grouping, got other value")
    }
}

use std::collections::HashMap;

use crate::{
    errors::RLoxError,
    tokens::{Object, Token},
};

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Object, RLoxError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        Err(RLoxError::InterpreterError(
            name,
            "Unknown variable used.".to_string(),
        ))
    }
}

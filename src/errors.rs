use crate::tokens::Token;

#[derive(Debug)]
pub enum RLoxError {
    ParseError(u64, String),         // line and message
    InterpreterError(Token, String), // operator and message
}

impl RLoxError {
    pub fn print(self) {
        match self {
            Self::InterpreterError(operator, message) => {
                println!("[Line {}] Error: {}", operator.line, message)
            }
            Self::ParseError(line, message) => println!("[Line {}] Error: {}", line, message),
        }
    }
}

pub fn rlox_error(line: u64, message: &str) {
    report(line, "", message);
}

pub fn report(line: u64, location: &str, message: &str) {
    println!("[Line {}] Error {}: {}", line, location, message);
}

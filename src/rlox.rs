use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
};

use anyhow::Result;
use clap::Parser;

use crate::{
    environment::Environment, errors::RLoxError, interpreter::Interpreter, scanner::Scanner,
    tokens::Object,
};

#[derive(Parser)]
#[command(name = "rLox")]
#[command(version = "1.0")]
#[command(about = "A Lox interpreter written in Rust", long_about = None)]
pub struct RLox {
    /// Path to the Lox file
    #[arg(long)]
    path: Option<String>,
}

impl RLox {
    /// initiate
    pub fn init(self) -> Result<()> {
        if self.path.is_some() {
            // path has been passed
            return self.run_file();
        } else {
            // interactive mode
            return self.run_prompt();
        }
    }

    fn run_file(self) -> Result<()> {
        // read file
        let file_bytes = fs::read_to_string(self.path.unwrap())?;
        if let Err(err) = Self::run(file_bytes) {
            err.print();
        }
        Ok(())
    }

    fn run_prompt(self) -> Result<()> {
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_string();

            if input == "quit" {
                break;
            }

            Self::run(input);
        }
        Ok(())
    }

    fn run(input: String) -> Result<(), RLoxError> {
        // lexing
        let mut scanner = Scanner::new(input);
        scanner.scan_tokens();

        // parsing
        let mut parser = crate::parser::Parser::new(scanner.tokens);
        let expressions = parser.parse()?;

        // interpreting
        let mut interpreter = Interpreter {
            environment: Environment::new(),
        };
        interpreter.interpret(expressions)?;
        Ok(())
    }
}

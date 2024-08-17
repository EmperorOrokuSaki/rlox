use std::{
    fs,
    io::{self, Write},
};

use anyhow::Result;
use clap::Parser;

use crate::scanner::Scanner;

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
        Self::run(file_bytes);
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

    fn run(input: String) {
        let mut scanner = Scanner::new(input);
        scanner.scan_tokens();
        for token in scanner.tokens {
            dbg!(token);
        }
    }
}

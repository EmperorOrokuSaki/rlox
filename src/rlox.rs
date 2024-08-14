use std::fs;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "rLox")]
#[command(version = "1.0")]
#[command(about = "A Lox compiler written in Rust", long_about = None)]
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
        Ok(())
    }

    fn run(input: String) {
        let tokens = Self::scan_tokens(input);
        for token in tokens {
            dbg!(token);
        }
    }

    fn scan_tokens(input: String) -> Result<()> {
        
        Ok(())
    }
}

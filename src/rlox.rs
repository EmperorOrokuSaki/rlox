use std::{
    fs,
    io::{self, Write},
};

use anyhow::Result;
use clap::Parser;

use crate::ast::expr::Expr;
use crate::{ast::print::AstPrinter, scanner::Scanner, tokens::Token};

#[derive(Parser)]
#[command(name = "rLox")]
#[command(version = "1.0")]
#[command(about = "A Lox interpreter written in Rust", long_about = None)]
pub struct RLox {
    /// Path to the Lox file
    #[arg(long)]
    path: Option<String>,
    /// Debug mode
    #[arg(short, long)]
    debug: bool,
}

impl RLox {
    /// initiate
    pub fn init(self) -> Result<()> {
        if self.path.is_some() {
            // path has been passed
            return self.run_file();
        } else if self.debug {
            return Self::debug();
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

    fn debug() -> Result<()> {
        let expressions = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token {
                    token_type: crate::tokens::TokenType::Minus,
                    lexeme: "-".to_string(),
                    literal: None,
                    line: 1,
                },
                right: Box::new(Expr::Literal {
                    value: "123".to_string(),
                }),
            }),
            operator: Token {
                token_type: crate::tokens::TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: "45.67".to_string(),
                }),
            }),
        };
        let printer = AstPrinter {};
        println!("{}", expressions.accept(&printer));
        Ok(())
    }
}

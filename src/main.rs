mod ast;
mod errors;
mod interpreter;
mod keywords;
mod parser;
mod rlox;
mod scanner;
mod tokens;
mod environment;

use anyhow::Result;
use clap::Parser;
use rlox::RLox;

fn main() -> Result<()> {
    let rlox = RLox::parse();
    rlox.init()
}

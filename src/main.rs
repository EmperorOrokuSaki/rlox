mod errors;
mod rlox;
mod scanner;
mod tokens;
mod keywords;

use anyhow::Result;
use clap::Parser;
use rlox::RLox;

fn main() -> Result<()> {
    let rlox = RLox::parse();
    rlox.init()
}

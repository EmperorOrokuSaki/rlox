mod errors;
mod keywords;
mod rlox;
mod scanner;
mod tokens;

use anyhow::Result;
use clap::Parser;
use rlox::RLox;

fn main() -> Result<()> {
    let rlox = RLox::parse();
    rlox.init()
}

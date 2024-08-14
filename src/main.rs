use clap::Parser;

#[derive(Parser)]
#[command(name = "rLox")]
#[command(version = "1.0")]
#[command(about = "A Lox compiler written in Rust", long_about = None)]
struct Cli {
    #[arg(long)]
    path: String,
}

fn main() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.path);
}

use std::process;
use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    eprintln!("unimplemented");
    process::exit(-1);
}

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },

    #[command(name = "rm")]
    Remove { key: String },
}

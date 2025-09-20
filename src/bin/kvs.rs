use std::path::Path;
use std::process::exit;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
struct Args {

    #[command(subcommand)]
    cmd: Command
}

#[derive(Subcommand)]
enum Command {
    Set {key: String, value: String},
    Get {key: String},

    #[command(name = "rm")]
    Remove {key: String}
}

fn main() {
    let args = Args::parse();
    eprint!("unimplemented");
    exit(-1);
}
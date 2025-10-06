use std::process::exit;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Args {

    #[command(subcommand)]
    cmd: Option<Command>
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
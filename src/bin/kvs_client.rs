use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    cmd: Command,

    #[arg(long, default_value = "127.0.0.1:4000")]
    addr: String,
}

#[derive(Subcommand)]
enum Command {
    Set {
        key: String,
        value: String,
    },
    Get {
        key: String,
    },

    #[command(name = "rm")]
    Remove {
        key: String,
    },
}

fn main() -> kvs::Result<()> {
    let args = Args::parse();

    Ok(())
}
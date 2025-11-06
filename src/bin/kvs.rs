use clap::{Parser, Subcommand};
use kvs::KvStore;
use std::env::current_dir;
use std::process;
use std::process::exit;

use kvs::Result;

fn main() {
    let cli = Cli::parse();

    match process(cli) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            exit(-1);
        }
    }
}

fn process(cli: Cli) -> Result<()> {
    let mut store = KvStore::open(current_dir()?)?;
    match cli.cmd {
        Commands::Set { key, value } => {
            store.set(key, value)?;
        }
        Commands::Get { key } => {
            let v = store.get(key)?.unwrap_or("Key not found".to_string());
            println!("{}", v);
        }
        Commands::Remove { key } => {
            store.remove(key)?;
        }
    }
    Ok(())
}

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

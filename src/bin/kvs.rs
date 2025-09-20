use clap::{Parser, Subcommand};
use kvs::KvStore;
use kvs::Result;
use std::env::current_dir;
use std::process::exit;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
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

fn main() -> Result<()> {
    let args = Args::parse();

    let dir = current_dir()?;
    let mut store = KvStore::open(&dir)?;

    let rst = match args.cmd {
        Command::Set { key, value } => store.set(key, value),
        Command::Get { key } => {
            let value = store.get(key)?;
            if let Some(str) = value {
                println!("{}", str);
            } else {
                println!("Key not found");
            }
            Ok(())
        }
        Command::Remove { key } => store.remove(key),
    };

    match rst {
        Err(e) => {
            println!("{}", e);
            exit(-1);
        }
        _ => {}
    }

    Ok(())
}
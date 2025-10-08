use std::env::current_dir;
use std::process::exit;
use clap::{Parser, Subcommand};
use kvs::KvStore;
use kvs::Result;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Set { key: String, value: String },
    Get { key: String },

    #[command(name = "rm")]
    Remove { key: String },
}

fn main() -> Result<()> {
    let args = Args::parse();

    let dir = current_dir()?;
    let mut store = KvStore::open(dir)?;

    if args.cmd.is_some() {
        let cmd = args.cmd.unwrap();
        match execute_command(store, cmd) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
                exit(-1);
            }
        }
    } else {
        exit(-1);
    }

    Ok(())
}

fn execute_command(mut store: KvStore, cmd: Command) -> Result<()> {
    match cmd {
        Command::Set {key, value} => {
            store.set(key, value)?;
        }
        Command::Remove {key} => {
            store.remove(key)?;
        }
        Command::Get {key} => {
            let v = store.get(key)?;
            match v {
                Some(v) => {
                    println!("{}", v);
                }
                None => {
                    println!("Key not found");
                }
            }
        }
    }

    Ok(())
}
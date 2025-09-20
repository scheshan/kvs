use std::env::current_dir;
use std::net::{TcpListener, TcpStream};
use clap::{Parser, ValueEnum};
use kvs::{KvStore, Result};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:4000")]
    addr: String,

    #[arg(long, value_enum, default_value = "kvs")]
    engine: Engine,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Engine {
    Kvs
}

fn main() -> Result<()> {
    let dir = current_dir()?;
    let store = KvStore::open(&dir)?;

    let args = Args::parse();
    let listener = TcpListener::bind(args.addr)?;
    loop {
        let stream = listener.accept()?;
    }

    Ok(())
}

fn handle(stream: TcpStream) {

}
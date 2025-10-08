use clap::{Parser, ValueEnum};
use std::net::TcpListener;
use std::process::exit;
use env_logger::Env;
use log::info;
use kvs::Result;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, value_enum)]
    engine: Option<Engine>,

    #[arg(long, default_value = "127.0.0.1:4000", required = false)]
    addr: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Engine {
    Kvs,
    Sled,
}

fn main() {
    init_logging();

    let args = Args::parse();

    info!("args: {:?}", args);

    match run(args) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            exit(-1);
        }
    }
}

fn run(args: Args) -> Result<()> {
    //let listener = TcpListener::bind(args.addr)?;
    Ok(())
}

fn init_logging() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}

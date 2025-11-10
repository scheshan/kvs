use anyhow::anyhow;
use clap::Parser;
use kvs::{KvStore, KvsEngine, KvsServer, Result, SledKvsEngine};
use std::env::current_dir;
use std::fs;
use std::fs::OpenOptions;
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use std::process::exit;
use env_logger::Target;
use log::{error, info, LevelFilter};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(long, default_value = "127.0.0.1:4000")]
    addr: String,

    #[arg(long, default_value = "kvs")]
    engine: String,
}

impl Cli {
    fn get_stored_engine(&self, dir: &PathBuf) -> Result<()> {
        if self.engine != "kvs" && self.engine != "sled" {
            return Err(anyhow!("Invalid engine"));
        }

        let path = dir.join("engine");
        if fs::exists(&path)? {
            let mut engine = String::new();
            let mut file = OpenOptions::new().read(true).open(&path)?;
            file.read_to_string(&mut engine)?;

            if engine != self.engine {
                return Err(anyhow!("Engine mismatch"));
            }
        } else {
            let mut file = OpenOptions::new().write(true).create(true).open(&path)?;
            file.write_all(self.engine.as_bytes())?;
        }
        Ok(())
    }
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let cli = Cli::parse();
    match run(cli) {
        Ok(_) => {}
        Err(err) => {
            error!("{}", err);
            exit(-1);
        }
    }
}

fn run(cli: Cli) -> Result<()> {
    let dir = current_dir()?;
    cli.get_stored_engine(&dir)?;

    info!("kvs version: {}", env!("CARGO_PKG_VERSION"));
    info!("Engine: {}", cli.engine);
    if cli.engine == "kvs" {
        KvsServer::run(cli.addr, KvStore::open(&dir)?)?;
    } else {
        KvsServer::run(cli.addr, SledKvsEngine::open(&dir)?)?;
    }

    Ok(())
}

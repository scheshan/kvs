use bytes::{Buf, Bytes, BytesMut};
use clap::{Parser, Subcommand};
use kvs::{Command, Response, Result};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;

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

        #[arg(long, default_value = "127.0.0.1:4000")]
        addr: String,
    },
    Get {
        key: String,

        #[arg(long, default_value = "127.0.0.1:4000")]
        addr: String,
    },

    #[command(name = "rm")]
    Remove {
        key: String,

        #[arg(long, default_value = "127.0.0.1:4000")]
        addr: String,
    },
}

impl Commands {
    fn addr(&self) -> &String {
        match self {
            Commands::Set { key, value, addr } => addr,
            Commands::Get { key, addr } => addr,
            Commands::Remove { key, addr } => addr,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("{}", e);
        exit(-1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let mut stream = TcpStream::connect(cli.cmd.addr())?;
    let mut is_get = false;
    let cmd = match cli.cmd {
        Commands::Set { key, value, addr } => Command::Set(key, value),
        Commands::Get { key, addr } => {
            is_get = true;
            Command::Get(key)
        }
        Commands::Remove { key, addr } => Command::Remove(key),
    };

    let buf: Bytes = cmd.into();
    stream.write_all(&buf)?;
    stream.flush()?;

    let res = read_frame(&mut stream)?;

    match res {
        Response::Success(None) => {
            if is_get {
                println!("Key not found");
            }
        }
        Response::Success(Some(str)) => {
            println!("{}", str);
        }
        Response::Error(str) => {
            eprintln!("{}", str);
            exit(-1);
        }
    }

    Ok(())
}

fn read_frame(stream: &mut TcpStream) -> Result<Response> {
    let mut buf = BytesMut::new();
    let mut read_buf = [0u8; 4096];
    loop {
        let n = stream.read(&mut read_buf[..])?;
        buf.extend_from_slice(&read_buf[..n]);

        if buf.len() < 8 {
            continue;
        }
        let len = u64::from_be_bytes([
            buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
        ]) as usize;
        if buf.len() < len + 8 {
            continue;
        }
        buf.advance(8);

        return Ok(Response::from(&buf[..len]));
    }
}

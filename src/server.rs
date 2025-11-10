use crate::KvsEngine;
use crate::Result;
use crate::common::{Command, Response};
use anyhow::anyhow;
use bytes::{Buf, Bytes, BytesMut};
use log::{error, info, warn};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct KvsServer {}

impl KvsServer {
    pub fn run<T>(addr: String, mut store: T) -> Result<()>
    where
        T: KvsEngine,
    {
        let listener = TcpListener::bind(&addr)?;
        info!("Server listening on addr: {}", &addr);
        loop {
            let (stream, _) = listener.accept()?;
            let mut handler = Handler::new(stream);
            loop {
                if let Some(cmd) = handler.read_frame()? {
                    let res = match cmd {
                        Command::Set(key, value) => match store.set(key, value) {
                            Ok(_) => Response::Success(None),
                            Err(e) => Response::Error(e.to_string()),
                        },
                        Command::Get(key) => match store.get(key) {
                            Ok(v) => Response::Success(v),
                            Err(e) => Response::Error(e.to_string()),
                        },
                        Command::Remove(key) => match store.remove(key) {
                            Ok(_) => Response::Success(None),
                            Err(e) => Response::Error(e.to_string()),
                        },
                    };
                    match handler.write_frame(res) {
                        Ok(_) => {}
                        Err(e) => {
                            warn!("write frame failed: {}", e);
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }
}

pub struct Handler {
    stream: TcpStream,
    read_buf: [u8; 4096],
    buf: BytesMut,
}

impl Handler {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            read_buf: [0u8; 4096],
            buf: BytesMut::new(),
        }
    }

    fn read_frame(&mut self) -> Result<Option<Command>> {
        loop {
            if let Some(cmd) = self.parse_frame() {
                return Ok(Some(cmd));
            }

            let n = self.stream.read(&mut self.read_buf[..]).unwrap_or(0);
            if n == 0 {
                if self.buf.len() > 0 {
                    warn!("Connection reset by peer")
                }
                return Ok(None);
            }

            self.buf.extend_from_slice(&self.read_buf[..n]);
        }
    }

    fn parse_frame(&mut self) -> Option<Command> {
        if self.buf.len() < 8 {
            return None;
        }

        let n = u64::from_be_bytes([
            self.buf[0],
            self.buf[1],
            self.buf[2],
            self.buf[3],
            self.buf[4],
            self.buf[5],
            self.buf[6],
            self.buf[7],
        ]) as usize;
        if self.buf.len() < n + 8 {
            return None;
        }

        self.buf.advance(8);
        let buf = &self.buf[..n];
        let cmd = Command::from(buf);
        self.buf.advance(n);
        Some(cmd)
    }

    fn write_frame(&mut self, res: Response) -> Result<()> {
        let buf: Bytes = res.into();
        self.stream.write_all(&buf)?;
        self.stream.flush()?;
        Ok(())
    }
}

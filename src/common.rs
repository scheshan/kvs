use std::net::TcpStream;
use anyhow::anyhow;
use bytes::{BufMut, Bytes, BytesMut};

fn read_str(data: &[u8]) -> (String, &[u8]) {
    let len_buf = [
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ];
    let len = u64::from_be_bytes(len_buf) as usize;

    let str_buf = &data[8..8 + len];
    let str = String::from_utf8_lossy(str_buf).to_string();

    let remain = &data[8 + len..];
    (str, remain)
}

fn write_str(v: &mut Vec<u8>, str: String) {
    let len = str.len() as u64;
    v.extend_from_slice(&len.to_be_bytes()[..]);
    v.extend_from_slice(str.as_bytes());
}

pub enum Command {
    Set(String, String),
    Get(String),
    Remove(String),
}

impl From<&[u8]> for Command {
    fn from(value: &[u8]) -> Self {
        let typ = value[0];
        let remain = &value[1..];

        let cmd = match typ {
            0 => {
                let (key, remain) = read_str(remain);
                let (value, _) = read_str(remain);
                Command::Set(key, value)
            }
            1 => {
                let (key, _) = read_str(remain);
                Command::Get(key)
            }
            2 => {
                let (key, _) = read_str(remain);
                Command::Remove(key)
            }
            _ => {
                panic!("invalid command type")
            }
        };
        cmd
    }
}

impl Into<Vec<u8>> for Command {
    fn into(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        match self {
            Command::Set(key, value) => {
                v.push(0);
                write_str(&mut v, key);
                write_str(&mut v, value);
            }
            Command::Get(key) => {
                v.push(1);
                write_str(&mut v, key);
            }
            Command::Remove(key) => {
                v.push(2);
                write_str(&mut v, key);
            }
        }

        v
    }
}

impl Into<Bytes> for Command {
    fn into(self) -> Bytes {
        let data : Vec<u8> = self.into();
        let mut b = BytesMut::with_capacity(data.len() + 8);
        b.put_u64(data.len() as u64);
        b.extend(data);

        b.freeze()
    }
}

pub enum Response {
    Success(Option<String>),
    Error(String),
}

impl From<&[u8]> for Response {
    fn from(value: &[u8]) -> Self {
        let typ = value[0];
        let remain = &value[1..];

        let res = match typ {
            0 => {
                let (str, _) = read_str(remain);
                if str.is_empty() {
                    Response::Success(None)
                } else {
                    Response::Success(Some(str))
                }
            }
            1 => {
                let (str, _) = read_str(remain);
                Response::Error(str)
            }
            _ => {
                panic!("invalid command type")
            }
        };
        res
    }
}

impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        match self {
            Response::Success(opt) => {
                v.push(0);
                let str = opt.unwrap_or(String::from(""));
                write_str(&mut v, str);
            }
            Response::Error(str) => {
                v.push(1);
                write_str(&mut v, str);
            }
        }

        v
    }
}

impl Into<Bytes> for Response {
    fn into(self) -> Bytes {
        let data : Vec<u8> = self.into();
        let mut b = BytesMut::with_capacity(data.len() + 8);
        b.put_u64(data.len() as u64);
        b.extend(data);

        b.freeze()
    }
}

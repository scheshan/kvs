mod cmd;
mod log;

use crate::cmd::Command;
use crate::log::{Position, Reader, Writer};
use std::collections::HashMap;
use std::fs::{create_dir_all, read_dir};
use std::path::PathBuf;
use anyhow::anyhow;

pub type Result<T> = anyhow::Result<T>;

pub struct KvStore {
    data: HashMap<String, Position>,
    readers: HashMap<u64, Reader>,
    writer: Writer,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        if !path.exists() {
            create_dir_all(&path)?;
        }

        let mut reader_list = Reader::load_exists(&path)?;
        let mut readers = HashMap::new();
        let mut data = HashMap::new();

        let mut id = 0u64;
        for mut reader in reader_list.into_iter() {
            reader.load(&mut data)?;
            if reader.id() > id {
                id = reader.id();
            }

            readers.insert(reader.id(), reader);
        }

        let writer = Writer::new(&path, id + 1)?;
        let reader = Reader::new(&path, writer.id())?;
        readers.insert(reader.id(), reader);

        Ok(Self {
            data,
            readers,
            writer,
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set(key.clone(), value.clone());
        let pos = self.writer.write(cmd)?;

        self.data.insert(key, pos);

        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let pos = self.data.get(&key);
        if pos.is_none() {
            return Ok(None);
        }
        let pos = pos.unwrap();

        let reader = self.readers.get_mut(&pos.id());
        if reader.is_none() {
            return Ok(None);
        }
        let mut reader = reader.unwrap();

        let cmd = reader.read(pos.pos())?;
        if cmd.is_none() {
            return Ok(None);
        }
        let cmd = cmd.unwrap();

        match cmd {
            Command::Set(_, value) => Ok(Some(value)),
            _ => Ok(None),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.data.contains_key(&key) {
            return Err(anyhow!("Key not found"))
        }

        let cmd = Command::Remove(key.clone());
        self.writer.write(cmd)?;

        self.data.remove(&key);
        Ok(())
    }
}

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
    un_compacted: usize,
    dir: PathBuf,
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
            dir: path,
            data,
            readers,
            writer,
            un_compacted: 0,
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set(key.clone(), value.clone());
        let pos = self.writer.write(cmd)?;

        if let Some(_) = self.data.insert(key, pos) {
            self.un_compacted += 1;
            self.compact()?;
        }

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

        if let Some(_) = self.data.remove(&key) {
            self.un_compacted += 1;
            self.compact()?;
        }
        Ok(())
    }

    fn compact(&mut self) -> Result<()> {
        if self.un_compacted * 10 < self.data.len() {
            return Ok(())
        }

        let writer_id = self.writer.id() + 2;
        let compact_id = self.writer.id() + 1;

        //rotate new writer
        let writer = Writer::new(&self.dir, writer_id)?;
        let reader = Reader::new(&self.dir, writer_id)?;
        self.writer = writer;
        self.readers.insert(reader.id(), reader);

        //compact
        let mut compact_writer = Writer::new(&self.dir, compact_id)?;
        let compact_reader = Reader::new(&self.dir, compact_id)?;
        self.readers.insert(compact_reader.id(), compact_reader);

        for (_, value) in self.data.iter_mut() {
            if let Some(reader) = self.readers.get_mut(&value.id()) {
                if let Some(cmd) = reader.read(value.pos())? {
                    let new_pos = compact_writer.write(cmd)?;
                    *value = new_pos;
                }
            }
        }

        let deleted_id_list : Vec<u64> = self.readers.keys()
            .filter(|&&id| id < compact_id)
            .map(|&id| id)
            .collect();

        for id in deleted_id_list {
            self.readers.remove(&id);
            log::remove_file(&self.dir, id)?;
        }

        self.un_compacted = 0;

        Ok(())
    }
}

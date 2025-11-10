use crate::common::Command;
use crate::engine::KvsEngine;
use crate::log::{LogPosition, LogReader, LogWriter};
use anyhow::anyhow;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct KvStore {
    dir: PathBuf,
    data: HashMap<String, LogPosition>,
    writer: LogWriter,
    readers: HashMap<u64, LogReader>,
    id: u64,
    dirty_size: usize,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> crate::Result<Self> {
        let dir = path.into();

        let reader_list = LogReader::open(&dir)?;
        let mut next_id = 0;
        let mut data = HashMap::new();
        let mut readers = HashMap::new();

        for mut reader in reader_list.into_iter() {
            reader.load_all(|cmd, pos| match cmd {
                Command::Set(key, _) => {
                    data.insert(key, pos);
                }
                Command::Get(_) => {}
                Command::Remove(key) => {
                    data.remove(&key);
                }
            })?;
            next_id = reader.id();
            readers.insert(reader.id(), reader);
        }

        next_id += 1;
        let writer = LogWriter::new(&dir, next_id)?;
        let reader = LogReader::new(&dir, next_id)?;
        readers.insert(reader.id(), reader);

        Ok(Self {
            dir,
            data,
            writer,
            readers,
            id: next_id,
            dirty_size: 0,
        })
    }

    fn try_compact(&mut self) -> crate::Result<()> {
        if self.dirty_size < self.data.len() {
            return Ok(());
        }

        self.compact()
    }

    fn compact(&mut self) -> crate::Result<()> {
        self.id += 1;
        let mut new_writer = LogWriter::new(&self.dir, self.id)?;
        let mut new_data = HashMap::new();

        for (key, pos) in self.data.iter() {
            let reader = self.readers.get_mut(&pos.id()).unwrap();
            let cmd = reader.read(pos.pos())?;

            if let Some(Command::Set(key, value)) = cmd {
                let pos = new_writer.write(Command::Set(key.clone(), value))?;
                new_data.insert(key, pos);
            }
        }
        new_writer.flush()?;

        for (id, reader) in self.readers.iter_mut() {
            reader.remove()?;
        }
        self.readers.clear();

        self.readers
            .insert(new_writer.id(), LogReader::new(&self.dir, new_writer.id())?);

        self.id += 1;
        self.writer = LogWriter::new(&self.dir, self.id)?;
        self.readers.insert(
            self.writer.id(),
            LogReader::new(&self.dir, self.writer.id())?,
        );
        self.data = new_data;
        self.dirty_size = 0;

        Ok(())
    }
}

impl KvsEngine for KvStore {
    fn set(&mut self, key: String, value: String) -> crate::Result<()> {
        let cmd = Command::Set(key.clone(), value);
        let pos = self.writer.write_and_flush(cmd)?;
        let old = self.data.insert(key, pos);
        if old.is_some() {
            self.dirty_size += 1;
            self.try_compact()?;
        }

        Ok(())
    }

    fn get(&mut self, key: String) -> crate::Result<Option<String>> {
        let pos = self.data.get(&key);
        if pos.is_none() {
            return Ok(None);
        }
        let pos = pos.unwrap();

        let reader = self.readers.get_mut(&pos.id());
        if reader.is_none() {
            panic!("invalid key");
        }
        let reader = reader.unwrap();

        let cmd = reader.read(pos.pos())?;
        if cmd.is_none() {
            panic!("invalid key");
        }
        let cmd = cmd.unwrap();

        match cmd {
            Command::Set(_, value) => Ok(Some(value)),
            _ => Ok(None),
        }
    }

    fn remove(&mut self, key: String) -> crate::Result<()> {
        let cmd = Command::Remove(key.clone());
        self.writer.write_and_flush(cmd)?;

        let option = self.data.remove(&key);
        if option.is_none() {
            return Err(anyhow!("Key not found"));
        }
        self.dirty_size += 1;
        self.try_compact()?;

        Ok(())
    }
}
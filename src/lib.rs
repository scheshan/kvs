mod record;
mod log;

use crate::record::Record;
use crate::log::{LogReader, LogWriter};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::anyhow;

pub type Result<T> = anyhow::Result<T>;

pub(crate) struct LogPosition {
    id: u64,
    pos: usize,
}

impl LogPosition {
    pub fn new(id: u64, pos: usize) -> Self {
        Self { id, pos }
    }
}

pub struct KvStore {
    readers: HashMap<u64, LogReader>,
    writer: LogWriter,
    hm: HashMap<String, LogPosition>,
    id: u64,
    dir: PathBuf,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<Self> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }

        let mut readers = HashMap::<u64, LogReader>::new();
        let mut hm = HashMap::<String, LogPosition>::new();
        let exist_readers = log::LogReader::load_exist(path)?;

        let mut id = 0u64;
        for reader in exist_readers.into_iter() {
            if reader.id() > id {
                id = reader.id();
            }

            let record_iter = reader.try_iter()?;
            for r in record_iter {
                let (record, pos) = r?;
                let Record {
                    typ,
                    key,
                    ..
                } = record;

                if typ == 0 {
                    hm.insert(key, LogPosition::new(reader.id(), pos));
                } else if typ == 2 {
                    hm.remove(&key);
                }
            }

            readers.insert(reader.id(), reader);
        }

        id += 1;
        let writer = LogWriter::new(path, id)?;
        let reader = LogReader::from(&writer.path(), id)?;
        readers.insert(id, reader);

        Ok(Self {
            readers,
            writer,
            hm,
            id,
            dir: path.to_path_buf()
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let record = Record::set(key.clone(), value);
        let pos = self.writer.write(record)?;
        self.hm.insert(key, LogPosition::new(self.writer.id(), pos));

        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let o = match self.hm.get(&key) {
            None => None,
            Some(pos) => {
                let o = self.readers.get_mut(&pos.id);
                match o {
                    None => None,
                    Some(reader) => {
                        let record = reader.read(pos.pos)?;
                        if record.typ == 0 {
                            let Record {
                                value,
                                ..
                            } = record;
                            Some(value)
                        } else {
                            None
                        }
                    }
                }
            }
        };
        Ok(o)
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.hm.contains_key(&key) {
            return Err(anyhow!("Key not found"))
        }

        let record = Record::remove(key.clone());
        self.writer.write(record)?;
        self.hm.remove(&key);
        Ok(())
    }
}

mod frame;
mod log;

use crate::frame::Frame;
use crate::log::{LogReader, LogWriter};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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

            let frame_reader = reader.try_iter()?;
            for r in frame_reader {
                let frame = r?;
                match frame {
                    (Frame::Set(key, _), pos) => {
                        hm.insert(key, LogPosition::new(reader.id(), pos));
                    }
                    (Frame::Remove(key), _) => {hm.remove(&key);}
                    _ => {}
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
        let frame = Frame::Set(key.clone(), value);
        let pos = self.writer.write(frame)?;
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
                        let frame = reader.read(pos.pos)?;
                        match frame {
                            Frame::Set(key, value) => Some(value),
                            _ => None,
                        }
                    }
                }
            }
        };
        Ok(o)
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let frame = Frame::Remove(key.clone());
        self.writer.write(frame)?;
        self.hm.remove(&key);
        Ok(())
    }
}

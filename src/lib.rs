mod command;
mod log;

use crate::command::Command;
use crate::log::{LogPosition, LogReader, LogWriter};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::result;

pub type KvsError = Box<dyn Error>;
pub type Result<T> = result::Result<T, KvsError>;

pub struct KvStore {
    dir: PathBuf,
    data: HashMap<String, LogPosition>,
    writer: LogWriter,
    readers: HashMap<u64, LogReader>,
    id: u64,
    dirty_size: usize,
}

#[derive(Debug)]
pub struct KeyNotFoundError {}

impl Display for KeyNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key not found")
    }
}

impl Error for KeyNotFoundError {}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
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

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set(key.clone(), value);
        let pos = self.writer.write_and_flush(cmd)?;
        let old = self.data.insert(key, pos);
        if old.is_some() {
            self.dirty_size += 1;
            self.try_compact()?;
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

    pub fn remove(&mut self, key: String) -> Result<()> {
        let cmd = Command::Remove(key.clone());
        self.writer.write_and_flush(cmd)?;

        let option = self.data.remove(&key);
        if option.is_none() {
            return Err(Box::new(KeyNotFoundError {}));
        }
        self.dirty_size += 1;
        self.try_compact()?;

        Ok(())
    }

    fn try_compact(&mut self) -> Result<()> {
        if self.dirty_size < self.data.len() {
            return Ok(());
        }

        self.compact()
    }

    fn compact(&mut self) -> Result<()> {
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

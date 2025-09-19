mod frame;
mod wal;

use crate::frame::Frame;
use crate::wal::{LogPosition, WAL};
use std::collections::HashMap;
use std::path::Path;

pub type Result<T> = anyhow::Result<T>;

pub struct KvStore {
    hm: HashMap<String, LogPosition>,
    wal: WAL,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<Self> {
        let wal = WAL::new(path)?;
        Ok(Self {
            hm: HashMap::new(),
            wal,
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let frame = Frame::Set(key.clone(), value);
        let pos = self.wal.insert(frame)?;
        self.hm.insert(key, pos);

        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let o = match self.hm.get(&key) {
            None => None,
            Some(pos) => {
                let frame = self.wal.get(pos)?;
                match frame {
                    Frame::Set(key, value) => Some(value),
                    _ => None,
                }
            }
        };
        Ok(o)
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let frame = Frame::Remove(key.clone());
        self.wal.insert(frame)?;
        self.hm.remove(&key);
        Ok(())
    }
}

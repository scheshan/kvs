use crate::KvsEngine;
use crate::Result;
use sled::Db;
use std::path::PathBuf;
use anyhow::anyhow;

pub struct SledKvsEngine {
    db: Db,
}

impl SledKvsEngine {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let db = sled::open(&path.into())?;
        Ok(Self { db })
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key, value.as_bytes())?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        let opt = self.db.get(&key)?;
        match opt {
            Some(v) => Ok(Some(String::from_utf8_lossy(&v).to_string())),
            None => Ok(None),
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let option = self.db.remove(&key)?;
        if option.is_none() {
            return Err(anyhow!("Key not found"));
        }
        Ok(())
    }
}

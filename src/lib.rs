use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub type Result<T> = anyhow::Result<T>;

pub struct KvStore {
    hm: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> Self {
        Self{
            hm: HashMap::new()
        }
    }

    pub fn open(path: &Path) -> Result<Self> {
        Ok(Self::new())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.hm.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.hm.get(&key).map(|str| str.clone()))
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.hm.remove(&key);
        Ok(())
    }
}
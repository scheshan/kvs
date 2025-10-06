use std::collections::HashMap;

pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.data.get(&key) {
            None => None,
            Some(str) => Some(str.clone()),
        }
    }

    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}

use std::collections::HashMap;

pub struct KvStore {
    hm: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> Self {
        Self{
            hm: HashMap::new()
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.hm.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.hm.get(&key).map(|str| str.clone())
    }

    pub fn remove(&mut self, key: String) {
        self.hm.remove(&key);
    }
}
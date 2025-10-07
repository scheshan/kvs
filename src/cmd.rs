pub enum Command {
    Set(String, String),
    Get(String),
    Remove(String),
}

impl Command {
    pub fn to_bytes(&self) -> Vec<u8> {
        let len = self.data_len();

        let mut buf = Vec::<u8>::with_capacity(len + 9);
        Self::write_u64(&mut buf, len as u64);

        match self {
            Command::Set(key, value) => {
                buf.push(1);
                Self::write_string(&mut buf, key);
                Self::write_string(&mut buf, value);
            }
            Command::Get(key) => {
                buf.push(2);
                Self::write_string(&mut buf, key);
            }
            Command::Remove(key) => {
                buf.push(3);
                Self::write_string(&mut buf, key);
            }
        }

        buf
    }

    fn data_len(&self) -> usize {
        match self {
            Command::Set(key, value) => 8 + key.as_bytes().len() + 8 + value.as_bytes().len(),
            Command::Get(key) => 8 + key.as_bytes().len(),
            Command::Remove(key) => 8 + key.as_bytes().len(),
        }
    }

    fn write_u64(v: &mut Vec<u8>, n: u64) {
        v.extend_from_slice(&n.to_be_bytes())
    }

    fn write_string(v: &mut Vec<u8>, str: &String) {
        Self::write_u64(v, str.len() as u64);
        v.extend_from_slice(str.as_bytes());
    }
}

pub enum Command {
    Set(String, String),
    Get(String),
    Remove(String),
}

impl Command {
    pub fn to_bytes(&self) -> Vec<u8> {
        let len = self.data_len();

        let mut buf = Vec::<u8>::with_capacity(len + 1);

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

    pub fn from_bytes(buf: &Vec<u8>) -> Self {
        let typ = buf[0];
        let buf = &buf[1..];

        match typ {
            1 => {
                let (key, buf) = Self::read_string(&buf);
                let (value, buf) = Self::read_string(&buf);
                Self::Set(key, value)
            }
            2 => {
                let (key, buf) = Self::read_string(&buf);
                Self::Get(key)
            }
            3 => {
                let (key, buf) = Self::read_string(&buf);
                Self::Remove(key)
            }
            _ => {
                panic!("invalid format")
            }
        }
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

    fn read_u64(buf: &[u8]) -> (u64, &[u8]) {
        let (data_buf, rest) = buf.split_at(8);
        let len = u64::from_be_bytes(data_buf.try_into().unwrap());
        (len, rest)
    }

    fn read_string(buf: &[u8]) -> (String, &[u8]) {
        let (len, rest) = Self::read_u64(buf);
        let (data_buf, rest) = rest.split_at(len as usize);
        let str = String::from_utf8_lossy(data_buf).to_string();
        (str, rest)
    }
}

#[test]
fn command_serialize_and_deserialize() {
    let mut cmd = Command::Set("hello".to_string(), "world".to_string());
    let mut buf = cmd.to_bytes();
    cmd = Command::from_bytes(&buf);

    match cmd {
        Command::Set(k, v) => {
            assert_eq!(k, "hello");
            assert_eq!(v, "world");
        }
        _ => {
            panic!("test failed")
        }
    }

    cmd = Command::Get("hello2".to_string());
    buf = cmd.to_bytes();
    cmd = Command::from_bytes(&buf);
    match cmd {
        Command::Get(k) => {
            assert_eq!(k, "hello2");
        }
        _ => {
            panic!("test failed")
        }
    }

    cmd = Command::Remove("hello3".to_string());
    buf = cmd.to_bytes();
    cmd = Command::from_bytes(&buf);
    match cmd {
        Command::Remove(k) => {
            assert_eq!(k, "hello3");
        }
        _ => {
            panic!("test failed")
        }
    }
}
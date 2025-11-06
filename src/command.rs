pub enum Command {
    Set(String, String),
    Get(String),
    Remove(String),
}

impl TryFrom<&[u8]> for Command {
    type Error = crate::KvsError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let typ = value[0];
        let remain = &value[1..];

        let cmd = match typ {
            0 => {
                let (key, remain) = Self::read_str(remain);
                let (value, _) = Self::read_str(remain);
                Command::Set(key, value)
            }
            1 => {
                let (key, _) = Self::read_str(remain);
                Command::Get(key)
            }
            2 => {
                let (key, _) = Self::read_str(remain);
                Command::Remove(key)
            }
            _ => {
                panic!("invalid command type")
            }
        };
        Ok(cmd)
    }
}

impl Into<Vec<u8>> for Command {
    fn into(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        match self {
            Command::Set(key, value) => {
                v.push(0);
                Self::write_str(&mut v, key);
                Self::write_str(&mut v, value);
            }
            Command::Get(key) => {
                v.push(1);
                Self::write_str(&mut v, key);
            }
            Command::Remove(key) => {
                v.push(2);
                Self::write_str(&mut v, key);
            }
        }

        v
    }
}

impl Command {
    fn read_str(data: &[u8]) -> (String, &[u8]) {
        let len_buf = [
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ];
        let len = u64::from_be_bytes(len_buf) as usize;

        let str_buf = &data[8..8 + len];
        let str = String::from_utf8_lossy(str_buf).to_string();

        let remain = &data[8 + len..];
        (str, remain)
    }

    fn write_str(v: &mut Vec<u8>, str: String) {
        let len = str.len() as u64;
        v.extend_from_slice(&len.to_be_bytes()[..]);
        v.extend_from_slice(str.as_bytes());
    }
}

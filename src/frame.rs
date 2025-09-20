use crate::Result;
use std::fs::File;
use std::io::{Read, Write};

pub struct Frame {
    pub(crate) typ: u8,
    pub(crate) key: String,
    pub(crate) value: String,
}

const EMPTY_STRING: &str = "";

impl Frame {
    pub fn set(key: String, value: String) -> Self {
        Self { typ: 0, key, value }
    }

    pub fn get(key: String) -> Self {
        Self {
            typ: 1,
            key,
            value: EMPTY_STRING.to_string(),
        }
    }

    pub fn remove(key: String) -> Self {
        Self {
            typ: 2,
            key,
            value: EMPTY_STRING.to_string(),
        }
    }

    pub fn write(&self, writer: &mut impl Write) -> Result<usize> {
        let mut res = 0usize;

        //write type
        writer.write(&[self.typ])?;
        res += 1;

        //write key and value length
        res += writer.write(&self.key.len().to_be_bytes())?;
        res += writer.write(&self.value.len().to_be_bytes())?;

        //write key and value data
        res += writer.write(self.key.as_bytes())?;
        res += writer.write(self.value.as_bytes())?;

        Ok(res)
    }

    pub fn read(reader: &mut impl Read) -> Result<(Self, usize)> {
        let mut header = [0u8; 1 + 2 * size_of::<usize>()];
        let x = reader.read_exact(&mut header)?;

        let typ = header[0];
        let key_arr = &header[1..(1 + size_of::<usize>())];
        let key_length = usize::from_be_bytes(key_arr.try_into().unwrap());
        let value_arr = &header[(1 + size_of::<usize>())..];
        let value_length = usize::from_be_bytes(value_arr.try_into().unwrap());

        let mut vec = Vec::<u8>::with_capacity(value_length);
        reader.read_exact(vec.as_mut_slice())?;

        let (key_data, value_data) = vec.split_at(key_length);

        let key = String::from_utf8(key_data.to_vec())?;
        let value = String::from_utf8(value_data.to_vec())?;
        
        let total_len = header.len() + key_length + value_length;

        Ok((Self { typ, key, value }, total_len))
    }
}

pub(crate) struct FrameReader {
    pos: usize,
    file: File,
}

impl FrameReader {
    pub fn new(file: File) -> Self {
        Self {
            file,
            pos: 0,
        }
    }

    fn read_frame(&mut self) -> Result<Frame> {
        unimplemented!()
    }

    fn read_string(&mut self, len: usize) -> Result<String> {
        unimplemented!()
    }
}

impl Iterator for FrameReader {
    type Item = Result<(Frame, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;

        let rst = Frame::read(&mut self.file);
        match rst {
            Ok((frame, len)) => {
                self.pos += pos;
                Some(Ok((frame, pos)))
            }
            Err(e) => {
                None
            }
        }
    }
}

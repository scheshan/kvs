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
        Self {
            typ: 0,
            key,
            value,
        }
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

    pub fn read(reader: &impl Read) -> Result<Self> {
        unimplemented!()
    }
}

pub(crate) struct FrameReader {
    pos: usize,
    file: File,
    header_buf: [u8; 9],
}

impl FrameReader {
    pub fn new(file: File) -> Self {
        Self {
            file,
            pos: 0,
            header_buf: [0u8; 9],
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
        unimplemented!()
    }
}

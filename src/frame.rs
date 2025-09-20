use crate::Result;
use std::fs::File;
use std::io;
use std::io::{BufReader, ErrorKind, Read, Write};

pub enum Frame {
    Set(String, String),
    Get(String),
    Remove(String),
}

impl Frame {
    pub fn write(&self, writer: &impl Write) -> Result<usize> {
        unimplemented!()
    }

    pub fn read(reader: &impl Read) -> Result<Self> {
        unimplemented!()
    }
}

pub(crate) struct FrameReader {
    pos: usize,
    file: File,
    header_buf: [u8; 9]
}

impl FrameReader {
    pub fn new(file: File) -> Self {
        Self { file, pos: 0, header_buf: [0u8; 9] }
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

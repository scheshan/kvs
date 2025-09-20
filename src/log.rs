use crate::Result;
use crate::frame::Frame;
use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Seek};
use std::path::{Path, PathBuf};

pub(crate) struct LogReader {
    id: u64,
    file: File,
}

impl LogReader {
    pub fn from(path: &Path, id: u64) -> Result<Self> {
        unimplemented!()
    }

    pub fn read(&mut self, pos: usize) -> Result<Frame> {
        unimplemented!()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn try_clone(&self) -> Result<Self> {
        let file = self.file.try_clone()?;
        Ok(Self { id: self.id, file })
    }

    pub fn load_exist(dir: &Path) -> Result<Vec<LogReader>> {
        unimplemented!()
    }
}

impl Iterator for LogReader {
    type Item = Result<(Frame, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

pub(crate) struct LogWriter {
    id: u64,
    path: PathBuf,
    w: BufWriter<File>,
    pos: usize,
}

impl LogWriter {
    pub fn new(dir: &Path, id: u64) -> Result<Self> {
        let path = dir.join(format!("{}.bin", id));
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.clone())?;
        let writer = BufWriter::new(file);

        Ok(Self {
            id,
            path,
            w: writer,
            pos: 0,
        })
    }

    pub fn write(&mut self, frame: Frame) -> Result<usize> {
        unimplemented!()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

use crate::cmd::Command;
use crate::Result;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;

pub struct Position {
    id: u64,
    pos: usize,
}

impl Position {
    pub fn new(id: u64, pos: usize) -> Self {
        Self { id, pos }
    }
}

pub struct Writer {
    id: u64,
    file: File,
    pos: usize,
}

impl Writer {
    pub fn new(dir: &PathBuf, id: u64) -> Result<Self> {
        let path = log_file_path(dir, id);
        let file = OpenOptions::new().create_new(true).write(true).open(path)?;

        Ok(Self { id, file, pos: 0 })
    }

    pub fn write(&mut self, cmd: Command) -> Result<Position> {
        let pos = self.pos;
        let buf = cmd.to_bytes();
        self.file.write(&buf)?;
        self.pos += buf.len();

        let pos = Position::new(self.id, pos);
        Ok(pos)
    }
}

pub struct Reader {
    id: u64,
    file: File,
}

impl Reader {
    pub fn new(dir: &PathBuf, id: u64) -> Result<Self> {
        let path = log_file_path(dir, id);
        let file = OpenOptions::new().read(true).open(path)?;

        Ok(Self { id, file })
    }

    pub fn read(&mut self, pos: usize) -> Result<Option<Command>> {
        self.file.seek(SeekFrom::Start(pos as u64))?;


        todo!()
    }
}

fn log_file_path(dir: &PathBuf, id: u64) -> PathBuf {
    dir.join(format!("{}.bin", id))
}

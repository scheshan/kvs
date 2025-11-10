use crate::Result;
use crate::common::Command;
use crate::log::{LOG_FILE_EXTENSION, LogPosition};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub struct LogWriter {
    id: u64,
    pos: usize,
    file: File,
}

impl LogWriter {
    pub fn new(dir: &PathBuf, id: u64) -> Result<Self> {
        let path = dir.join(format!("{}.{}", id, LOG_FILE_EXTENSION));

        let file = OpenOptions::new().create(true).write(true).open(path)?;

        Ok(Self { id, file, pos: 0 })
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn write(&mut self, cmd: Command) -> Result<LogPosition> {
        let pos = LogPosition::new(self.id, self.pos);
        let data: Vec<u8> = cmd.into();

        let len_buf = (data.len() as u64).to_be_bytes();
        self.file.write_all(&len_buf)?;
        self.pos += 8;

        self.file.write_all(&data)?;
        self.pos += data.len();

        Ok(pos)
    }

    pub fn write_and_flush(&mut self, cmd: Command) -> Result<LogPosition> {
        let pos = self.write(cmd)?;
        self.flush()?;
        Ok(pos)
    }

    pub fn flush(&mut self) -> Result<()> {
        self.file.sync_all()?;
        Ok(())
    }
}

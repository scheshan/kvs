use std::fs::{File, OpenOptions, read_dir, remove_file};
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::Result;
use crate::command::Command;
use crate::log::{LOG_FILE_EXTENSION, LogPosition};

pub struct LogReader {
    id: u64,
    path: PathBuf,
    file: File,
    pos: usize,
    len_buf: [u8; 8],
}

impl LogReader {
    pub fn open(dir: &PathBuf) -> Result<Vec<Self>> {
        let mut res = Vec::new();

        for entry in read_dir(dir)? {
            let entry = entry?;
            if entry.metadata()?.is_dir() {
                continue;
            }

            let path = entry.path();
            if path.extension().unwrap_or_default() != LOG_FILE_EXTENSION {
                continue;
            }

            let id = path
                .file_stem()
                .map(|s| s.to_str().unwrap())
                .unwrap()
                .parse::<u64>()?;

            let reader = Self::new(dir, id)?;
            res.push(reader);
        }

        res.sort_by_key(|r| r.id);
        Ok(res)
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn new(dir: &PathBuf, id: u64) -> Result<Self> {
        let path = dir.join(format!("{}.{}", id, LOG_FILE_EXTENSION));
        let file = OpenOptions::new().read(true).open(&path)?;

        Ok(Self {
            id,
            path,
            file,
            pos: 0,
            len_buf: [0u8; 8],
        })
    }

    pub fn read(&mut self, pos: usize) -> Result<Option<Command>> {
        self.seek(pos)?;

        self.pos += 8;
        let size = self.file.read(&mut self.len_buf[..])?;
        if size < self.len_buf.len() {
            if size == 0 {
                return Ok(None);
            } else {
                panic!("invalid file")
            }
        }

        let len = u64::from_be_bytes(self.len_buf) as usize;

        self.pos += len;
        let mut content = vec![0u8; len];
        self.file.read_exact(&mut content[..])?;

        let cmd = Command::try_from(content.as_slice())?;
        Ok(Some(cmd))
    }

    pub fn load_all<F>(&mut self, mut f: F) -> Result<()>
    where
        F: FnMut(Command, LogPosition),
    {
        self.seek(0)?;

        loop {
            let pos = LogPosition::new(self.id, self.pos);
            let opt = self.read(self.pos)?;
            match opt {
                Some(cmd) => f(cmd, pos),
                None => return Ok(()),
            }
        }
    }

    fn seek(&mut self, pos: usize) -> Result<()> {
        if self.pos != pos {
            self.pos = pos;
            self.file.seek(SeekFrom::Start(pos as u64))?;
        }

        Ok(())
    }

    pub fn remove(&self) -> Result<()> {
        remove_file(&self.path)?;
        Ok(())
    }
}

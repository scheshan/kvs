use crate::Result;
use crate::cmd::Command;
use anyhow::anyhow;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions, read_dir};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

pub struct Position {
    id: u64,
    pos: usize,
}

impl Position {
    pub fn new(id: u64, pos: usize) -> Self {
        Self { id, pos }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}

pub struct Writer {
    id: u64,
    w: File,
    pos: usize,
}

impl Writer {
    pub fn new(dir: &PathBuf, id: u64) -> Result<Self> {
        let path = log_file_path(dir, id);
        let file = OpenOptions::new().create_new(true).write(true).open(path)?;

        Ok(Self {
            id,
            w: file,
            pos: 0,
        })
    }

    pub fn write(&mut self, cmd: Command) -> Result<Position> {
        let pos = self.pos;
        let buf = cmd.to_bytes();
        self.w.write_all(&buf.len().to_be_bytes())?;
        self.w.write_all(&buf)?;
        self.w.flush()?;
        self.w.sync_all()?;

        self.pos += buf.len() + 8;

        let pos = Position::new(self.id, pos);
        Ok(pos)
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

pub struct Reader {
    id: u64,
    file: File,
    pos: usize,
}

impl Reader {
    pub fn new(dir: &PathBuf, id: u64) -> Result<Self> {
        let path = log_file_path(dir, id);
        let file = OpenOptions::new().read(true).open(path)?;

        Ok(Self { id, file, pos: 0 })
    }

    pub fn read(&mut self, pos: usize) -> Result<Option<Command>> {
        self.seek(pos)?;
        self.read_at_current()
    }

    fn seek(&mut self, pos: usize) -> Result<()> {
        if self.pos != pos {
            self.file.seek(SeekFrom::Start(pos as u64))?;
            self.pos = pos;
        }

        Ok(())
    }

    fn read_at_current(&mut self) -> Result<Option<Command>> {
        let mut len_buf = [0u8; 8];
        let size = self.file.read(&mut len_buf)?;
        if size == 0 {
            //EOF
            return Ok(None);
        } else if size < len_buf.len() {
            return Err(anyhow!("invalid file format"));
        }

        let len = u64::from_be_bytes(len_buf);
        let mut buf = vec![0u8; len as usize];
        self.file.read_exact(&mut buf)?;

        self.pos += 8 + buf.len();

        Ok(Some(Command::from_bytes(&buf)))
    }

    pub fn load(&mut self, data: &mut HashMap<String, Position>) -> Result<()> {
        self.seek(0)?;

        loop {
            let pos = self.pos;
            let opt = self.read_at_current()?;
            match opt {
                Some(cmd) => match cmd {
                    Command::Set(key, value) => {
                        let pos = Position::new(self.id, pos);
                        data.insert(key, pos);
                    }
                    Command::Remove(key) => {
                        data.remove(&key);
                    }
                    _ => {}
                },
                None => {
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn load_exists(dir: &PathBuf) -> Result<Vec<Self>> {
        let mut res = Vec::new();

        let entries = read_dir(&dir)?;
        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if !metadata.is_file() || !entry.path().to_str().unwrap().ends_with(".bin") {
                continue;
            }

            let id_str = entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let id: u64 = id_str.parse()?;
            res.push(Self::new(dir, id)?);
        }

        res.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(res)
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

fn log_file_path(dir: &PathBuf, id: u64) -> PathBuf {
    dir.join(format!("{}.bin", id))
}

pub fn remove_file(dir: &PathBuf, id: u64) -> Result<()> {
    fs::remove_file(log_file_path(dir, id))?;
    
    Ok(())
}
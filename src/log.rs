use crate::Result;
use crate::frame::{Frame, FrameReader};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

const LOG_FILE_EXTENSION: &str = ".bin";

pub(crate) struct LogReader {
    id: u64,
    file: File,
}

impl LogReader {
    pub fn from(path: &Path, id: u64) -> Result<Self> {
        let file = OpenOptions::new().read(true).open(path)?;

        Ok(Self { id, file })
    }

    pub fn read(&mut self, pos: usize) -> Result<Frame> {
        self.file.seek(SeekFrom::Start(pos as u64))?;
        Frame::read(&self.file)
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn try_clone(&self) -> Result<Self> {
        let file = self.file.try_clone()?;
        Ok(Self { id: self.id, file })
    }

    pub fn load_exist(dir: &Path) -> Result<Vec<LogReader>> {
        let mut vec = Vec::<LogReader>::new();

        let rd = fs::read_dir(dir)?;
        for r in rd {
            let entry = r?;
            let path = entry.path();
            if path.extension().is_none() || path.extension().unwrap() != LOG_FILE_EXTENSION {
                continue;
            }
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let id = file_name.parse::<u64>()?;

            let reader = LogReader::from(&path, id)?;
            vec.push(reader);
        }

        Ok(vec)
    }

    pub fn try_iter(&self) -> Result<FrameReader> {
        let file = self.file.try_clone()?;
        Ok(FrameReader::new(file))
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
        let pos = self.pos;
        let size = frame.write(&mut self.w)?;
        self.pos += size;
        Ok(pos)
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

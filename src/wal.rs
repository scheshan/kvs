use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use crate::frame::Frame;
use crate::Result;

pub(crate) struct WAL {

}

impl WAL {
    pub fn new(path: &Path) -> Result<Self> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }

        let log_files = Self::init_log_files(path)?;
        unimplemented!()
    }

    fn init_log_files(dir: &Path) -> Result<Vec<LogFile>> {
        unimplemented!()
    }

    pub fn insert(&mut self, frame: Frame) -> Result<LogPosition> {
        unimplemented!()
    }

    pub fn get(&mut self, pos: &LogPosition) -> Result<Frame> {
        unimplemented!()
    }
}

struct LogFile {
    id: u64,
    path: PathBuf,
    file: File,
}

impl LogFile {
    fn open(path: PathBuf) -> Result<LogFile> {
        unimplemented!()
    }
}

pub(crate) struct LogPosition {
    id: u64,
    pos: usize
}
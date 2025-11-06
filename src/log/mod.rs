mod reader;
mod writer;

pub use reader::LogReader;
pub use writer::LogWriter;

const LOG_FILE_EXTENSION: &str = "bin";

pub struct LogPosition {
    id: u64,
    pos: usize,
}

impl LogPosition {
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

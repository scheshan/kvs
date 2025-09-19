pub enum Frame {
    Set(String, String),
    Get(String),
    Remove(String),
}
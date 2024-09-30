use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct IndexError {
    pub reason: String,
}

impl Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for IndexError {}

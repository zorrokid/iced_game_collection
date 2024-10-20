use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::DialogClosed => write!(f, "Dialog closed"),
            Error::IoError(message) => write!(f, "IO error: {}", message),
        }
    }
}

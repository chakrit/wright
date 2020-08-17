use std::convert::From;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    UsageError(&'static str),
    IoError(std::io::Error),
    PathError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

use std::convert::From;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    NotImplementedError, // <- for use in temporary code
    PathError(String),
    UsageError(&'static str),
    ZipError(zip::result::ZipError),
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

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Error::ZipError(err)
    }
}

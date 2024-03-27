use std::fmt;

#[derive(Debug)]
pub enum MalisError {
    StdIoError(std::io::Error),
    ScannerError(ScannerError),
}

impl From<std::io::Error> for MalisError {
    fn from(err: std::io::Error) -> Self {
        Self::StdIoError(err)
    }
}

impl From<ScannerError> for MalisError {
    fn from(err: ScannerError) -> Self {
        Self::ScannerError(err)
    }
}

#[derive(Debug)]
pub enum ScannerError {}

pub struct SourceError<P: fmt::Debug> {
    line: usize,
    location: P,
    err: P,
}

impl<P: fmt::Debug> fmt::Debug for SourceError<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "[line {0}] Error {1:?}: {2:?}", self.line, self.location, self.err)
    }
}

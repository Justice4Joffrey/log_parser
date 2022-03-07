use crate::AsyncBufReaderSummarizerError;
use std::io;


/// The main entry point error which can wrap any other error from within
/// the library.
///
/// Although this is currently just a copy of the
/// [crate::AsyncBufReaderSummarizerError], it is actually a superset of
/// all errors that could arise at runtime. That just _happens_ to be true
/// for that type
#[derive(Debug)]
pub enum LogParserError {
    Io(io::Error),
    AsyncBufReaderSummarizer(AsyncBufReaderSummarizerError),
}

impl From<io::Error> for LogParserError {
    fn from(err: io::Error) -> Self {
        LogParserError::Io(err)
    }
}

impl From<AsyncBufReaderSummarizerError> for LogParserError {
    fn from(err: AsyncBufReaderSummarizerError) -> Self {
        Self::AsyncBufReaderSummarizer(err)
    }
}

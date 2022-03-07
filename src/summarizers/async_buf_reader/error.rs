use std::io;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum AsyncBufReaderSummarizerError {
    Io(io::Error),
    TokioError(JoinError),
}

impl From<JoinError> for AsyncBufReaderSummarizerError {
    fn from(err: JoinError) -> Self {
        AsyncBufReaderSummarizerError::TokioError(err)
    }
}

impl From<io::Error> for AsyncBufReaderSummarizerError {
    fn from(err: io::Error) -> Self {
        AsyncBufReaderSummarizerError::Io(err)
    }
}

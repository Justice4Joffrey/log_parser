use std::io;
use tokio::sync::mpsc::error::SendError;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum AsyncBufReaderSummarizerError {
    Io(io::Error),
    TokioJoinError(JoinError),
    /// Don't hang on to the inner details. There was an error sending
    /// messages between the mpsc and the reducer
    TokioSendError,
}

impl From<JoinError> for AsyncBufReaderSummarizerError {
    fn from(err: JoinError) -> Self {
        AsyncBufReaderSummarizerError::TokioJoinError(err)
    }
}

impl From<io::Error> for AsyncBufReaderSummarizerError {
    fn from(err: io::Error) -> Self {
        AsyncBufReaderSummarizerError::Io(err)
    }
}
impl<T> From<SendError<T>> for AsyncBufReaderSummarizerError {
    fn from(_: SendError<T>) -> Self {
        AsyncBufReaderSummarizerError::TokioSendError
    }
}

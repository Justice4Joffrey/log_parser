mod async_buf_reader;
mod sync_buf_reader;

use crate::Parser;
pub use async_buf_reader::{
    AsyncBufReaderSummarizer, AsyncBufReaderSummarizerError, DEFAULT_BATCH_SIZE,
    DEFAULT_REDUCER_CHANNEL_SIZE,
};

use crate::summary::{ErrorSummary, Summary};

pub use sync_buf_reader::{BufReaderSummarizer, DEFAULT_BUFFER_CAPACITY};

pub const DEFAULT_DELIMITER: u8 = b'\n';

/// The worker function for the process. Takes a Parser and aggregates
/// a [Summary] over the entire file
pub trait Summarizer {
    /// The type of error that can occur during parsing (i.e. invalid line)
    type ParserError: ErrorSummary;
    /// The type of error that can occur during processing (i.e. file not
    /// found)
    type SummarizerError;

    /// Take a file, read and parse it, and return a [Summary]
    fn summarize<P: Parser>(
        &self,
        logfile: &str,
    ) -> Result<Summary<Self::ParserError>, Self::SummarizerError>;
}

pub use parsers::CharParser;
pub use parsers::JsonParser;
pub use parsers::LogLineMetadata;
pub use parsers::Parser;
pub use parsers::RegexParser;
pub use parsers::StringParser;
pub use summarizers::{
    AsyncBufReaderSummarizer, AsyncBufReaderSummarizerError, BufReaderSummarizer,
    DEFAULT_BATCH_SIZE, DEFAULT_BUFFER_CAPACITY, DEFAULT_DELIMITER, DEFAULT_REDUCER_CHANNEL_SIZE,
};
pub use summary::{ErrorSummary, JsonSummary, Summary};

pub use crate::summarizers::Summarizer;

mod error;
/// Convert a byte array to parsed metadata.
mod parsers;
/// Reading from files and delegating to parsers.
mod summarizers;
/// The aggregated return type
mod summary;

pub use error::LogParserError;

pub type MainParser = StringParser;

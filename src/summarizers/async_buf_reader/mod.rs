use crate::summarizers::async_buf_reader::batch_reader::{AsyncBatchRead, AsyncBatchReader};
use crate::summarizers::async_buf_reader::parser_error::AsyncBatchReaderParserError;
use crate::{Parser, Summarizer, Summary, DEFAULT_DELIMITER};
use byte_lines::ByteLinesExt;
pub use error::AsyncBufReaderSummarizerError;
use tokio::fs::File;
use tokio::io;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

mod batch_reader;
/// A utility for iterating byte arrays over delimiters.
mod byte_lines;
mod error;
mod parser_error;

pub const DEFAULT_BATCH_SIZE: usize = 1_048_576;
pub const DEFAULT_REDUCER_CHANNEL_SIZE: usize = 1024;

pub struct AsyncBufReaderSummarizer {
    /// The maximum number of messages in the reducer channel before it
    /// blocks.
    reducer_channel_size: usize,
    /// The maximum amount of bytes to read in a single call.
    batch_size: usize,
    /// The character to split on
    delim: u8,
}

impl Default for AsyncBufReaderSummarizer {
    fn default() -> Self {
        Self {
            reducer_channel_size: DEFAULT_REDUCER_CHANNEL_SIZE,
            batch_size: DEFAULT_BATCH_SIZE,
            delim: DEFAULT_DELIMITER,
        }
    }
}

impl AsyncBufReaderSummarizer {
    pub fn new(reducer_channel_size: usize, batch_size: usize, delim: u8) -> Self {
        Self {
            reducer_channel_size,
            batch_size,
            delim,
        }
    }
}

impl Summarizer for AsyncBufReaderSummarizer {
    type ParserError = AsyncBatchReaderParserError;
    type SummarizerError = AsyncBufReaderSummarizerError;

    /// Rather complex map reduce algorithm to read a file in batches
    /// and delegate each batch to a new tokio task (mapper) before
    /// sending to a single reducer task.
    fn summarize<P: Parser>(
        &self,
        logfile: &str,
    ) -> Result<Summary<Self::ParserError>, Self::SummarizerError> {
        #[cfg(feature = "console")]
        console_subscriber::init();

        // Copy out these values to avoid lifetime shenanigans
        let logfile = logfile.to_owned();
        let delim = self.delim;
        let batch_size = self.batch_size;
        let reducer_channel_size = self.reducer_channel_size;
        // start a tokio runtime here just so it's not a hard requirement
        // for the application
        let rt = Runtime::new()?;

        let res: Result<Summary<Self::ParserError>, AsyncBufReaderSummarizerError> =
            rt.block_on(async {
                // channels from mappers to reducers
                let (tx, mut rx) =
                    tokio::sync::mpsc::channel::<Summary<Self::ParserError>>(reducer_channel_size);

                // the reader task which spawns the mappers
                let reader_handle: JoinHandle<io::Result<()>> = tokio::spawn(async move {
                    let file = File::open(logfile).await?;
                    let meta = file.metadata().await?;
                    let mut reader = AsyncBatchReader::new(file, batch_size, delim);
                    // Preallocate this based on the file size vs batch size. Add one to account for truncating
                    let mut handles =
                        Vec::with_capacity(((meta.len() / batch_size as u64) + 1) as usize);
                    while let Some(data) = reader.read_batch().await? {
                        let tx = tx.clone();
                        // spawn a mapper task per batch
                        handles.push(tokio::spawn(async move {
                            let mut summary = Summary::new();
                            for line in data.byte_lines(delim) {
                                if let Ok(meta) = P::parse(line) {
                                    summary.accumulate(&meta);
                                } else {
                                    summary.register_error(1);
                                }
                            }
                            tx.send(summary)
                                .await
                                .expect("Failed to send summary data between threads");
                        }));
                    }
                    // await all tasks (don't worry, they're not lazy)
                    // propagate any errors up the chain
                    for handle in handles {
                        handle.await?;
                    }
                    Ok(())
                });

                // the reducer task which listens to rx until there are no
                // more senders
                let reducer_handle = tokio::spawn(async move {
                    let mut summary = Summary::default();
                    while let Some(metadata) = rx.recv().await {
                        summary.combine(metadata)
                    }
                    summary
                });

                // join it all together!
                let (summary_result, reader_result) = tokio::join![reducer_handle, reader_handle];

                // guh this nested error handling is ugly
                reader_result??;
                let summary = summary_result?;
                Ok(summary)
            });
        res
    }
}

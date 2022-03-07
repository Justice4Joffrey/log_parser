mod parser_error;

use crate::summarizers::sync_buf_reader::parser_error::BufReaderParserError;
use crate::{Parser, Summarizer, Summary, DEFAULT_DELIMITER};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub const DEFAULT_BUFFER_CAPACITY: usize = 1024;

pub struct BufReaderSummarizer {
    /// The length of the read buffer
    capacity: usize,
    /// The character to split on
    delim: u8,
}

impl Default for BufReaderSummarizer {
    fn default() -> Self {
        Self {
            capacity: DEFAULT_BUFFER_CAPACITY,
            delim: DEFAULT_DELIMITER,
        }
    }
}

impl BufReaderSummarizer {
    pub fn new(capacity: usize, delim: u8) -> Self {
        Self { capacity, delim }
    }
}

impl Summarizer for BufReaderSummarizer {
    type ParserError = BufReaderParserError;
    type SummarizerError = std::io::Error;

    fn summarize<P: Parser>(
        &self,
        logfile: &str,
    ) -> Result<Summary<Self::ParserError>, Self::SummarizerError> {
        let file = File::open(logfile)?;
        let mut reader = BufReader::new(file);
        let mut summary = Summary::new();
        let mut buf = Vec::with_capacity(self.capacity);
        let mut line_number: usize = 0;
        loop {
            // TODO: for incredibly long lines this will truncate and
            //  behave incorrectly
            //  we could grow the buffer to account for this
            let bytes = reader.read_until(self.delim, &mut buf)?;
            if bytes == 0 {
                break;
            }
            match P::parse(buf.as_slice()) {
                Ok(metadata) => {
                    summary.accumulate(&metadata);
                }
                Err(_) => summary.register_error(line_number),
            }
            line_number += 1;
            buf.clear();
        }
        Ok(summary)
    }
}

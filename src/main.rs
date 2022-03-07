use clap::{Parser, Subcommand};
use log_parser::{
    AsyncBufReaderSummarizer, BufReaderSummarizer, LogParserError, MainParser, Summarizer,
    DEFAULT_BATCH_SIZE, DEFAULT_BUFFER_CAPACITY, DEFAULT_DELIMITER, DEFAULT_REDUCER_CHANNEL_SIZE,
};


/// Simple CLI tool to extract JSON from a log file. By default, a human
/// readable summary is printed to stdout. Both a synchronous and
/// asynchronous version are available.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Subcommand
    #[clap(subcommand)]
    command: Commands,
    /// Print JSON to stdout.
    #[clap(long)]
    json: bool,
    /// The delimiter to use
    #[clap(long, short, default_value_t = DEFAULT_DELIMITER)]
    delimiter: u8,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Sync {
        file: String,
        #[clap(short, long, help = "Length of the line buffer", default_value_t = DEFAULT_BUFFER_CAPACITY)]
        buffer_capacity: usize,
    },
    Async {
        file: String,
        #[clap(short, long, help = "The size in bytes of each read batch", default_value_t = DEFAULT_BATCH_SIZE)]
        batch_size: usize,
        #[clap(short, long, help = "Maximum messages stored in the reducer queue.", default_value_t = DEFAULT_REDUCER_CHANNEL_SIZE)]
        reducer_channel_size: usize,
    },
}

fn main() -> Result<(), LogParserError> {
    let cmd = Cli::parse();
    let delim = cmd.delimiter;
    match cmd.command {
        Commands::Sync {
            file,
            buffer_capacity,
        } => {
            let summary = BufReaderSummarizer::new(buffer_capacity, delim)
                .summarize::<MainParser>(file.as_str())?;
            summary.print(cmd.json);
        }
        Commands::Async {
            file,
            batch_size,
            reducer_channel_size,
        } => {
            let summary = AsyncBufReaderSummarizer::new(reducer_channel_size, batch_size, delim)
                .summarize::<MainParser>(file.as_str())?;
            summary.print(cmd.json);
        }
    }
    Ok(())
}

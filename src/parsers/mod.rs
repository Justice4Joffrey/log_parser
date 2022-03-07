use std::fmt::Debug;

/// Available in benches
pub use char_parser::CharParser;
/// Available in benches
pub use json_parser::JsonParser;
/// Available in benches
pub use re_parser::RegexParser;
/// The winner of best parser
pub use string_parser::StringParser;

mod char_parser;
mod json_parser;
mod re_parser;
mod string_parser;

/// The successful result type of a line parse. References the string in the
/// type name to avoid copying.
#[derive(Debug, PartialEq)]
pub struct LogLineMetadata<'a> {
    pub type_name: &'a [u8],
    pub bytes: usize,
}

/// Take a line slice, return some metadata or a predefined error
pub trait Parser {
    /// The error of a parse operation
    type Error: Debug;
    fn parse(line: &[u8]) -> Result<LogLineMetadata, Self::Error>;
}

use crate::{LogLineMetadata, Parser};
use lazy_static::lazy_static;
use regex::bytes::Regex;

lazy_static! {
    static ref TYPE_RE: Regex = Regex::new(r#""type":\s*"([^"]+)"#).unwrap();
}

#[derive(Debug)]
pub struct NoTypeParsed;

pub struct RegexParser;

impl Parser for RegexParser {
    type Error = NoTypeParsed;

    fn parse(line: &[u8]) -> Result<LogLineMetadata, Self::Error> {
        match TYPE_RE.captures(line) {
            Some(caps) => Ok(LogLineMetadata {
                // Unwrap is safe because the regex guarantees that the
                // capture exists
                type_name: caps.get(1).unwrap().as_bytes(),
                bytes: line.len(),
            }),
            None => Err(NoTypeParsed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_parse() {
        let line = b"{\"type\":\"some_complicated_TYPE123\",\"message\":\"Hello, world!\"}";
        let result = RegexParser::parse(line).unwrap();
        assert_eq!(result.type_name, b"some_complicated_TYPE123");
        assert_eq!(result.bytes, line.len());
    }

    #[test]
    fn test_regex_parse_no_type() {
        let line = b"{\"message\":\"Hello, world!\"}";
        let result = RegexParser::parse(line);
        assert!(result.is_err());
    }
}

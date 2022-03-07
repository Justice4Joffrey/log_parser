use crate::{LogLineMetadata, Parser};

use serde::Deserialize;

/// A convenience for using [serde_json]
#[derive(Debug, PartialEq, Deserialize)]
pub struct LogLine<'a> {
    #[serde(borrow, rename = "type")]
    pub typ: &'a str,
}

pub struct JsonParser;

impl Parser for JsonParser {
    type Error = serde_json::Error;

    fn parse(line: &[u8]) -> Result<LogLineMetadata, Self::Error> {
        let log_line: LogLine = serde_json::from_slice(line)?;
        Ok(LogLineMetadata {
            type_name: log_line.typ.as_bytes(),
            bytes: line.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() -> serde_json::Result<()> {
        let line = b"{\"type\":\"test\",\"value\":\"test\"}";
        let metadata = JsonParser::parse(line)?;
        assert_eq!(metadata.type_name, b"test");
        assert_eq!(metadata.bytes, line.len());
        Ok(())
    }

    #[test]
    fn test_parse_invalid_line() {
        let line = b"{\"typ\":\"test\",\"value\":\"test\"}";
        assert!(JsonParser::parse(line).is_err());
        let line = b"{\"type\":1,\"value\":\"test\"}";
        assert!(JsonParser::parse(line).is_err());
        let line = b"{\"type\":\"test\",\"value\":\"test\"";
        assert!(JsonParser::parse(line).is_err());
    }
}

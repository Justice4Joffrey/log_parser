use crate::{LogLineMetadata, Parser};

const TYPE_STR: &[u8] = b"\"type\":";
const QUOTE_STR: &[u8] = b"\"";

#[derive(Debug)]
pub struct NoTypeParsed;

pub struct StringParser;

impl Parser for StringParser {
    type Error = NoTypeParsed;

    fn parse(line: &[u8]) -> Result<LogLineMetadata, Self::Error> {
        find_subsequence(line, TYPE_STR)
            .and_then(|index| {
                find_subsequence(&line[index + TYPE_STR.len()..], QUOTE_STR)
                    .map(|i| i + index + TYPE_STR.len())
            })
            .and_then(|start_index| {
                find_subsequence(&line[start_index + QUOTE_STR.len()..], QUOTE_STR)
                    .map(|rel_index| (start_index + QUOTE_STR.len(), start_index + rel_index))
            })
            .map(|(start_index, end_index)| LogLineMetadata {
                type_name: &line[start_index..=end_index],
                bytes: line.len(),
            })
            .ok_or(NoTypeParsed)
    }
}

pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_parse() {
        let line = b"{\"type\":\"some_complicated_TYPE123\",\"message\":\"Hello, world!\"}";
        let result = StringParser::parse(line).unwrap();
        println!("IM {}", String::from_utf8_lossy(result.type_name));
        assert_eq!(result.type_name, b"some_complicated_TYPE123");
        assert_eq!(result.bytes, line.len());
    }

    #[test]
    fn test_string_parse_no_type() {
        let line = b"{\"message\":\"Hello, world!\"}";
        let result = StringParser::parse(line);
        assert!(result.is_err());
    }
}

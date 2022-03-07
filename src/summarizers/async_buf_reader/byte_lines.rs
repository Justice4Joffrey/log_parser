/// Doesn't seem like there's great support for doing this in Rust. I
/// specifically want to split a byte slice on a delimiter, including that
/// delimiter.
pub struct ByteLines<'a> {
    delim: u8,
    buf: &'a [u8],
    read_start: usize,
    read_end: usize,
}

impl<'a> ByteLines<'a> {
    pub fn new(delim: u8, buf: &'a [u8]) -> ByteLines<'a> {
        ByteLines {
            delim,
            buf,
            read_start: 0,
            read_end: 0,
        }
    }
}

impl<'a> Iterator for ByteLines<'a> {
    type Item = &'a [u8];

    /// Iterate through the buffer until we find a delimiter or the end.
    /// Return a slice to the buffer _including_ the delimiter.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.read_end == self.buf.len() {
                if self.read_start < self.read_end {
                    let slice = &self.buf[self.read_start..self.read_end];
                    self.read_start = self.read_end;
                    return Some(slice);
                } else {
                    return None;
                }
            } else {
                let byte = self.buf[self.read_end];
                if byte == self.delim {
                    self.read_end += 1;
                    let slice = &self.buf[self.read_start..self.read_end];
                    self.read_start = self.read_end;
                    return Some(slice);
                }
                self.read_end += 1;
            }
        }
    }
}

/// Extension trait to make `ByteLines` available on [&[u8]].
pub trait ByteLinesExt<'a> {
    fn byte_lines(&'a self, delim: u8) -> ByteLines<'a>;
}

impl<'a> ByteLinesExt<'a> for [u8] {
    fn byte_lines(&'a self, delim: u8) -> ByteLines<'a> {
        ByteLines::new(delim, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_lines() {
        let mut lines = ByteLines::new(b'\n', b"hello\nworld\n");
        assert_eq!(lines.next(), Some(&b"hello\n"[..]));
        assert_eq!(lines.next(), Some(&b"world\n"[..]));
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn test_byte_lines_empty_buffer() {
        let mut lines = ByteLines::new(b'\n', b"");
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn test_byte_lines_no_end_newline() {
        let mut lines = ByteLines::new(b'\n', b"hello");
        assert_eq!(lines.next(), Some(&b"hello"[..]));
        assert_eq!(lines.next(), None);
    }
}

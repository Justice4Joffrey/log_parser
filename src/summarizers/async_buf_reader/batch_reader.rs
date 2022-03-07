use async_trait::async_trait;
use tokio::io;
use tokio::io::{AsyncRead, AsyncReadExt};

/// Custom reader which streams a fixed size from the reader and truncates
/// any trailing data past the delimiter, starting the next batch with it,
/// if present.
pub struct AsyncBatchReader<R: AsyncRead + Unpin + Send + Sync> {
    reader: R,
    remainder_buffer: Vec<u8>,
    buffer_size: usize,
    delimiter: u8,
}

impl<R: AsyncRead + Unpin + Send + Sync> AsyncBatchReader<R> {
    pub fn new(reader: R, buffer_size: usize, delimiter: u8) -> AsyncBatchReader<R> {
        AsyncBatchReader {
            reader,
            remainder_buffer: Vec::with_capacity(buffer_size),
            buffer_size,
            delimiter,
        }
    }
}
#[async_trait]
pub trait AsyncBatchRead {
    async fn read_batch(&mut self) -> io::Result<Option<Vec<u8>>>;
}

#[async_trait]
impl<R: AsyncRead + Unpin + Send + Sync> AsyncBatchRead for AsyncBatchReader<R> {
    async fn read_batch(&mut self) -> io::Result<Option<Vec<u8>>> {
        let mut buffer = Vec::with_capacity(self.buffer_size);
        std::mem::swap(&mut buffer, &mut self.remainder_buffer);
        (&mut self.reader)
            .take((self.buffer_size - buffer.len()) as u64)
            .read_to_end(&mut buffer)
            .await?;
        if buffer.is_empty() {
            return Ok(None);
        }
        // TODO: don't unwrap - grow buffer
        //  in practice just choose a decent buffer size (if your lines are
        //  over 1MB you don't deserve a summary :P)
        let delim = last_delim(buffer.as_slice(), self.delimiter).unwrap();
        self.remainder_buffer = buffer[delim + 1..].to_vec();
        buffer.truncate(delim + 1);
        Ok(Some(buffer))
    }
}

/// Return the last occurrence of a delimiter in a buffer
fn last_delim(buf: &[u8], delim: u8) -> Option<usize> {
    let mut i = buf.len() - 1;
    loop {
        if buf[i] == delim {
            return Some(i);
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_delim() {
        let buf = b"Here's some text\nHere's some more";
        assert_eq!(last_delim(buf, b'\n').unwrap(), 16);
        let buf = b"\nHere's some text";
        assert_eq!(last_delim(buf, b'\n').unwrap(), 0);
    }

    #[test]
    fn test_last_delim_non_existant() {
        let buf = b"Here's some text";
        assert!(last_delim(buf, b'\n').is_none());
        let buf = b"Here's some text\\nHere's some more";
        assert!(last_delim(buf, b'\n').is_none());
    }
}

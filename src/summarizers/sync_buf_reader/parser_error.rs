use crate::ErrorSummary;
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct BufReaderParserError {
    /// The lines we failed to parse
    error_lines: Vec<usize>,
}

impl ErrorSummary for BufReaderParserError {
    fn display_error(&self) {
        if !self.error_lines.is_empty() {
            if self.error_lines.len() == 1 {
                self.red_stderr_line(format!("Failed to parse line {}", self.error_lines[0]));
            } else {
                self.red_stderr_line("Failed to parse the following lines:");
                for line in self.error_lines.iter().sorted() {
                    self.red_stderr_line(format!("{}", line));
                }
            }
            self.red_stderr_line("");
        }
    }

    fn accumulate(&mut self, error: usize) {
        self.error_lines.push(error)
    }

    fn combine(&mut self, other: Self) {
        self.error_lines.extend(other.error_lines.iter());
    }

    fn total_errors(&self) -> usize {
        self.error_lines.len()
    }
}

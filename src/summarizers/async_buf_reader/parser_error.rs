use crate::ErrorSummary;

#[derive(Debug, Default)]
pub struct AsyncBatchReaderParserError {
    /// Just the total number of errors. We can't reasonably figure out the
    /// line number when batch processing.
    errors: usize,
}

impl ErrorSummary for AsyncBatchReaderParserError {
    fn display_error(&self) {
        if self.errors > 0 {
            self.red_stderr_line(format!(
                "{} line{} could not be parsed\n",
                self.errors,
                if self.errors == 1 { "" } else { "s" }
            ));
        }
    }

    fn accumulate(&mut self, error: usize) {
        self.errors += error;
    }

    fn combine(&mut self, other: Self) {
        self.errors += other.errors;
    }

    fn total_errors(&self) -> usize {
        self.errors
    }
}

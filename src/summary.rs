use crate::LogLineMetadata;
use bytesize::ByteSize;
use colored::Colorize;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::ops::Deref;
use tabwriter::TabWriter;

/// Don't use references for the keys as this would enforce any file buffer
/// to live for 'static ==> no buffered loading
type TypeCountMap = HashMap<Vec<u8>, usize>;

/// A public facing JSON type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSummary {
    /// The total file size
    pub total_size: usize,
    /// All types and aggregate sizes in bytes
    pub type_size: HashMap<String, usize>,
    /// The total number of lines which returned errors
    pub total_errors: usize,
}

/// An aggregated type string count
#[derive(Debug)]
pub struct Summary<E: ErrorSummary> {
    type_counts: TypeCountMap,
    errors: E,
}

impl<E: ErrorSummary> Default for Summary<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: ErrorSummary> Display for Summary<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn tw_write<L: std::io::Write, S: Deref<Target = str>>(
            tw: &mut TabWriter<L>,
            s: S,
        ) -> Result<usize, std::fmt::Error> {
            tw.write(s.as_bytes()).map_err(|_| std::fmt::Error)
        }
        let mut tw = TabWriter::new(vec![]);
        // sorted by size desc
        for (k, v) in self.type_counts.iter().sorted_by(|(_, a), (_, b)| b.cmp(a)) {
            tw_write(
                &mut tw,
                format!("{}\t{}\n", String::from_utf8_lossy(k), ByteSize(*v as u64)),
            )?;
        }
        tw_write(
            &mut tw,
            format!("Total:\t{}", ByteSize(self.total_size() as u64)),
        )?;
        write!(
            f,
            "{}",
            String::from_utf8_lossy(&tw.into_inner().map_err(|_| std::fmt::Error)?)
        )
    }
}

impl<E: ErrorSummary> Summary<E> {
    pub fn new() -> Self {
        Summary {
            type_counts: HashMap::new(),
            errors: E::default(),
        }
    }

    pub fn register_error(&mut self, error: usize) {
        self.errors.accumulate(error);
    }

    /// The total file size
    fn total_size(&self) -> usize {
        self.type_counts.values().sum()
    }

    pub fn print(&self, json: bool) {
        let payload = if json {
            self.to_json()
        } else {
            // here we output any errors to stderr
            // can be more descriptive than the json e.g. line numbers
            self.errors.display_error();
            self.to_string()
        };
        println!("{}", payload);
    }

    /// Convert this to a json summary
    pub fn to_json(&self) -> String {
        let mut type_size = HashMap::new();
        for (k, v) in self.type_counts.iter() {
            type_size.insert(String::from_utf8_lossy(k).to_string(), *v);
        }
        serde_json::to_string_pretty(&JsonSummary {
            total_size: self.total_size(),
            type_size,
            total_errors: self.errors.total_errors(),
        })
        .expect("Failed to serialize json summary")
    }

    pub fn accumulate(&mut self, metadata: &LogLineMetadata<'_>) {
        // don't use entry API as that would require cloning for _every_
        // lookup
        match self.type_counts.get_mut(metadata.type_name) {
            Some(count) => {
                *count += metadata.bytes;
            }
            None => {
                self.type_counts
                    .insert(metadata.type_name.to_owned(), metadata.bytes);
            }
        }
    }

    /// Not quite [std::ops::Add]. This is an in-place merge with another [Summary]
    pub fn combine(&mut self, other: Self) {
        for (key, value) in other.type_counts {
            let entry = self.type_counts.entry(key).or_insert(0);
            *entry += value;
        }
        self.errors.combine(other.errors);
    }
}

/// This is just a convenience so we can abstract over handling different
/// error types
pub trait ErrorSummary: Debug + Default {
    fn red_stderr_line<S: AsRef<str>>(&self, line: S) {
        eprintln!("{}", line.as_ref().to_string().red());
    }
    fn display_error(&self);
    /// Some indication of the error. E.g. 1 for 'an error' or `line_number`.
    fn accumulate(&mut self, error: usize);
    /// Combine another [ErrorSummary]
    fn combine(&mut self, other: Self);
    /// Total number of errors
    fn total_errors(&self) -> usize;
}

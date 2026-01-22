use std::fmt::Display;

use log::trace;

use crate::error::Location;

// A span represents a span of text
// The start is inclusive and the end is exclusive [start, end)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize, 
    pub end: usize,
}

pub struct SourceFile {
    file_path: String,
    pub contents: String, 
    line_starts: Vec<usize>,
}

impl SourceFile {
    pub fn new(file_path: String, contents: String) -> Self {
        let mut line_starts = vec![0];
        for (i, byte) in contents.bytes().enumerate() {
            if byte == b'\n' {
                trace!("Found newline in {} at {}", file_path, i);
                line_starts.push(i + 1);
            }
        }
        SourceFile { file_path, contents, line_starts }

    }

    // Get a location (row, col) based on the byte position 
    pub fn lookup(&self, pos: usize) -> Location {

        let row = match self.line_starts.binary_search(&pos) {
            Ok(idx) => idx,
            Err(idx) => idx - 1,
        };

        let column = pos - self.line_starts[row];
        Location { row, column }
    }
}

impl Span {
    pub fn merge(&self, other: &Span) -> Self {
        Self {
            start: self.start, 
            end: other.end
        }
    }

}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

use std::fs::File;
use std::io::Write;
use std::thread::JoinHandle;

#[derive(Debug, Default)]
pub(super) struct ProgramBuilder {
    /// none '0', -q '2', -b '3' or -h '1'
    pub(super) method: u8,
    /// if -b then -t '1' or -l <file> '2' else none '0'
    pub(super) output: u8,
    /// in case of -l, output file
    /// if -lo is given then then the log file will be overwritten
    /// else if -lc is given the new data will be appended to the file
    /// if -l alone is given without o or c then -lc will be assumed
    pub(super) log_file: Option<File>,
    /// if -b then -d '1' or -r '2' else none '0'
    pub(super) data: u8,
    /// query or bind patterns, if any
    pub(super) patterns: Vec<Vec<String>>,
}

impl ProgramBuilder {
    /// creates a new instance of a ProgramBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// changes the method field with new value then returns self
    pub fn method(mut self, method: u8) -> Self {
        self.method = method;
        self
    }

    /// changes the output field with new value then returns self
    pub fn output(mut self, output: u8) -> Self {
        self.output = output;
        self
    }

    /// changes the data field with new value then returns self
    pub fn data(mut self, data: u8) -> Self {
        self.data = data;
        self
    }

    /// changes the log_file field with new value then returns self
    pub fn log_file(mut self, file: File) -> Self {
        self.log_file = Some(file);

        self
    }

    /// changes the method field with new value in place
    pub fn method_mut(&mut self, method: u8) {
        if self.method != method {
            self.method = method;
        }
    }

    /// changes the output field with new value then returns self
    pub fn output_mut(&mut self, output: u8) {
        if self.output != output {
            self.output = output;
        }
    }

    /// changes the data field with new value then returns self
    pub fn data_mut(&mut self, data: u8) {
        if self.data != data {
            self.data = data;
        }
    }

    pub fn log_file_mut(&mut self, file: File) {
        self.log_file = Some(file);
    }

    pub fn push(&mut self, pats: Vec<String>) {
        if !pats.is_empty() {
            self.patterns.push(pats);
        }
    }
}

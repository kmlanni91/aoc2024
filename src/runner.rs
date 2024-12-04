use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InputParseError {
    pub message: &'static str,
}

impl Display for InputParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse input")
    }
}

impl Error for InputParseError {}

pub trait Run {
    fn run(&self, reader: impl BufRead) -> Result<u64, InputParseError>;

    fn run2(&self, reader: impl BufRead) -> Result<u64, InputParseError>;
}

pub trait RunFile {
    fn run(&self, reader: BufReader<File>) -> Result<u64, InputParseError>;

    fn run2(&self, reader: BufReader<File>) -> Result<u64, InputParseError>;
}

impl<T> RunFile for T
where
    T: Run,
{
    fn run(&self, reader: BufReader<File>) -> Result<u64, InputParseError> {
        Run::run(self, reader)
    }

    fn run2(&self, reader: BufReader<File>) -> Result<u64, InputParseError> {
        Run::run2(self, reader)
    }
}

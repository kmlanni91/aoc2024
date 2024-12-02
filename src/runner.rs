use std::{error::Error, fmt::Display, io::BufRead};

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
    fn run(&self, reader: impl BufRead) -> Result<impl Display, InputParseError>;
}

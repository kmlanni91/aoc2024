use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub struct InputParseError {
    pub message: &'static str,
}

pub trait Run {
    fn run(&self, fp: &str) -> Result<impl Display, InputParseError>;
}

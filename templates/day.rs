use crate::runner::{InputParseError, Run};

#[derive(Debug)]
use std::io::BufRead;
pub struct Runner;

impl Run for Runner {
    #[allow(refining_impl_trait)]
    fn run(&self, reader: impl BufRead) -> Result<u64, InputParseError> {
        Ok(0)
    }

    #[allow(refining_impl_trait)]
    fn run2(&self, reader: impl BufRead) -> Result<u64, InputParseError> {
        Ok(0)
    }
}

fn parse(reader: impl BufRead) -> Result<(), InputParseError> {
    Ok(())
}

fn parse_line(line: String) -> Result<(), InputParseError> {
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn part1() {
        let input = String::from("");

        let expected = 1;
        let result = Runner
            .run(BufReader::new(&mut input.as_bytes()))
            .expect("Unexpected parse error");
        assert_eq!(result, expected)
    }

    #[test]
    fn part2() {
        let input = String::from("");

        let expected = 1;
        let result = Runner
            .run2(BufReader::new(&mut input.as_bytes()))
            .expect("Unexpected parse error");
        assert_eq!(result, expected)
    }
}

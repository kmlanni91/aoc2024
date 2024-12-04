use crate::runner::{InputParseError, Run};
use std::io::BufRead;

#[derive(Debug)]
pub struct Runner;

impl Run for Runner {
    #[allow(refining_impl_trait)]
    fn run(&self, reader: impl BufRead) -> Result<u64, InputParseError> {
        let reports = parse(reader)?;
        println!("{:?}", reports);
        Ok(0)
    }

    #[allow(refining_impl_trait)]
    fn run2(&self, _reader: impl BufRead) -> Result<u64, InputParseError> {
        Ok(0)
    }
}

fn parse(reader: impl BufRead) -> Result<Vec<Vec<u32>>, InputParseError> {
    reader
        .lines()
        .map(|x| {
            x.map_err(|_x| InputParseError {
                message: "Failed to parse line",
            })
        })
        .map(|x| x.and_then(parse_line))
        .collect()
}

fn parse_line(line: String) -> Result<Vec<u32>, InputParseError> {
    line.split_whitespace()
        .map(|x| {
            x.parse().map_err(|_x| InputParseError {
                message: "Failed to parse char in int",
            })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn part1() {
        let input = String::from(
            "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        ",
        );

        let expected = 2;
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

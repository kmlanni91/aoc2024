use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::runner::{InputParseError, Run};

#[derive(Debug)]
pub struct Runner;

impl Run for Runner {
    #[allow(refining_impl_trait)]
    fn run(&self, fp: &str) -> Result<u32, InputParseError> {
        let val = "Test day1";
        println!("{}", val);
        Ok(0)
    }
}

fn parse(fp: &str) -> Result<(Vec<u32>, Vec<u32>), InputParseError> {
    let f = File::open(fp).unwrap();
    let lists: Result<Vec<(u32, u32)>, InputParseError> = BufReader::new(f)
        .lines()
        .map(|x| {
            x.map_err(|_x| InputParseError {
                message: "Failed to parse line",
            })
        })
        .map(|x| x.and_then(parse_line))
        .collect();
    lists.map(move |x| x.into_iter().unzip())
}

fn parse_line(line: String) -> Result<(u32, u32), InputParseError> {
    let mut chars = line.split_whitespace();
    let left: u32 = chars
        .nth(0)
        .ok_or(InputParseError {
            message: "No char found at index 0",
        })?
        .parse()
        .or(Err(InputParseError {
            message: "Could not parse char at index 0",
        }))?;
    let right: u32 = chars
        .nth(0)
        .ok_or(InputParseError {
            message: "No char found at index 1",
        })?
        .parse()
        .or(Err(InputParseError {
            message: "Could not parse char at index 1",
        }))?;
    Ok((left.clone(), right.clone()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "3   4\n\
                     4   3\n\
                     2   5\n\
                     1   3\n\
                     3   9\n\
                     3   3";

        let expected = 11;
        let result = Runner.run(&input).expect("Unexpected parse error");
        assert_eq!(result, expected)
    }
}

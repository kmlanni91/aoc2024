use std::{collections::HashMap, fmt::Debug, io::BufRead, iter::zip, u64};

use crate::runner::{InputParseError, Run};

#[derive(Debug)]
pub struct Runner;

impl Run for Runner {
    #[allow(refining_impl_trait)]
    fn run(&self, reader: impl BufRead) -> Result<u64, InputParseError> {
        let (mut left, mut right) = parse(reader)?;
        left.sort();
        right.sort();
        Ok(calculate_distance(left, right))
    }

    #[allow(refining_impl_trait)]
    fn run2(&self, reader: impl BufRead) -> Result<u64, InputParseError> {
        let (mut left, mut right) = parse(reader)?;
        left.sort();
        right.sort();
        Ok(calculate_sim_score(left, right))
    }
}

fn parse(reader: impl BufRead) -> Result<(Vec<u32>, Vec<u32>), InputParseError> {
    let lists: Result<Vec<(u32, u32)>, InputParseError> = reader
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

fn calculate_distance(left: Vec<u32>, right: Vec<u32>) -> u64 {
    assert_eq!(left.len(), right.len());
    u64::from(zip(left, right).map(|(x, y)| x.abs_diff(y)).sum::<u32>())
}

fn add_to_counter(mut counter: HashMap<u32, u64>, num: u32) -> HashMap<u32, u64> {
    let mut count = 1;
    if let Some(val) = counter.get(&num) {
        count += val;
    }
    counter.insert(num, count);
    counter
}

fn calculate_sim_score(left: Vec<u32>, right: Vec<u32>) -> u64 {
    assert_eq!(left.len(), right.len());
    let counter: HashMap<u32, u64> = HashMap::new();
    let counts = right.iter().fold(counter, |acc, x| add_to_counter(acc, *x));
    left.iter()
        .map(|x| counts.get(x).unwrap_or(&0) * u64::from(*x))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn part1() {
        let input = String::from(
            "3   4\n\
             4   3\n\
             2   5\n\
             1   3\n\
             3   9\n\
             3   3",
        );

        let expected = 11;
        let result = Runner
            .run(BufReader::new(&mut input.as_bytes()))
            .expect("Unexpected parse error");
        assert_eq!(result, expected)
    }

    #[test]
    fn part2() {
        let input = String::from(
            "3   4\n\
             4   3\n\
             2   5\n\
             1   3\n\
             3   9\n\
             3   3",
        );

        let expected = 31;
        let result = Runner
            .run2(BufReader::new(&mut input.as_bytes()))
            .expect("Unexpected parse error");
        assert_eq!(result, expected)
    }
}

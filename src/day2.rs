use crate::runner::{InputParseError, Run};
use std::io::BufRead;

#[derive(Debug)]
pub struct Runner;

impl Run for Runner {
    #[allow(refining_impl_trait)]
    fn run(&self, reader: impl BufRead) -> Result<u64, InputParseError> {
        let reports = parse(reader)?;
        //println!("{:?}", reports);
        //println!("{}", reports.len());
        Ok(u64::try_from(
            reports
                .iter()
                .map(|report| analyze_report(report))
                .filter(|safety| *safety == Safety::Safe)
                .count(),
        )
        .expect("Count was bigger than u64"))
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

#[derive(Eq, PartialEq, Debug)]
enum Safety {
    Safe,
    UnSafe,
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    Ascending,
    Descending,
    Neutral,
}

#[derive(Debug, PartialEq, Eq)]
struct ReportState {
    safety: Safety,
    direction: Option<Direction>,
    prev: Option<u32>,
}

fn analyze_report(report: &Vec<u32>) -> Safety {
    let safety = report
        .iter()
        .fold(
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: None,
            },
            |state, next| compute_state(state, *next),
        )
        .safety;
    //println!("{:?}", safety);
    safety
}

fn compute_state(state: ReportState, next: u32) -> ReportState {
    let direction = match state.prev {
        Some(prev) => {
            let _dir = if prev < next {
                Direction::Ascending
            } else if prev > next {
                Direction::Descending
            } else {
                Direction::Neutral
            };
            Some(_dir)
        }
        None => None,
    };
    let is_in_range = match state.prev {
        Some(prev) => {
            let diff = prev.abs_diff(next);
            //println!("{}", diff);
            (diff >= 1) && (diff <= 3)
        }
        None => true,
    };
    //println!("{}", is_in_range);
    let safety = match state.safety {
        Safety::Safe => match (state.direction, &direction) {
            (Some(prev_dir), Some(curr_dir)) => {
                if (prev_dir == *curr_dir) && is_in_range {
                    Safety::Safe
                } else {
                    Safety::UnSafe
                }
            },
            (_, Some(Direction::Neutral)) => Safety::UnSafe,
            (None, _) => if is_in_range {Safety::Safe} else {Safety::UnSafe},
            _ => Safety::Safe,
        },
        Safety::UnSafe => Safety::UnSafe
    };
    ReportState {
        safety,
        direction,
        prev: Some(next),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_compute_state() {
        // Test initial state
        let result = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: None,
            },
            7,
        );
        assert_eq!(
            result,
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: Some(7)
            }
        );
        // Test second state safe desc
        let result2 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: Some(7),
            },
            6,
        );
        assert_eq!(
            result2,
            ReportState {
                safety: Safety::Safe,
                direction: Some(Direction::Descending),
                prev: Some(6)
            }
        );
        // Test mid state safe desc
        let result3 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: Some(Direction::Descending),
                prev: Some(7),
            },
            6,
        );
        assert_eq!(
            result3,
            ReportState {
                safety: Safety::Safe,
                direction: Some(Direction::Descending),
                prev: Some(6)
            }
        );
        // Test unsafe state neut
        let result4 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: Some(Direction::Descending),
                prev: Some(7),
            },
            7,
        );
        assert_eq!(
            result4,
            ReportState {
                safety: Safety::UnSafe,
                direction: Some(Direction::Neutral),
                prev: Some(7)
            }
        );
        // Test unsafe dir change
        let result5 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: Some(Direction::Descending),
                prev: Some(7),
            },
            8,
        );
        assert_eq!(
            result5,
            ReportState {
                safety: Safety::UnSafe,
                direction: Some(Direction::Ascending),
                prev: Some(8)
            }
        );
        // Test second state unsafe neut
        let result6 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: Some(7),
            },
            7,
        );
        assert_eq!(
            result6,
            ReportState {
                safety: Safety::UnSafe,
                direction: Some(Direction::Neutral),
                prev: Some(7)
            }
        );
        // Test second state unsafe out of range
        let result6 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: Some(7),
            },
            11,
        );
        assert_eq!(
            result6,
            ReportState {
                safety: Safety::UnSafe,
                direction: Some(Direction::Ascending),
                prev: Some(11)
            }
        );

    }

    #[test]
    fn test_analyze_report() {
        let input = vec![40, 42, 44, 47, 49, 50, 48];
        let result = analyze_report(&input);
        assert_eq!(result, Safety::UnSafe)
    }

    #[test]
    fn part1() {
        let input = String::from(
            "\
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9\
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

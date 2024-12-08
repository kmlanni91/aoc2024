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
                .map(|report| analyze_report(report, false))
                .filter(|safety| *safety == Safety::Safe)
                .count(),
        )
        .expect("Count was bigger than u64"))
    }

    #[allow(refining_impl_trait)]
    fn run2(&self, reader: impl BufRead) -> Result<u64, InputParseError> {
        let reports = parse(reader)?;
        //println!("{:?}", reports);
        //println!("{}", reports.len());
        Ok(u64::try_from(
            reports
                .iter()
                .map(|report| analyze_report(report, true))
                .filter(|safety| *safety == Safety::Safe)
                .count(),
        )
        .expect("Count was bigger than u64"))
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

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Safety {
    Safe,
    UnSafe,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    Ascending,
    Descending,
    Neutral,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Skipper<'a> {
    start: &'a Vec<u32>,
    index: Option<u32>,
    used: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ReportState<'a> {
    safety: Safety,
    direction: Option<Direction>,
    prev: Option<u32>,
    skipper: Skipper<'a>,
}

fn analyze_report(report: &Vec<u32>, dampen: bool) -> Safety {
    let safety = report
        .iter()
        .fold(
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: None,
                skipper: Skipper {
                    start: report[0..4].iter().cloned().collect::<Vec<u32>>().as_ref(),
                    index: None,
                    used: false,
                },
            },
            |state, next| compute_state(state, *next, dampen),
        )
        .safety;
    //println!("{:?}", safety);
    safety
}

fn compute_state(state: ReportState, next: u32, dampen: bool) -> ReportState {
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
            }
            (_, Some(Direction::Neutral)) => Safety::UnSafe,
            (None, _) => {
                if is_in_range {
                    Safety::Safe
                } else {
                    Safety::UnSafe
                }
            }
            _ => Safety::Safe,
        },
        Safety::UnSafe => Safety::UnSafe,
    };
    let index = state.skipper.index.map_or_else(|| 0, |idx| idx + 1);
    match (&safety, &state.skipper.used, dampen) {
        (Safety::UnSafe, false, true) => {
            if index == 2 {
                let dir_0_to_1 = get_direction(
                    *state.skipper.start.get(0).unwrap(),
                    *state.skipper.start.get(1).unwrap(),
                );
                let dir_0_to_3 = get_direction(
                    *state.skipper.start.get(0).unwrap(),
                    *state.skipper.start.get(3).unwrap(),
                );
                if dir_0_to_1 == dir_0_to_3 {
                    ReportState {
                        safety: Safety::Safe,
                        direction: state.direction,
                        prev: state.prev,
                        skipper: Skipper {
                            start: state.skipper.start,
                            index: state.skipper.index,
                            used: true
                        }
                    }
                } else {
                    ReportState {
                        safety: Safety::Safe,
                        direction,
                        prev: Some(next),
                        skipper: Skipper {
                            start: state.skipper.start,
                            index: Some(index),
                            used: true
                        }
                    }
                }
            } else {
                ReportState {
                    safety: Safety::Safe,
                    direction,
                    prev: Some(next),
                    skipper: Skipper {
                        start: state.skipper.start,
                        index: Some(index),
                        used: true
                    }
                }
            }
        }
        _ => ReportState {
            safety,
            direction,
            prev: Some(next),
            skipper: Skipper {
                start: state.skipper.start,
                index: Some(index),
                used: state.skipper.used,
            },
        },
    }
}

fn get_direction(first: u32, second: u32) -> Direction {
    if first < second {
        Direction::Ascending
    } else if first < second {
        Direction::Descending
    } else {
        Direction::Neutral
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_compute_state() {
        // Test initial state
        let start = vec![7, 6, 4, 2];
        let result = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: None,
                skipper: Skipper {
                    start: &start,
                    index: None,
                    used: false,
                },
            },
            7,
            false,
        );
        assert_eq!(
            result,
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: Some(7),
                skipper: Skipper {
                    start: &start,
                    index: Some(0),
                    used: false,
                }
            }
        );
        // Test second state safe desc
        let result2 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: Some(7),
                skipper: Skipper {
                    start: &start,
                    index: Some(0),
                    used: false,
                },
            },
            6,
            false,
        );
        assert_eq!(
            result2,
            ReportState {
                safety: Safety::Safe,
                direction: Some(Direction::Descending),
                prev: Some(6),
                skipper: Skipper {
                    start: &start,
                    index: Some(1),
                    used: false,
                },
            }
        );
        // Test mid state safe desc
        let result3 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: Some(Direction::Descending),
                prev: Some(7),
                skipper: Skipper {
                    start: &start,
                    index: Some(0),
                    used: false,
                },
            },
            6,
            false,
        );
        assert_eq!(
            result3,
            ReportState {
                safety: Safety::Safe,
                direction: Some(Direction::Descending),
                prev: Some(6),
                skipper: Skipper {
                    start: &start,
                    index: Some(1),
                    used: false,
                }
            }
        );
        // Test unsafe state neut
        let result4 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: Some(Direction::Descending),
                prev: Some(7),
                skipper: Skipper {
                    start: &start,
                    index: Some(0),
                    used: false,
                },
            },
            7,
            false,
        );
        assert_eq!(
            result4,
            ReportState {
                safety: Safety::UnSafe,
                direction: Some(Direction::Neutral),
                prev: Some(7),
                skipper: Skipper {
                    start: &start,
                    index: Some(1),
                    used: false,
                }
            }
        );
        // Test unsafe dir change
        let result5 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: Some(Direction::Descending),
                prev: Some(7),
                skipper: Skipper {
                    start: &start,
                    index: Some(0),
                    used: false,
                },
            },
            8,
            false,
        );
        assert_eq!(
            result5,
            ReportState {
                safety: Safety::UnSafe,
                direction: Some(Direction::Ascending),
                prev: Some(8),
                skipper: Skipper {
                    start: &start,
                    index: Some(1),
                    used: false,
                }
            }
        );
        // Test second state unsafe neut
        let result6 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: Some(7),
                skipper: Skipper {
                    start: &start,
                    index: Some(1),
                    used: false,
                },
            },
            7,
            false,
        );
        assert_eq!(
            result6,
            ReportState {
                safety: Safety::UnSafe,
                direction: Some(Direction::Neutral),
                prev: Some(7),
                skipper: Skipper {
                    start: &start,
                    index: Some(2),
                    used: false,
                }
            }
        );
        // Test second state unsafe out of range
        let result6 = compute_state(
            ReportState {
                safety: Safety::Safe,
                direction: None,
                prev: Some(7),
                skipper: Skipper {
                    start: &start,
                    index: Some(0),
                    used: false,
                },
            },
            11,
            false,
        );
        assert_eq!(
            result6,
            ReportState {
                safety: Safety::UnSafe,
                direction: Some(Direction::Ascending),
                prev: Some(11),
                skipper: Skipper {
                    start: &start,
                    index: Some(1),
                    used: false,
                }
            }
        );
    }

    #[test]
    fn test_analyze_report() {
        let input = vec![40, 42, 44, 47, 49, 50, 48];
        let result = analyze_report(&input, false);
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
    fn test_analyze_report_with_dampen() {
        // Test change in direction skip
        let input = vec![40, 42, 39, 44, 47, 49, 50];
        let result = analyze_report(&input, true);
        assert_eq!(result, Safety::Safe);

        let input2 = vec![40, 39, 42, 44, 47, 49, 50];
        let result2 = analyze_report(&input2, true);
        assert_eq!(result2, Safety::Safe);
    }

    #[test]
    fn part2() {
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

        let expected = 4;
        let result = Runner
            .run2(BufReader::new(&mut input.as_bytes()))
            .expect("Unexpected parse error");
        assert_eq!(result, expected)
    }
}

use crate::runner::{InputParseError, Run};
use std::io::{BufRead, Error};

#[derive(Debug)]
pub struct Runner;

impl Run for Runner {
    #[allow(refining_impl_trait)]
    fn run(&self, reader: impl BufRead) -> Result<u64, InputParseError> {
        let data = parse(reader)?;
        let result: u32 = data.iter().map(|(a, b)| a * b).sum();
        Ok(u64::from(result))
    }

    #[allow(refining_impl_trait)]
    fn run2(&self, _reader: impl BufRead) -> Result<u64, InputParseError> {
        Ok(0)
    }
}

#[derive(Debug)]
struct Eof;

struct Parser<'a> {
    input: &'a str,
}

//type Result<char>;

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser<'a> {
        Parser { input }
    }

    fn peek(&mut self) -> Result<char, Eof> {
        self.input.chars().next().ok_or(Eof)
    }

    fn next(&mut self) -> Result<char, Eof> {
        let ch = self.peek()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    fn validate_char(state: &Vec<char>, ch: char) -> bool {
        let len = state.len();
        match (len, ch) {
            (0, 'm') => true,
            (1, 'u') => true,
            (2, 'l') => true,
            (3, '(') => true,
            (4, ch) if ch.is_digit(10) => true,
            (5, ch) if ch.is_digit(10) | (ch == ',') => true,
            (6, ch) => {
                let prev = &state[4..6];
                match prev {
                    [_, ','] if ch.is_digit(10) => true,
                    _ if ch.is_digit(10) | (ch == ',') => true,
                    _ => false,
                }
            }
            (7, ch) => {
                let prev = &state[4..7];
                match prev {
                    [_, _, ','] if ch.is_digit(10) => true,
                    [_, ',', _] if ch.is_digit(10) | (ch == ')') => true,
                    _ if ch == ',' => true,
                    _ => false,
                }
            }
            (8, ch) => {
                let prev = [state[5], state[6], state[7]];
                match prev {
                    [_, _, ','] if ch.is_digit(10) => true,
                    [_, ',', _] if ch.is_digit(10) | (ch == ')') => true,
                    [',', _, _] if ch.is_digit(10) | (ch == ')') => true,
                    _ => false,
                }
            }
            (9, ch) => {
                let prev = [state[6], state[7], state[8]];
                match prev {
                    [_, ',', _] if ch.is_digit(10) | (ch == ')') => true,
                    [',', _, _] if ch.is_digit(10) | (ch == ')') => true,
                    _ if ch == ')' => true,
                    _ => false,
                }
            }
            (10, ch) => {
                let prev = [state[7], state[8], state[9]];
                match prev {
                    [',', _, _] if ch.is_digit(10) | (ch == ')') => true,
                    _ if ch == ')' => true,
                    _ => false,
                }
            }
            (11, ')') => true,
            _ => false,
        }
    }

    fn transform(parse_set: &Vec<char>) -> (u32, u32) {
        let nums: Vec<u32> = parse_set[4..parse_set.len() - 1]
            .split(|c| *c == ',')
            .map(|chars| chars.iter().collect::<String>().parse::<u32>().unwrap())
            .collect();
        (nums[0], nums[1])
    }

    fn parse(&mut self) -> Vec<(u32, u32)> {
        let mut state: Vec<char> = Vec::new();
        let mut output: Vec<(u32, u32)> = Vec::new();
        let mut curr: Result<char, Eof> = self.next();
        while let Ok(char) = curr {
            if Parser::validate_char(&state, char) {
                state.push(char);
                if char == ')' {
                    output.push(Parser::transform(&state));
                    state.clear();
                }
                curr = self.next();
            } else {
                if state.len() == 0 {
                    curr = self.next();
                }
                state.clear();
            }
        }
        output
    }
}

fn parse(reader: impl BufRead) -> Result<Vec<(u32, u32)>, InputParseError> {
    let data: Vec<(u32,u32)> = reader
        .lines()
        .flat_map(|line_res| {
            line_res.map(|line| Parser::new(&line).parse())
        })
        .collect::<Result<Vec<(u32,u32)>,Error>>()
        .map_err(|_e| InputParseError {message: "Failed to read line"})?
    data
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn parser_test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)\
            mul(8,5))mul(888,3) mul(88,675)mul(123,133)mul(4,123)mul(888,12)mul(4,12)\
            mul(22,12)mumul(1,1)mul(23mul(23,456)";

        let expected = vec![
            (2, 4),
            (5, 5),
            (11, 8),
            (8, 5),
            (888, 3),
            (88, 675),
            (123, 133),
            (4, 123),
            (888, 12),
            (4, 12),
            (22, 12),
            (1, 1),
            (23, 456),
        ];
        let result = parse(BufReader::new(&mut input.as_bytes())).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn part1() {
        let input =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

        let expected = 161;
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

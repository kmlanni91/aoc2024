use crate::runner::{InputParseError, Run};
use std::io::BufRead;

#[derive(Debug)]
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

/*
Grammar
expr -> MUL LPAREN 3DIGITLIT COMMA 3DIGITLIT RPAREN
*/
#[derive(Debug, Clone)]
enum GrammarItem {
    Mul,
    Paren,
    Comma,
    Num(u32),
}

fn parse(reader: impl BufRead) -> Result<Vec<(u32, u32)>, InputParseError> {
    let buffer = reader.bytes(); 
    let mut parsed: Vec<(u32, u32)> = Vec::new();

    Ok(parsed)
}

fn parse_line(line: String) -> Result<(), InputParseError> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn parser_test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let expected = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let result = parse(BufReader::new(&mut input.as_bytes())).unwrap();
        assert_eq!(result, expected)
    }

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

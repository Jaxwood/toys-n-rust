#![allow(dead_code)]

use std::fs;

use nom::{IResult, multi::separated_list1, character::complete::{newline, not_line_ending}};

fn parse_snafu(input: &str) -> IResult<&str, Vec<i8>> {
    let (input, lines) = not_line_ending(input)?;
    let mut result = Vec::new();
    for c in lines.chars() {
        match c {
            '2' => result.push(i8::from(2)),
            '1' => result.push(i8::from(1)),
            '0' => result.push(i8::from(0)),
            '-' => result.push(i8::from(-1)),
            '=' => result.push(i8::from(-2)),
            _ => continue,
        }
    }

    Ok((input, result))
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i8>>> {
    let (input, lines) = separated_list1(newline, parse_snafu)(input)?;

    Ok((input, lines))
}

pub fn day25a(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    let (_, lines) = parse(&input).unwrap();

    for line in lines {
        println!("{:?}", line);
    }

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn decode_snafu() {
        let actual = day25a("./data/day25.txt");
        assert_eq!(actual, 4890);
    }
}

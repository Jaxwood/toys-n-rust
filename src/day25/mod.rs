#![allow(dead_code)]

use std::fs;

use nom::{IResult, multi::separated_list1, character::complete::{newline, not_line_ending}};

fn parse_snafu(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, lines) = not_line_ending(input)?;
    let mut result = Vec::new();
    for c in lines.chars() {
        match c {
            '2' => result.push(i64::from(2)),
            '1' => result.push(i64::from(1)),
            '0' => result.push(i64::from(0)),
            '-' => result.push(i64::from(-1)),
            '=' => result.push(i64::from(-2)),
            _ => panic!("Unknown char: {}", c)
        }
    }

    Ok((input, result))
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let (input, lines) = separated_list1(newline, parse_snafu)(input)?;

    Ok((input, lines))
}

fn snafu_to_number(nums: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for (idx, num) in nums.iter().rev().enumerate() {
        let n = i64::pow(5, idx as u32);
        sum += (num * n) as i64;
    }
    sum
}

fn number_to_snafu(num: i64) -> String {
    let mut remainder = num;
    let mut out = String::default();
    while remainder > 0 {
        let glyph = match remainder % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => panic!("Unknown remainder: {}", remainder),
        };
        out.push(glyph);
        remainder += 2;
        remainder /= 5;
    }
    out.chars().rev().collect()
}

pub fn day25a(path: &str) -> String {
    let input = fs::read_to_string(path).unwrap();
    let (_, lines) = parse(&input).unwrap();

    let mut total = 0;
    for nums in lines.iter() {
        total += snafu_to_number(nums);
    }

    number_to_snafu(total)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn decode_snafu() {
        let actual = day25a("./data/day25.txt");
        assert_eq!(actual, "2=-1=0");
    }

    #[test]
    fn decode_snafu_part_a() {
        let actual = day25a("./data/day25final.txt");
        assert_eq!(actual, "2-1-110-=01-1-0-0==2");
    }
}


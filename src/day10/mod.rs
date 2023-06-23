#![allow(dead_code)]
use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, not_line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Instruction {
    Noop(),
    Addx(i32),
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, c)) = separated_pair(tag("addx"), space1, not_line_ending)(input)?;
    match c.parse() {
        Ok(num) => Ok((input, Instruction::Addx(num))),
        _ => panic!("could not parse!"),
    }
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Instruction::Noop()))
}

fn parse(input: &str) -> Vec<Instruction> {
    let signals = separated_list1(newline, alt((parse_noop, parse_addx)))(input);
    match signals {
        Ok((_, instructions)) => instructions,
        _ => panic!("parsing"),
    }
}

fn day10a(path: &str) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    let instructions = parse(content.as_str());
    let cycles = vec![20, 60, 100, 140, 180, 220];
    let initial = &mut vec![(1, 1)];
    let signals = instructions.iter().fold(initial, |acc, next| match next {
        Instruction::Noop() => {
            let (idx, x) = acc.last().expect("no last element");
            acc.push((*idx + 1, *x));
            acc
        }
        Instruction::Addx(val) => {
            let (idx, x) = acc.last().expect("no last element");
            acc.push((*idx + 2, *x + val));
            acc
        }
    });
    cycles
        .iter()
        .map(
            |cycle| match signals.iter().take_while(|(idx, _)| idx <= cycle).last() {
                Some(&(_, v)) => v * cycle,
                _ => panic!("not found"),
            },
        )
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_signal_strength() {
        let actual = day10a("./data/day10.txt");
        assert_eq!(actual, 13140);
    }

    #[test]
    fn find_signal_strength_part_a() {
        let actual = day10a("./data/day10final.txt");
        assert_eq!(actual, 12460);
    }
}

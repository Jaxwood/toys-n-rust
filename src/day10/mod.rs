#![allow(dead_code)]
use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space1, self},
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
    let (input, (_, c)) = separated_pair(tag("addx"), space1, complete::i32)(input)?;
    Ok((input, Instruction::Addx(c)))
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

fn is_in_range(sprite: i32, val: &i32) -> bool {
    vec![sprite - 1, sprite, sprite + 1]
        .iter()
        .any(|&v| v == *val)
}

fn day10b(path: &str) -> i32 {
    let screen = 40 * 6;
    let content = fs::read_to_string(path).expect("file not found");
    let instructions = parse(content.as_str());
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

    let sprite_positions: HashMap<_, _> = (0..screen)
        .into_iter()
        .enumerate()
        .map(|(idx, _)| {
            let result = signals.iter().take_while(|&&(i, _)| idx >= i - 1).last();
            let register = match result {
                Some(&(_, val)) => val,
                None => 1 as i32,
            };
            return (idx as i32, register);
        })
        .into_iter()
        .collect();

    let pixels: String = (0..screen)
        .into_iter()
        .enumerate()
        .map(|(idx, _)| match sprite_positions.get(&(idx as i32)) {
            Some(val) => {
                if is_in_range(idx as i32 % 40, val) {
                    return '#';
                } else {
                    return '.';
                }
            }
            None => panic!("no value found for idx"),
        })
        .collect();

    let result: Vec<String> = pixels
        .as_str()
        .chars()
        .collect::<Vec<char>>()
        .chunks(40)
        .map(|chunk| chunk.into_iter().collect())
        .collect();

    dbg!(result);
    0
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
    fn draw_signal() {
        let actual = day10b("./data/day10final.txt");
        assert_eq!(actual, 0);
    }

    #[test]
    fn find_signal_strength_part_a() {
        let actual = day10a("./data/day10final.txt");
        assert_eq!(actual, 12460);
    }
}

#![allow(dead_code)]

use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
enum Riddle {
    Num(i64),
    Add(String, String),
    Multiply(String, String),
    Divide(String, String),
    Subtract(String, String),
}

fn parse_calculation(input: &str) -> IResult<&str, (String, Riddle)> {
    let (input, job) = complete::alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, left) = complete::alpha1(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, op) = alt((
        complete::char('+'),
        complete::char('-'),
        complete::char('/'),
        complete::char('*'),
    ))(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, right) = complete::alpha1(input)?;
    let riddle = match op {
        '+' => Riddle::Add(left.to_string(), right.to_string()),
        '*' => Riddle::Multiply(left.to_string(), right.to_string()),
        '/' => Riddle::Divide(left.to_string(), right.to_string()),
        '-' => Riddle::Subtract(left.to_string(), right.to_string()),
        _ => panic!("Unknown op: {}", op),
    };
    Ok((input, (job.to_string(), riddle)))
}

fn parse_num(input: &str) -> IResult<&str, (String, Riddle)> {
    let (input, job) = complete::alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, num) = complete::i64(input)?;
    Ok((input, (job.to_string(), Riddle::Num(num))))
}

fn parse_riddle(input: &str) -> IResult<&str, (String, Riddle)> {
    let (input, riddle) = alt((parse_num, parse_calculation))(input)?;
    Ok((input, riddle))
}

fn parse(input: &str) -> IResult<&str, HashMap<String, Riddle>> {
    let (input, riddles) = separated_list1(newline, parse_riddle)(input)?;
    let riddles = riddles.into_iter().collect::<HashMap<_, _>>();
    Ok((input, riddles))
}

fn traverse(riddles: &HashMap<String, Riddle>, job: &str) -> i64 {
    match &riddles[job] {
        Riddle::Num(num) => *num,
        Riddle::Add(left, right) => traverse(riddles, &left) + traverse(riddles, &right),
        Riddle::Multiply(left, right) => traverse(riddles, &left) * traverse(riddles, &right),
        Riddle::Divide(left, right) => traverse(riddles, &left) / traverse(riddles, &right),
        Riddle::Subtract(left, right) => traverse(riddles, &left) - traverse(riddles, &right),
    }
}

fn day21a(path: &str) -> i64 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, riddles) = parse(&content).unwrap();
    let result = traverse(&riddles, "root");
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn solve_riddle() {
        let actual = day21a("./data/day21.txt");
        assert_eq!(actual, 152);
    }

    #[test]
    fn solve_riddle_part_a() {
        let actual = day21a("./data/day21final.txt");
        assert_eq!(actual, 85616733059734);
    }
}

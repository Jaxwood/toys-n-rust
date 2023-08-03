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
    Equal(String, String),
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
        Riddle::Equal(left, right) => {
            let left = traverse(riddles, &left);
            let right = traverse(riddles, &right);
            if left > right {
                -1
            } else if left < right {
                1
            } else {
                0
            }
        }
    }
}

fn binary_search(
    riddles: &mut HashMap<String, Riddle>,
    job: &str,
    mut lower: i64,
    mut upper: i64,
) -> i64 {
    loop {
        let mid = (lower + upper) / 2;
        riddles.insert("humn".to_string(), Riddle::Num(mid));
        let result = traverse(riddles, job);
        if result == 0 {
            return mid - 1;
        }
        if result == -1 {
            lower = mid;
        } else {
            upper = mid;
        }
    }
}

fn day21a(path: &str) -> i64 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, riddles) = parse(&content).unwrap();
    traverse(&riddles, "root")
}

fn day21b(path: &str, lower: i64, upper: i64) -> i64 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, mut riddles) = parse(&content).unwrap();
    let (left, right) = match &riddles["root"] {
        Riddle::Add(left, right) => (left, right),
        Riddle::Multiply(left, right) => (left, right),
        Riddle::Divide(left, right) => (left, right),
        Riddle::Subtract(left, right) => (left, right),
        _ => todo!(),
    };
    riddles.insert(
        "root".to_string(),
        Riddle::Equal(left.to_string(), right.to_string()),
    );

    binary_search(&mut riddles, "root", lower, upper)
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
    fn solve_riddle_with_input() {
        let actual = day21b("./data/day21.txt", 1000, 1);
        assert_eq!(actual, 301);
    }

    #[test]
    fn solve_riddle_part_a() {
        let actual = day21a("./data/day21final.txt");
        assert_eq!(actual, 85616733059734);
    }

    #[test]
    fn solve_riddle_part_b() {
        let actual = day21b(
            "./data/day21final.txt",
            1_000_000_000_000,
            30_000_000_000_000,
        );
        assert_eq!(actual, 3560324848168);
    }
}

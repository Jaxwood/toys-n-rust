#![allow(dead_code)]

use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, self, space1, alpha1, alphanumeric1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, branch::alt,
};

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    MultiplySelf(),
}

#[derive(Debug)]
struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: Operation,
    check: i32,
    true_case: i32,
    false_case: i32,
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    // parse id
    let (input, id) = preceded(tag("Monkey "), complete::i32)(input)?;
    let (input, _) = preceded(tag(":"), newline)(input)?;
    // parse items
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), complete::i32)(input)?;
    let (input, _) = newline(input)?;
    // parse operation
    let (input, _) = tag("  Operation: new = old ")(input)?;
    let (input, (op, operand)) = separated_pair(alt((tag("+"), tag("*"))),space1, alphanumeric1)(input)?;
    let operation = match operand.parse::<usize>() {
        Ok(num) => match op {
            "*" => Operation::Multiply(num),
            _ => Operation::Add(num),
        }
        Err(_) => Operation::MultiplySelf(),
    };
    let (input, _) = newline(input)?;
    // parse condition
    let (input, check) = preceded(tag("  Test: divisible by "), complete::i32)(input)?;
    let (input, _) = newline(input)?;
    // parse true case
    let (input, true_case) = preceded(tag("    If true: throw to monkey "), complete::i32)(input)?;
    let (input, _) = newline(input)?;
    // parse false case
    let (input, false_case) = preceded(tag("    If false: throw to monkey "), complete::i32)(input)?;
    let (input, _) = newline(input)?;

    Ok((
        input,
        Monkey {
            id,
            items,
            operation,
            check,
            true_case,
            false_case,
        },
    ))
}

fn parse(input: &str) -> Vec<Monkey> {
    let monkeys = separated_list1(newline, parse_monkey)(input);
    match monkeys {
        Ok((_, monkeys)) => monkeys,
        _ => panic!("could not parse monkey"),
    }
}

fn day11a(path: &str) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    let monkeys = parse(content.as_str());
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_most_active_monkeys() {
        let actual = day11a("./data/day11.txt");
        assert_eq!(actual, 10605);
    }

    #[test]
    fn find_most_active_monkeys_part_a() {
        let actual = day11a("./data/day11final.txt");
        assert_eq!(actual, 10605);
    }
}

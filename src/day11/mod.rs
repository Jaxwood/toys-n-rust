#![allow(dead_code)]

use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
enum Operation {
    Add(i32),
    Multiply(i32),
    MultiplySelf(),
}

#[derive(Debug)]
struct Monkey {
    id: i32,
    count: i32,
    items: Vec<i32>,
    operation: Operation,
    check: i32,
    true_case: i32,
    false_case: i32,
}

impl Monkey {
    fn inspect(&mut self, map: &mut HashMap<i32, Vec<i32>>) {
        if let Some(values) = map.get(&self.id) {
            for val in values.iter() {
                self.items.push(*val);
            }
        }

        self.count += self.items.len() as i32;

        for item in self.items.iter() {
            match self.operation {
                Operation::Add(num) => {
                    let val = (item + num) / 3;
                    if val % self.check == 0 {
                        map.entry(self.true_case)
                            .and_modify(|v| v.push(val))
                            .or_insert(vec![val]);
                    } else {
                        map.entry(self.false_case)
                            .and_modify(|v| v.push(val))
                            .or_insert(vec![val]);
                    }
                }
                Operation::Multiply(num) => {
                    let val = (item * num) / 3;
                    if val % self.check == 0 {
                        map.entry(self.true_case)
                            .and_modify(|v| v.push(val))
                            .or_insert(vec![val]);
                    } else {
                        map.entry(self.false_case)
                            .and_modify(|v| v.push(val))
                            .or_insert(vec![val]);
                    }
                }
                Operation::MultiplySelf() => {
                    let val = (item * item) / 3;
                    if val % self.check == 0 {
                        map.entry(self.true_case)
                            .and_modify(|v| v.push(val))
                            .or_insert(vec![val]);
                    } else {
                        map.entry(self.false_case)
                            .and_modify(|v| v.push(val))
                            .or_insert(vec![val]);
                    }
                }
            }
        }

        self.items = vec![];
        map.remove(&self.id);
    }
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
    let (input, (op, operand)) =
        separated_pair(alt((tag("+"), tag("*"))), space1, alphanumeric1)(input)?;
    let operation = match operand.parse() {
        Ok(num) => match op {
            "*" => Operation::Multiply(num),
            _ => Operation::Add(num),
        },
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
    let (input, false_case) =
        preceded(tag("    If false: throw to monkey "), complete::i32)(input)?;
    let (input, _) = newline(input)?;

    Ok((
        input,
        Monkey {
            id,
            count: 0,
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
    let mut monkeys = parse(content.as_str());
    let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
    for _ in 0..20 {
        for monkey in monkeys.iter_mut() {
            monkey.inspect(&mut result);
        }
    }

    let mut inspections: Vec<_> = monkeys.iter().map(|monkey| monkey.count).collect();
    inspections.sort();
    inspections.reverse();
    inspections
        .iter_mut()
        .take(2)
        .fold(1, |mut acc, inspection| {
            acc *= *inspection;
            acc
        })
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
        assert_eq!(actual, 182293);
    }
}

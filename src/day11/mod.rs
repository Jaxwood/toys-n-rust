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
    Add(u64),
    Multiply(u64),
    MultiplySelf(),
}

#[derive(Debug)]
struct Monkey {
    id: u64,
    count: u64,
    items: Vec<u64>,
    operation: Operation,
    check: u64,
    true_case: u64,
    false_case: u64,
}

impl Monkey {
    fn lower_worry_level(&self, worry_level: u64, chinese_remainder: Option<u64>) -> u64 {
        if let Some(remainder) = chinese_remainder {
            worry_level % remainder
        } else {
            worry_level / 3
        }
    }

    fn inspect(&mut self, map: &mut HashMap<u64, Vec<u64>>, chinese_remainder: Option<u64>) {
        if let Some(values) = map.get(&self.id) {
            for worry_level in values.iter() {
                self.items.push(*worry_level);
            }
        }

        self.count += self.items.len() as u64;

        for item in self.items.iter() {
            match self.operation {
                Operation::Add(num) => {
                    let worry_level = self.lower_worry_level(item + num, chinese_remainder);
                    if worry_level % self.check == 0 {
                        map.entry(self.true_case)
                            .and_modify(|v| v.push(worry_level))
                            .or_insert(vec![worry_level]);
                    } else {
                        map.entry(self.false_case)
                            .and_modify(|v| v.push(worry_level))
                            .or_insert(vec![worry_level]);
                    }
                }
                Operation::Multiply(num) => {
                    let worry_level = self.lower_worry_level(item * num, chinese_remainder);
                    if worry_level % self.check == 0 {
                        map.entry(self.true_case)
                            .and_modify(|v| v.push(worry_level))
                            .or_insert(vec![worry_level]);
                    } else {
                        map.entry(self.false_case)
                            .and_modify(|v| v.push(worry_level))
                            .or_insert(vec![worry_level]);
                    }
                }
                Operation::MultiplySelf() => {
                    let worry_level = self.lower_worry_level(item * item, chinese_remainder);
                    if worry_level % self.check == 0 {
                        map.entry(self.true_case)
                            .and_modify(|v| v.push(worry_level))
                            .or_insert(vec![worry_level]);
                    } else {
                        map.entry(self.false_case)
                            .and_modify(|v| v.push(worry_level))
                            .or_insert(vec![worry_level]);
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
    let (input, id) = preceded(tag("Monkey "), complete::u64)(input)?;
    let (input, _) = preceded(tag(":"), newline)(input)?;
    // parse items
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), complete::u64)(input)?;
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
    let (input, check) = preceded(tag("  Test: divisible by "), complete::u64)(input)?;
    let (input, _) = newline(input)?;
    // parse true case
    let (input, true_case) = preceded(tag("    If true: throw to monkey "), complete::u64)(input)?;
    let (input, _) = newline(input)?;
    // parse false case
    let (input, false_case) =
        preceded(tag("    If false: throw to monkey "), complete::u64)(input)?;
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

fn day11a(path: &str) -> u64 {
    let content = fs::read_to_string(path).expect("file not found");
    let mut monkeys = parse(content.as_str());
    let mut result: HashMap<u64, Vec<u64>> = HashMap::new();
    for _ in 0..20 {
        for monkey in monkeys.iter_mut() {
            monkey.inspect(&mut result, None);
        }
    }

    let mut inspections: Vec<_> = monkeys.iter().map(|monkey| monkey.count).collect();
    inspections.sort();
    inspections.reverse();
    inspections.iter().take(2).product()
}

fn day11b(path: &str) -> u64 {
    let content = fs::read_to_string(path).expect("file not found");
    let mut monkeys = parse(content.as_str());
    let mut result: HashMap<u64, Vec<u64>> = HashMap::new();
    // keep worry level down by using the [chinese remainder
    // theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
    let chinese_reminder = monkeys.iter().map(|monkey| monkey.check).product::<u64>();
    for _ in 0..10_000 {
        for monkey in monkeys.iter_mut() {
            monkey.inspect(&mut result, Some(chinese_reminder));
        }
    }

    let mut inspections: Vec<_> = monkeys.iter().map(|monkey| monkey.count).collect();
    inspections.sort();
    inspections.reverse();
    inspections.iter().take(2).product()
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
    fn find_most_active_monkeys_with_more_rounds() {
        let actual = day11b("./data/day11.txt");
        assert_eq!(actual, 2713310158);
    }

    #[test]
    fn find_most_active_monkeys_part_a() {
        let actual = day11a("./data/day11final.txt");
        assert_eq!(actual, 182293);
    }

    #[test]
    fn find_most_active_monkeys_part_b() {
        let actual = day11b("./data/day11final.txt");
        assert_eq!(actual, 54832778815);
    }
}

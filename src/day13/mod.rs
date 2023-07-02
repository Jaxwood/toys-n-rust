#![allow(dead_code)]

use std::{collections::VecDeque, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug)]
enum Packet {
    List(VecDeque<Packet>),
    Number(u32),
}

enum Order {
    Sorted,
    UnSorted,
    InProgress,
}

impl Order {
    fn in_progress(&self) -> bool {
        match self {
            Order::Sorted => false,
            Order::UnSorted => false,
            Order::InProgress => true,
        }
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]"))
            .map(|vec| Packet::List(VecDeque::from(vec))),
        nom::character::complete::u32.map(|num| Packet::Number(num)),
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(|(p1, p2)| Pair {
            left: p1,
            right: p2,
        }),
    )(input)
}

fn compare(mut left: VecDeque<Packet>, mut right: VecDeque<Packet>) -> Order {
    let mut result = Order::InProgress;
    while (left.len() > 0 || right.len() > 0) && result.in_progress() {
        let l = left.pop_front();
        let r = right.pop_front();
        match (l, r) {
            (None, None) => {
                return Order::Sorted;
            }
            (None, Some(_)) => {
                return Order::Sorted;
            }
            (Some(_), None) => {
                return Order::UnSorted;
            }
            (Some(l), Some(r)) => match (l, r) {
                (Packet::Number(n1), Packet::Number(n2)) => {
                    if n1 > n2 {
                        return Order::UnSorted;
                    }
                    if n1 < n2 {
                        return Order::Sorted;
                    }
                }
                (Packet::List(ls), Packet::Number(n)) => {
                    result = compare(ls, VecDeque::from([Packet::Number(n)]));
                }
                (Packet::Number(n), Packet::List(rs)) => {
                    result = compare(VecDeque::from([Packet::Number(n)]), rs);
                }
                (Packet::List(ls), Packet::List(rs)) => {
                    result = compare(ls, rs);
                }
            },
        }
    }
    result
}

fn day13a(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, packets) = parse(content.as_str()).expect("parsing failed");
    packets
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, next)| match (next.left, next.right) {
            (Packet::List(_), Packet::Number(_)) => acc,
            (Packet::Number(_), Packet::List(_)) => acc,
            (Packet::Number(_), Packet::Number(_)) => acc,
            (Packet::List(ls), Packet::List(rs)) => match compare(ls, rs) {
                Order::Sorted => acc + idx + 1,
                Order::UnSorted => acc,
                Order::InProgress => acc,
            },
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_packets_in_right_order() {
        let actual = day13a("./data/day13.txt");
        assert_eq!(actual, 13);
    }

    #[test]
    fn find_packets_in_right_order_part_a() {
        let actual = day13a("./data/day13final.txt");
        assert_eq!(actual, 5208);
    }
}

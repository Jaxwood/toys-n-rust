#![allow(dead_code)]

use std::{cmp::Ordering, collections::VecDeque, fs};

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

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(VecDeque<Packet>),
    Number(u32),
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

fn compare(mut left: VecDeque<Packet>, mut right: VecDeque<Packet>) -> Ordering {
    let mut result = Ordering::Equal;
    while (left.len() > 0 || right.len() > 0) && result.is_eq() {
        let l = left.pop_front();
        let r = right.pop_front();
        match (l, r) {
            (None, None) => {
                return Ordering::Equal;
            }
            (None, Some(_)) => {
                return Ordering::Less;
            }
            (Some(_), None) => {
                return Ordering::Greater;
            }
            (Some(l), Some(r)) => match (l, r) {
                (Packet::Number(n1), Packet::Number(n2)) => {
                    if n1 > n2 {
                        return Ordering::Greater;
                    }
                    if n1 < n2 {
                        return Ordering::Less;
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
                Ordering::Less => acc + idx + 1,
                Ordering::Greater => acc,
                Ordering::Equal => acc,
            },
        })
}

fn day13b(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, mut packets) = parse(content.as_str()).expect("parsing failed");

    let extra = Pair {
        left: Packet::List(VecDeque::from([Packet::List(VecDeque::from([
            Packet::Number(2),
        ]))])),
        right: Packet::List(VecDeque::from([Packet::List(VecDeque::from([
            Packet::Number(6),
        ]))])),
    };
    packets.push(extra);

    let mut all = packets
        .iter()
        .flat_map(|packet| {
            let left = packet.left.clone();
            let right = packet.right.clone();
            return vec![left, right];
        })
        .map(|packet| match packet {
            Packet::List(ls) => Some(ls),
            Packet::Number(_) => None,
        })
        .flatten()
        .collect::<Vec<_>>();

    all.sort_by(|a, b| {
        let lls = a.clone();
        let rrs = b.clone();
        return compare(lls, rrs);
    });

    all.into_iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            let two = match compare(
                item.clone(),
                VecDeque::from([Packet::List(VecDeque::from([Packet::Number(2)]))]),
            ) {
                Ordering::Less => None,
                Ordering::Equal => Some(idx + 1),
                Ordering::Greater => None,
            };
            let six = match compare(
                item.clone(),
                VecDeque::from([Packet::List(VecDeque::from([Packet::Number(6)]))]),
            ) {
                Ordering::Less => None,
                Ordering::Equal => Some(idx + 1),
                Ordering::Greater => None,
            };
            two.or(six)
        })
        .product()
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

    #[test]
    fn sort_packets() {
        let actual = day13b("./data/day13.txt");
        assert_eq!(actual, 140);
    }

    #[test]
    fn sort_packets_part_b() {
        let actual = day13b("./data/day13final.txt");
        assert_eq!(actual, 25792);
    }
}

#![allow(dead_code)]

use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Valve {
    name: String,
    rate: i32,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(name: &str, rate: i32, tunnels: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            rate,
            tunnels,
        }
    }
}

fn parse_tunnels(input: &str) -> IResult<&str, Vec<String>> {
    separated_list0(tag(", "), alpha1)(input)
        .map(|(input, tunnels)| (input, tunnels.iter().map(|t| t.to_string()).collect()))
}

fn parse_tunnel(input: &str) -> IResult<&str, Vec<String>> {
    alpha1(input).map(|(input, tunnel)| (input, vec![tunnel.to_string()]))
}

fn parse_valves(input: &str) -> IResult<&str, Valve> {
    tuple((
        tag("Valve "),
        alpha1,
        tag(" has flow rate="),
        nom::character::complete::i32,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        alt((parse_tunnels, parse_tunnel)),
    ))(input)
    .map(|(input, (_, valve, _, rate, _, tunnels))| (input, Valve::new(valve, rate, tunnels)))
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn parse(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(newline, parse_valves)(input).map(|(input, readings)| (input, readings))
}

fn day16a(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, _valves) = parse(content.as_str()).expect("parsing failed");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_most_pressure() {
        let actual = day16a("./data/day16.txt");
        assert_eq!(actual, 1651);
    }
}
